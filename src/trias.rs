use anyhow::Context;
use anyhow::Result;
use chrono::prelude::*;
use quick_xml::events::{BytesText, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;

use crate::{Departure, Stop};

const STOP_EVENT_REQUEST_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<Trias version="1.1" xmlns="http://www.vdv.de/trias" xmlns:siri="http://www.siri.org.uk/siri" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <ServiceRequest>
        <siri:RequestTimestamp>REQUEST_TIMESTAMP</siri:RequestTimestamp>
        <siri:RequestorRef>REQUESTOR_REF</siri:RequestorRef>
        <RequestPayload>
            <StopEventRequest>
                <Location>
                    <LocationRef>
                        <StopPointRef>STOP_POINT_REF</StopPointRef>
                    </LocationRef>
                </Location>
                <Params>
                    <NumberOfResults>10</NumberOfResults>
                    <StopEventType>departure</StopEventType>
                    <IncludePreviousCalls>false</IncludePreviousCalls>
                    <IncludeOnwardCalls>false</IncludeOnwardCalls>
                    <IncludeRealtimeData>true</IncludeRealtimeData>
                </Params>
            </StopEventRequest>
        </RequestPayload>
    </ServiceRequest>
</Trias>"#;

/// Format a TRIAS StopEventRequest.
pub fn format_stop_event_request(
    req_timestamp: DateTime<Utc>,
    req_ref: &str,
    stop_point_ref: &str,
) -> String {
    let mut reader = Reader::from_str(STOP_EVENT_REQUEST_XML);
    let mut writer = Writer::new(std::io::Cursor::new(Vec::new()));
    let req_timestamp_rfc3339 = req_timestamp.to_rfc3339();
    loop {
        match reader.read_event() {
            Ok(Event::Text(e)) => writer
                .write_event(Event::Text(match e.as_ref() {
                    b"REQUEST_TIMESTAMP" => BytesText::new(&req_timestamp_rfc3339),
                    b"REQUESTOR_REF" => BytesText::new(req_ref),
                    b"STOP_POINT_REF" => BytesText::new(stop_point_ref),
                    _ => e,
                }))
                .unwrap(),

            Ok(Event::Eof) => break,
            Ok(e) => writer.write_event(e).unwrap(),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        }
    }
    String::from_utf8(writer.into_inner().into_inner()).unwrap()
}

/// Parse a TRIAS StopEventResponse XML.
pub fn parse_stop_event_response(xml: &str) -> Result<Vec<Departure>> {
    let mut reader = Reader::from_str(xml);
    let mut departures = Vec::new();
    let mut current_departure: Option<Departure> = None;
    let mut in_text = false;
    let mut current_text: Option<String> = None;
    let mut buf = Vec::new();

    macro_rules! set_to_text {
        ($prop:ident) => {
            if let Some(ref mut dep) = current_departure {
                if let Some(text) = current_text.take() {
                    dep.$prop = text.into();
                }
            }
        };
    }

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Eof => break,
            Event::Start(e) => match e.name().as_ref() {
                b"StopEventResult" => {
                    current_departure = Some(Departure::default());
                }
                b"Text" | b"TimetabledTime" | b"EstimatedTime" | b"PtMode" => {
                    current_text = Some(String::new());
                    in_text = true;
                }

                _ => {}
            },
            Event::Text(e) => {
                if in_text {
                    if let Some(ref mut t) = current_text {
                        t.push_str(&String::from_utf8_lossy(e.as_ref()));
                    }
                }
            }
            Event::End(e) => match e.name().as_ref() {
                b"StopEventResult" => {
                    // S-Bahn S1 => S1
                    let mut d = current_departure.take().unwrap();
                    let line = d.line.splitn(2, ' ');
                    d.line = line.last().unwrap().to_string();
                    departures.push(d);
                }
                b"Text" => in_text = false,
                b"PlannedBay" => set_to_text!(bay),
                b"TimetabledTime" => set_to_text!(timetable_time),
                b"EstimatedTime" => set_to_text!(estimated_time),
                b"PtMode" => set_to_text!(mode),
                b"PublishedLineName" => set_to_text!(line),
                b"DestinationText" => set_to_text!(destination),
                _ => {}
            },
            _ => {}
        }
    }

    Ok(departures)
}

const LOCATION_INFORMATION_REQUEST_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<Trias version="1.1" xmlns="http://www.vdv.de/trias" xmlns:siri="http://www.siri.org.uk/siri" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <ServiceRequest>
        <siri:RequestTimestamp>REQUEST_TIMESTAMP</siri:RequestTimestamp>
        <siri:RequestorRef>REQUESTOR_REF</siri:RequestorRef>
        <RequestPayload>
            <LocationInformationRequest>
                <InitialInput>
                    <GeoRestriction>
                        <Circle>
                            <Center>
                                <Latitude>LATITUDE</Latitude>
                                <Longitude>LONGITUDE</Longitude>
                            </Center>
                            <Radius>1000</Radius>
                        </Circle>
                    </GeoRestriction>
                </InitialInput>
                <Restrictions>
                    <Type>stop</Type>
                    <NumberOfResults>10</NumberOfResults>
                    <IncludePtModes>true</IncludePtModes>
                </Restrictions>
            </LocationInformationRequest>
        </RequestPayload>
    </ServiceRequest>
</Trias>"#;

/// Format a TRIAS LocationInformationRequest.
pub fn format_location_information_request(
    req_timestamp: DateTime<Utc>,
    req_ref: &str,
    latitude: f32,
    longitude: f32,
) -> String {
    let mut reader = Reader::from_str(LOCATION_INFORMATION_REQUEST_XML);
    let mut writer = Writer::new(std::io::Cursor::new(Vec::new()));
    let req_timestamp_rfc3339 = req_timestamp.to_rfc3339();
    let latitude_str = latitude.to_string();
    let longitude_str = longitude.to_string();
    loop {
        match reader.read_event() {
            Ok(Event::Text(e)) => writer
                .write_event(Event::Text(match e.as_ref() {
                    b"REQUEST_TIMESTAMP" => BytesText::new(&req_timestamp_rfc3339),
                    b"REQUESTOR_REF" => BytesText::new(req_ref),
                    b"LATITUDE" => BytesText::new(&latitude_str),
                    b"LONGITUDE" => BytesText::new(&longitude_str),
                    _ => e,
                }))
                .unwrap(),

            Ok(Event::Eof) => break,
            Ok(e) => writer.write_event(e).unwrap(),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        }
    }
    String::from_utf8(writer.into_inner().into_inner()).unwrap()
}

/// Parse a TRIAS LocationInformationResponse XML.
pub fn parse_location_information_response(xml: &str) -> Result<Vec<Stop>> {
    let mut reader = Reader::from_str(xml);
    let mut locations = Vec::new();
    let mut current_location: Option<Stop> = None;
    let mut in_location = 0;
    let mut in_text = false;
    let mut current_text: Option<String> = None;
    let mut buf = Vec::new();

    macro_rules! set_to_text {
        ($prop:ident) => {
            if let Some(ref mut dep) = current_location {
                if let Some(text) = current_text.take() {
                    dep.$prop = text;
                }
            }
        };
    }

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Eof => break,
            Event::Start(e) => match e.name().as_ref() {
                b"Location" => {
                    // <Location> is weirdly nested
                    in_location += 1;
                    if in_location == 1 {
                        current_location = Some(Stop::default());
                    }
                }
                b"Text" | b"StopPointRef" | b"Longitude" | b"Latitude" | b"PtMode" => {
                    current_text = Some(String::new());
                    in_text = true;
                }

                _ => {}
            },
            Event::Text(e) => {
                if in_text {
                    if let Some(ref mut t) = current_text {
                        t.push_str(&String::from_utf8_lossy(e.as_ref()));
                    }
                }
            }
            Event::End(e) => match e.name().as_ref() {
                b"Location" => {
                    if in_location == 1 {
                        locations.push(current_location.take().unwrap());
                    }
                    in_location -= 1;
                }
                b"Text" => in_text = false,
                b"StopPointRef" => set_to_text!(id),
                b"StopPointName" => set_to_text!(name),
                tag @ b"Longitude" | tag @ b"Latitude" => {
                    if let Some(ref mut loc) = current_location {
                        if let Some(text) = current_text.take() {
                            let coord = text.parse().with_context(|| {
                                format!("error parsing {}", &String::from_utf8_lossy(tag))
                            })?;
                            if tag == b"Longitude" {
                                loc.long = coord;
                            } else {
                                loc.lat = coord;
                            }
                        }
                    }
                }
                b"PtMode" => {
                    if let Some(ref mut loc) = current_location {
                        if let Some(text) = current_text.take() {
                            loc.modes.push(text);
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    Ok(locations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_stop_event_request() {
        let date = DateTime::<Utc>::from_timestamp(0, 0).unwrap();
        assert_eq!(
            format_stop_event_request(date, "abc1234", "de:08212:1001"),
            r#"<?xml version="1.0" encoding="UTF-8"?>
<Trias version="1.1" xmlns="http://www.vdv.de/trias" xmlns:siri="http://www.siri.org.uk/siri" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <ServiceRequest>
        <siri:RequestTimestamp>1970-01-01T00:00:00+00:00</siri:RequestTimestamp>
        <siri:RequestorRef>abc1234</siri:RequestorRef>
        <RequestPayload>
            <StopEventRequest>
                <Location>
                    <LocationRef>
                        <StopPointRef>de:08212:1001</StopPointRef>
                    </LocationRef>
                </Location>
                <Params>
                    <NumberOfResults>10</NumberOfResults>
                    <StopEventType>departure</StopEventType>
                    <IncludePreviousCalls>false</IncludePreviousCalls>
                    <IncludeOnwardCalls>false</IncludeOnwardCalls>
                    <IncludeRealtimeData>true</IncludeRealtimeData>
                </Params>
            </StopEventRequest>
        </RequestPayload>
    </ServiceRequest>
</Trias>"#
        );
    }

    #[test]
    fn test_format_location_information_request() {
        let date = DateTime::<Utc>::from_timestamp(0, 0).unwrap();
        assert_eq!(
            format_location_information_request(date, "abc1234", 49.009, 8.417),
            r#"<?xml version="1.0" encoding="UTF-8"?>
<Trias version="1.1" xmlns="http://www.vdv.de/trias" xmlns:siri="http://www.siri.org.uk/siri" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <ServiceRequest>
        <siri:RequestTimestamp>1970-01-01T00:00:00+00:00</siri:RequestTimestamp>
        <siri:RequestorRef>abc1234</siri:RequestorRef>
        <RequestPayload>
            <LocationInformationRequest>
                <InitialInput>
                    <GeoRestriction>
                        <Circle>
                            <Center>
                                <Latitude>49.009</Latitude>
                                <Longitude>8.417</Longitude>
                            </Center>
                            <Radius>1000</Radius>
                        </Circle>
                    </GeoRestriction>
                </InitialInput>
                <Restrictions>
                    <Type>stop</Type>
                    <NumberOfResults>10</NumberOfResults>
                    <IncludePtModes>true</IncludePtModes>
                </Restrictions>
            </LocationInformationRequest>
        </RequestPayload>
    </ServiceRequest>
</Trias>"#
        );
    }

    #[test]
    fn test_parse_location_information_response() {
        let xml = r#"
<?xml version="1.0" encoding="UTF-8"?>
<Trias xmlns="http://www.vdv.de/trias" version="1.1">
  <ServiceDelivery>
    <ResponseTimestamp xmlns="http://www.siri.org.uk/siri">2024-04-14T15:21:27Z</ResponseTimestamp>
    <ProducerRef xmlns="http://www.siri.org.uk/siri">EFAController10.3.18.46-EFA03</ProducerRef>
    <Status xmlns="http://www.siri.org.uk/siri">true</Status>
    <MoreData>false</MoreData>
    <Language>de</Language>
    <DeliveryPayload>
      <LocationInformationResponse>
        <Location>
          <Location>
            <StopPoint>
              <StopPointRef>de:08212:1001</StopPointRef>
              <StopPointName>
                <Text>Durlacher Tor/KIT-Campus Süd (U)</Text>
                <Language>de</Language>
              </StopPointName>
              <LocalityRef>8212000:15</LocalityRef>
            </StopPoint>
            <LocationName>
              <Text>Karlsruhe</Text>
              <Language>de</Language>
            </LocationName>
            <GeoPosition>
              <Longitude>8.41711</Longitude>
              <Latitude>49.00893</Latitude>
            </GeoPosition>
          </Location>
          <Complete>true</Complete>
          <Mode>
            <PtMode>rail</PtMode>
            <RailSubmode>suburbanRailway</RailSubmode>
          </Mode>
          <Mode>
            <PtMode>tram</PtMode>
            <TramSubmode>cityTram</TramSubmode>
          </Mode>
        </Location>
        <Location>
          <Location>
            <StopPoint>
              <StopPointRef>de:08212:3</StopPointRef>
              <StopPointName>
                <Text>Durlacher Tor/KIT-Campus Süd</Text>
                <Language>de</Language>
              </StopPointName>
              <LocalityRef>8212000:15</LocalityRef>
            </StopPoint>
            <LocationName>
              <Text>Karlsruhe</Text>
              <Language>de</Language>
            </LocationName>
            <GeoPosition>
              <Longitude>8.41783</Longitude>
              <Latitude>49.00909</Latitude>
            </GeoPosition>
          </Location>
          <Complete>true</Complete>
          <Mode>
            <PtMode>rail</PtMode>
            <RailSubmode>suburbanRailway</RailSubmode>
          </Mode>
          <Mode>
            <PtMode>tram</PtMode>
            <TramSubmode>cityTram</TramSubmode>
          </Mode>
          <Mode>
            <PtMode>bus</PtMode>
            <BusSubmode>unknown</BusSubmode>
          </Mode>
        </Location>
      </LocationInformationResponse>
    </DeliveryPayload>
  </ServiceDelivery>
</Trias>
"#;
        let locations = parse_location_information_response(xml).unwrap();
        assert!(locations.len() == 2);
        assert_eq!(
            &locations,
            &[
                Stop {
                    id: "de:08212:1001".to_string(),
                    name: "Durlacher Tor/KIT-Campus Süd (U)".to_string(),
                    lat: 49.00893,
                    long: 8.41711,
                    modes: ["rail".to_string(), "tram".to_string()].to_vec(),
                },
                Stop {
                    id: "de:08212:3".to_string(),
                    name: "Durlacher Tor/KIT-Campus Süd".to_string(),
                    lat: 49.00909,
                    long: 8.41783,
                    modes: ["rail".to_string(), "tram".to_string(), "bus".to_string()].to_vec()
                }
            ]
        );
    }

    #[test]
    fn test_parse_stop_event_response() {
        let xml = r#"
const STOP_EVENT_RESPONSE_XML: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<Trias xmlns="http://www.vdv.de/trias" version="1.1">
  <ServiceDelivery>
    <ResponseTimestamp xmlns="http://www.siri.org.uk/siri">2024-04-11T21:33:02Z</ResponseTimestamp>
    <ProducerRef xmlns="http://www.siri.org.uk/siri">EFAController10.3.18.46-EFA02</ProducerRef>
    <Status xmlns="http://www.siri.org.uk/siri">true</Status>
    <MoreData>false</MoreData>
    <Language>de</Language>
    <DeliveryPayload>
      <StopEventResponse>
        <StopEventResponseContext>
          <Situations>
          </Situations>
        </StopEventResponseContext>
        <StopEventResult>
          <ResultId>ID-4D35FD78-F33F-460D-B7F0-F657B7B4BF62</ResultId>
          <StopEvent>
            <ThisCall>
              <CallAtStop>
                <StopPointRef>de:08212:1001:91:1</StopPointRef>
                <StopPointName>
                  <Text>KA Durlacher Tor/KIT-Campus Süd (U)</Text>
                  <Language>de</Language>
                </StopPointName>
                <PlannedBay>
                  <Text>Gleis 1 (U)</Text>
                  <Language>de</Language>
                </PlannedBay>
                <ServiceDeparture>
                  <TimetabledTime>2024-04-11T21:30:00Z</TimetabledTime>
                  <EstimatedTime>2024-04-11T21:34:00Z</EstimatedTime>
                </ServiceDeparture>
                <StopSeqNumber>49</StopSeqNumber>
              </CallAtStop>
            </ThisCall>
            <Service>
              <OperatingDayRef>2024-04-11</OperatingDayRef>
              <JourneyRef>kvv:22304:E:H:j24:19932</JourneyRef>
              <LineRef>kvv:22304:E:H</LineRef>
              <DirectionRef>outward</DirectionRef>
              <Mode>
                <PtMode>rail</PtMode>
                <RailSubmode>suburbanRailway</RailSubmode>
                <Name>
                  <Text>S-Bahn</Text>
                  <Language>de</Language>
                </Name>
              </Mode>
              <PublishedLineName>
                <Text>S-Bahn S4</Text>
                <Language>de</Language>
              </PublishedLineName>
              <OperatorRef>kvv:01</OperatorRef>
              <RouteDescription>
                <Text>Öhringen - Heilbronn - Karlsruhe</Text>
                <Language>de</Language>
              </RouteDescription>
              <Attribute>
                <Text>
                  <Text>Stufenloses Fahrzeug, WLAN</Text>
                  <Language>de</Language>
                </Text>
                <Code>1102871</Code>
                <Mandatory>false</Mandatory>
              </Attribute>
              <OriginStopPointRef>de:08121:173:2:2</OriginStopPointRef>
              <OriginText>
                <Text>Heilbronn Pfühlpark</Text>
                <Language>de</Language>
              </OriginText>
              <DestinationText>
                <Text>Albtalbahnhof über Hbf</Text>
                <Language>de</Language>
              </DestinationText>
              <SituationFullRef>
                <ParticipantRef xmlns="http://www.siri.org.uk/siri">KVV</ParticipantRef>
                <SituationNumber xmlns="http://www.siri.org.uk/siri">110008062_KVV_ICSKVV</SituationNumber>
              </SituationFullRef>
            </Service>
          </StopEvent>
        </StopEventResult>
        <StopEventResult>
          <ResultId>ID-FD57D1CC-2B2C-46C3-A63D-524C813C8E77</ResultId>
          <StopEvent>
            <ThisCall>
              <CallAtStop>
                <StopPointRef>de:08212:3:3:3</StopPointRef>
                <StopPointName>
                  <Text>KA Durlacher Tor/KIT-Campus Süd</Text>
                  <Language>de</Language>
                </StopPointName>
                <PlannedBay>
                  <Text>Gleis 3</Text>
                  <Language>de</Language>
                </PlannedBay>
                <ServiceDeparture>
                  <TimetabledTime>2024-04-11T21:32:30Z</TimetabledTime>
                  <EstimatedTime>2024-04-11T21:32:30Z</EstimatedTime>
                </ServiceDeparture>
                <StopSeqNumber>13</StopSeqNumber>
              </CallAtStop>
            </ThisCall>
            <Service>
              <OperatingDayRef>2024-04-11</OperatingDayRef>
              <JourneyRef>kvv:21004:E:H:j24:604</JourneyRef>
              <LineRef>kvv:21004:E:H</LineRef>
              <DirectionRef>outward</DirectionRef>
              <Mode>
                <PtMode>tram</PtMode>
                <TramSubmode>cityTram</TramSubmode>
                <Name>
                  <Text>Straßenbahn</Text>
                  <Language>de</Language>
                </Name>
              </Mode>
              <PublishedLineName>
                <Text>Straßenbahn 4</Text>
                <Language>de</Language>
              </PublishedLineName>
              <OperatorRef>kvv:02</OperatorRef>
              <RouteDescription>
                <Text>Waldstadt - Oberreut</Text>
                <Language>de</Language>
              </RouteDescription>
              <Attribute>
                <Text>
                  <Text>Niederflurwagen</Text>
                  <Language>de</Language>
                </Text>
                <Code>1102872</Code>
                <Mandatory>false</Mandatory>
              </Attribute>
              <OriginStopPointRef>de:08212:3012:1:1</OriginStopPointRef>
              <OriginText>
                <Text>Waldstadt Europäische Schule</Text>
                <Language>de</Language>
              </OriginText>
              <DestinationText>
                <Text>Oberreut (Umleitung)</Text>
                <Language>de</Language>
              </DestinationText>
              <SituationFullRef>
                <ParticipantRef xmlns="http://www.siri.org.uk/siri">KVV</ParticipantRef>
                <SituationNumber xmlns="http://www.siri.org.uk/siri">110008317_KVV_ICSKVV</SituationNumber>
              </SituationFullRef>
            </Service>
          </StopEvent>
        </StopEventResult>
      </StopEventResponse>
    </DeliveryPayload>
  </ServiceDelivery>
</Trias>
"#;
        let departures = parse_stop_event_response(xml).unwrap();
        assert!(departures.len() == 2);

        assert_eq!(
            &departures,
            &[
                Departure {
                    line: "S-Bahn S4".to_string(),
                    destination: "Albtalbahnhof über Hbf".to_string(),
                    bay: "Gleis 1 (U)".to_string(),
                    mode: "rail".to_string(),
                    timetable_time: "2024-04-11T21:30:00Z".to_string(),
                    estimated_time: "2024-04-11T21:34:00Z".to_string()
                },
                Departure {
                    line: "Straßenbahn 4".to_string(),
                    destination: "Oberreut (Umleitung)".to_string(),
                    bay: "Gleis 3".to_string(),
                    mode: "tram".to_string(),
                    timetable_time: "2024-04-11T21:32:30Z".to_string(),
                    estimated_time: "2024-04-11T21:32:30Z".to_string()
                }
            ]
        );
    }
}
