use std::io::BufRead;

use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use chrono::prelude::*;
use quick_xml::events::{BytesText, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;

use crate::SituationRef;
use crate::{Departure, Situation, Stop};

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
                    <NumberOfResults>16</NumberOfResults>
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

/// Fix-up of TRIAS data.
fn process_departure(d: &mut Departure) {
    // S-Bahn S1 => S1
    if let Some(line) = d.line.strip_prefix(&d.mode_name) {
        d.line = line[1..].to_string();
    }

    // ICE 76 InterCityExpress => ICE 76
    if let Some(line) = d.line.strip_suffix(&d.mode_name) {
        d.line = line[..line.len() - 1].to_string();
    }

    // XML entities
    d.destination = d
        .destination
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&amp;", "&");
}

/// Parse a TRIAS StopEventResponse XML.
pub fn parse_stop_event_response(xml: &str) -> Result<(Vec<Departure>, Vec<Situation>)> {
    let mut reader = Reader::from_str(xml);
    let mut situations = Vec::new();
    let mut situation_ref: SituationRef = Default::default();
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
                b"Situations" => {
                    situations = parse_situations(&mut reader)?;
                }
                b"Text" | b"TimetabledTime" | b"EstimatedTime" | b"PtMode" | b"ParticipantRef"
                | b"SituationNumber" | b"Cancelled" => {
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
                    let mut d = current_departure.take().unwrap();
                    process_departure(&mut d);
                    departures.push(d);
                }
                b"Text" => in_text = false,
                b"PlannedBay" => set_to_text!(bay),
                b"TimetabledTime" => set_to_text!(timetable_time),
                b"EstimatedTime" => set_to_text!(estimated_time),
                b"PtMode" => set_to_text!(mode),
                b"Name" => set_to_text!(mode_name),
                b"PublishedLineName" => set_to_text!(line),
                b"DestinationText" => set_to_text!(destination),
                b"ParticipantRef" => {
                    if let Some(text) = current_text.take() {
                        situation_ref.participant_ref = text.into();
                    }
                }
                b"SituationNumber" => {
                    if let Some(text) = current_text.take() {
                        situation_ref.situation_number = text.into();
                    }
                }
                b"SituationFullRef" => {
                    if let Some(ref mut dep) = current_departure {
                        dep.situations.push(std::mem::take(&mut situation_ref));
                    }
                }
                b"Cancelled" => {
                    if let Some(text) = current_text.take() {
                        if text == "true" {
                            if let Some(ref mut dep) = current_departure {
                                dep.cancelled = true;
                            }
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    Ok((departures, situations))
}

fn parse_situations<R: BufRead>(reader: &mut Reader<R>) -> Result<Vec<Situation>> {
    let mut situations: Vec<Situation> = Vec::new();
    let mut situation: Situation = Default::default();
    let mut in_text = false;
    let mut current_text: Option<String> = None;
    let mut buf = Vec::new();
    macro_rules! set_to_text {
        ($prop:ident) => {
            if let Some(text) = current_text.take() {
                situation.$prop = text.into();
            }
        };
    }
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Eof => bail!("unexpected EOF in PtSituation"),
            Event::Start(e) => match e.name().as_ref() {
                b"PtSituation" => {
                    situation = Default::default();
                }
                b"ParticipantRef" | b"SituationNumber" | b"CreationTime" | b"StartTime"
                | b"EndTime" | b"Priority" | b"ScopeType" | b"Summary" | b"Description"
                | b"Detail" => {
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
                b"Situations" => break,
                b"PtSituation" => {
                    situations.push(std::mem::take(&mut situation));
                }
                b"ParticipantRef" => set_to_text!(participant_ref),
                b"SituationNumber" => set_to_text!(situation_number),
                b"CreationTime" => set_to_text!(creation_time),
                b"StartTime" => set_to_text!(validity_start_time),
                b"EndTime" => set_to_text!(validity_end_time),
                b"Priority" => {
                    if let Some(text) = current_text.take() {
                        situation.priority = text.parse().unwrap_or_default();
                    }
                }
                b"ScopeType" => set_to_text!(scope_type),
                b"Summary" => set_to_text!(summary),
                b"Description" => set_to_text!(description),
                b"Detail" => set_to_text!(detail),
                _ => {}
            },
            _ => {}
        }
    }
    Ok(situations)
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
                    <NumberOfResults>16</NumberOfResults>
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
            <PtSituation>
              <CreationTime xmlns="http://www.siri.org.uk/siri">2024-01-29T11:41:00Z</CreationTime>
              <ParticipantRef xmlns="http://www.siri.org.uk/siri">KVV</ParticipantRef>
              <SituationNumber xmlns="http://www.siri.org.uk/siri">110008062_KVV_ICSKVV</SituationNumber>
              <Version xmlns="http://www.siri.org.uk/siri">9</Version>
              <Source xmlns="http://www.siri.org.uk/siri">
                <SourceType>other</SourceType>
              </Source>
              <Progress xmlns="http://www.siri.org.uk/siri">open</Progress>
              <ValidityPeriod xmlns="http://www.siri.org.uk/siri">
                <StartTime>2023-12-10T02:00:00Z</StartTime>
                <EndTime>2500-12-30T23:00:59Z</EndTime>
              </ValidityPeriod>
              <UnknownReason xmlns="http://www.siri.org.uk/siri">unknown</UnknownReason>
              <Priority xmlns="http://www.siri.org.uk/siri">3</Priority>
              <Audience xmlns="http://www.siri.org.uk/siri">public</Audience>
              <ScopeType xmlns="http://www.siri.org.uk/siri">line</ScopeType>
              <Planned xmlns="http://www.siri.org.uk/siri">false</Planned>
              <Language xmlns="http://www.siri.org.uk/siri"/>
              <Summary xmlns="http://www.siri.org.uk/siri" overridden="true">AVG: temporäre Anpassung des Fahrplanangebotes</Summary>
              <Description xmlns="http://www.siri.org.uk/siri" overridden="true">Linien S4, S5, S6, S7, S8, S12, S31, S32</Description>
              <Detail xmlns="http://www.siri.org.uk/siri" overridden="true">&lt;p&gt;(kue) Die Albtal-Verkehrs-Gesellschaft (AVG) wird&lt;br /&gt;&lt;br /&gt;&lt;span style="color: #c30a37;"&gt;&lt;strong&gt;vom 08. Januar 2024&lt;/strong&gt;&lt;/span&gt;&lt;br /&gt;&lt;span style="color: #c30a37;"&gt;&lt;strong&gt;bis 08. Juni 2024&lt;/strong&gt;&lt;/span&gt;&lt;br /&gt;&lt;br /&gt;das Fahrplanangebot auf ihren Stadtbahnlinien reduzieren. Grund hierf&amp;uuml;r ist die angespannte Personalsituation. Die gezielte Ausd&amp;uuml;nnung umfasst weniger als drei Prozent der gesamten Verkehrsleistung, die die AVG erbringt.&lt;br /&gt;&lt;br /&gt;Die einzelnen Fahrplan&amp;auml;nderungen &lt;strong&gt;ab dem 08. Januar 2024 i&lt;/strong&gt;m &amp;Uuml;berblick:&lt;br /&gt;&lt;br /&gt;&lt;strong&gt;Linie S12&lt;/strong&gt;&lt;br /&gt;- es entf&amp;auml;llt ein Zugpaar zwischen Ittersbach Rathaus und Karlsruhe Rheinhafen. Betroffen hiervon sind folgende Verbindungen:&lt;br /&gt;&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; Linie S12 (40005), 7:37 &amp;ndash; 8:40 Uhr Ittersbach Rathaus (Abfahrt 7:37 Uhr) &amp;ndash; Karlsruhe Rheinhafen (Ankunft 8.40 Uhr)&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; Linie S12 (40008), Karlsruhe Rheinhafen (Abfahrt 16:25)&amp;nbsp; &amp;ndash; Ittersbach Rathaus (Ankunft 17:30 Uhr)&lt;br /&gt;&lt;br /&gt;&lt;strong&gt;Linie S31&lt;/strong&gt;&lt;br /&gt;- montags bis freitags entf&amp;auml;llt der Zwischentakt Ubstadt Ort &amp;ndash; Odenheim am Nachmittag zwischen 13 und 18 Uhr (jeweils Abfahrt zur Minute :40 in Ubstadt Ort bzw. zur Minute :03 in Odenheim); das bedeutet eine Fahrplan-Reduzierung auf einen 20/40-Takt (der Menzinger Zugteil f&amp;auml;hrt jeweils)&lt;br /&gt;&lt;br /&gt;- samstags und sonntags entf&amp;auml;llt die H&amp;auml;lfte der Fahrten im Abschnitt Ubstadt Ort &amp;ndash; Odenheim (jeweils Abfahrt zur Minute :00 in Ubstadt Ort bzw. zur Minute :23 in Odenheim); das bedeutet eine Reduzierung auf einen Stundentakt (der Menzinger Zugteil f&amp;auml;hrt jeweils)&lt;br /&gt;&lt;br /&gt;&lt;strong&gt;Linie S4&lt;/strong&gt;&lt;br /&gt;- es entf&amp;auml;llt t&amp;auml;glich der Zwischentakt zwischen Bretten und Flehingen (au&amp;szlig;er in der Hauptverkehrszeit): Die Fahrten der Linie S4 aus Richtung Karlsruhe enden zur Minute :22 in Bretten Stadtmitte und fahren von dort zur Minute :34 wieder zur&amp;uuml;ck nach Karlsruhe.&lt;br /&gt;&lt;br /&gt;- montags bis freitags entfallen nachmittags folgende Verdichterfahrten zwischen Heilbronn Hbf-Vorplatz (Willy-Brandt-Platz) und Heilbronn Pf&amp;uuml;hlpark/Weinsberg:&lt;br /&gt;&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 14:51 Uhr HN Hbf-Vorplatz &amp;ndash; 15:06 Uhr Weinsberg&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 15:23 Uhr HN Pf&amp;uuml;hlpark &amp;ndash; 15:33 Uhr HN Hbf-Vorplatz&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 15:49 Uhr Weinsberg &amp;ndash; 16:05 Uhr HN Hbf-Vorplatz&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 15:51 Uhr HN Hbf-Vorplatz &amp;ndash; 16:06 Uhr Weinsberg&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 16:23 Uhr HN Pf&amp;uuml;hlpark &amp;ndash; 16:33 Uhr HN Hbf-Vorplatz&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 16:49 Uhr Weinsberg &amp;ndash; 17:05 Uhr HN Hbf-Vorplatz&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 16:51 Uhr HN Hbf-Vorplatz &amp;ndash; 17:06 Uhr Weinsberg&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 17:49 Uhr Weinsberg &amp;ndash; 18:05 Uhr HN Hbf-Vorplatz&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 17:51 Uhr HN Hbf-Vorplatz &amp;ndash; 18:06 Uhr Weinsberg&lt;br /&gt;&lt;br /&gt;&lt;strong&gt;Linie S5&lt;/strong&gt;&lt;br /&gt;- montags bis freitags entfallen nach der morgendlichen Hauptverkehrszeit zwischen 9 und 19 Uhr je zwei Fahrten pro Stunde zwischen Karlsruhe Tullastra&amp;szlig;e und Knielingen Rheinbergstra&amp;szlig;e. Diese Bahnen der Linie S5, die sonst zu den Minuten :12 und :52 an der Rheinbergstra&amp;szlig;e beginnen und dort zu den Minuten :4 und :44 enden, verkehren nur noch im Abschnitt zwischen Tullastra&amp;szlig;e und Pfinztal.&lt;/p&gt;
&lt;p&gt;- sonntags entfallen folgende Verbindungen:&lt;br /&gt;&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 07:11 Uhr KA Tullastra&amp;szlig;e &amp;ndash; 07:53 Uhr W&amp;ouml;rth Badepark&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 08:05 Uhr W&amp;ouml;rth Badepark &amp;ndash; 09:06 Uhr S&amp;ouml;llingen Bf&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 08:12 Uhr Knielingen Rheinbergstra&amp;szlig;e &amp;ndash; 08:51 Uhr Berghausen&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 09:05 Uhr Berghausen &amp;ndash; 09:44 Uhr Knielingen Rheinbergstra&amp;szlig;e&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 09:51 Uhr S&amp;ouml;llingen Bf &amp;ndash; 10:53 Uhr W&amp;ouml;rth Badepark&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 09:52 Uhr Knielingen Rheinbergstra&amp;szlig;e &amp;ndash; 10:37 Uhr S&amp;ouml;llingen Bf&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 10:51 Uhr S&amp;ouml;llingen Bf &amp;ndash; 11:53 Uhr W&amp;ouml;rth Badepark&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 11:05 Uhr W&amp;ouml;rth Badepark &amp;ndash; 12:06 Uhr S&amp;ouml;llingen Bf&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 12:05 Uhr W&amp;ouml;rth Badepark &amp;ndash; 13:06 Uhr S&amp;ouml;llingen Bf&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 12:51 Uhr S&amp;ouml;llingen Bf &amp;ndash; 13:53 Uhr W&amp;ouml;rth Badepark&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 13:18 Uhr S&amp;ouml;llingen Bf &amp;ndash; 14:04 Uhr Knielingen Rheinbergstra&amp;szlig;e&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 14:05 Uhr W&amp;ouml;rth Badepark &amp;ndash; 15:06 Uhr S&amp;ouml;llingen Bf&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 14:12 Uhr Knielingen Rheinbergstra&amp;szlig;e &amp;ndash; 14:51 Uhr Berghausen&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 15:05 Uhr Berghausen &amp;ndash; 15:44 Uhr Knielingen Rheinbergstra&amp;szlig;e&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 15:51 Uhr S&amp;ouml;llingen Bf &amp;ndash; 16:53 Uhr W&amp;ouml;rth Badepark&lt;br /&gt;&amp;nbsp;&amp;nbsp;&amp;nbsp; 17:05 Uhr W&amp;ouml;rth Badepark &amp;ndash; 17:45 Uhr KA Tullastra&amp;szlig;e&lt;/p&gt;
&lt;p&gt;&lt;br /&gt;&lt;strong&gt;Linien S32, S6, S7 und S8&lt;/strong&gt;&lt;br /&gt;- Auf diesen Linien entfallen einzelne Fahrtabschnitte.&lt;/p&gt;</Detail>
            </PtSituation>
            <PtSituation>
              <CreationTime xmlns="http://www.siri.org.uk/siri">2024-03-20T13:05:00Z</CreationTime>
              <ParticipantRef xmlns="http://www.siri.org.uk/siri">KVV</ParticipantRef>
              <SituationNumber xmlns="http://www.siri.org.uk/siri">110008317_KVV_ICSKVV</SituationNumber>
              <Version xmlns="http://www.siri.org.uk/siri">1</Version>
              <Source xmlns="http://www.siri.org.uk/siri">
                <SourceType>other</SourceType>
              </Source>
              <Progress xmlns="http://www.siri.org.uk/siri">open</Progress>
              <ValidityPeriod xmlns="http://www.siri.org.uk/siri">
                <StartTime>2024-03-23T05:45:00Z</StartTime>
                <EndTime>2024-04-15T01:30:59Z</EndTime>
              </ValidityPeriod>
              <UnknownReason xmlns="http://www.siri.org.uk/siri">unknown</UnknownReason>
              <Priority xmlns="http://www.siri.org.uk/siri">3</Priority>
              <Audience xmlns="http://www.siri.org.uk/siri">public</Audience>
              <ScopeType xmlns="http://www.siri.org.uk/siri">line</ScopeType>
              <Planned xmlns="http://www.siri.org.uk/siri">false</Planned>
              <Language xmlns="http://www.siri.org.uk/siri"/>
              <Summary xmlns="http://www.siri.org.uk/siri" overridden="true">Kaiserplatz: Streckensperrung aufgrund von Gleisarbeiten</Summary>
              <Description xmlns="http://www.siri.org.uk/siri" overridden="true">Linien S12, 2, 3, 4 und 9</Description>
              <Detail xmlns="http://www.siri.org.uk/siri" overridden="true">&lt;p&gt;Die VBK-Strecke im Bereich Kaiserstra&amp;szlig;e wird&lt;/p&gt;
&lt;p&gt;&lt;span style="color: #9b2321;"&gt;&lt;strong&gt;von Samstag, 23. M&amp;auml;rz, 6.45 Uhr &lt;/strong&gt;&lt;/span&gt;&lt;/p&gt;
&lt;p&gt;&lt;span style="color: #9b2321;"&gt;&lt;strong&gt;bis Montag, 15. April, 3.30 Uhr&lt;/strong&gt; &lt;/span&gt;&lt;/p&gt;
&lt;p&gt;in beiden Fahrtrichtungen f&amp;uuml;r den Bahnverkehr komplett gesperrt. W&amp;auml;hrend der Bauarbeiten am Kaiserplatz wird kein eigener Ersatzverkehr eingerichtet. Daf&amp;uuml;r verkehrt w&amp;auml;hrend der Ma&amp;szlig;nahme aber die Tramlinie 9. Diese Linie pendelt zwischen den Haltestellen Europaplatz/Postgalerie und der Haltestelle Tivoli. Die Linie f&amp;auml;hrt dabei &amp;uuml;ber den Hauptbahnhof. Zum genauen Routenverlauf der Linie 9: Europaplatz/Postgalerie &amp;ndash; Karlstor/Bundesgerichtshof &amp;ndash; Mathystra&amp;szlig;e &amp;ndash; Kolpingplatz &amp;ndash; Ebertstra&amp;szlig;e &amp;ndash; Hauptbahnhof &amp;ndash; Poststra&amp;szlig;e &amp;ndash; Tivoli.&lt;/p&gt;</Detail>
            </PtSituation>
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
        <StopEventResult>
          <ResultId>ID-839340DA-5FE6-4520-A0D3-A66784CA79D6</ResultId>
          <StopEvent>
            <ThisCall>
              <CallAtStop>
                <StopPointRef>de:08212:90:2:2</StopPointRef>
                <StopPointName>
                  <Text>Karlsruhe Hauptbahnhof</Text>
                  <Language>de</Language>
                </StopPointName>
                <ServiceDeparture>
                  <TimetabledTime>2024-04-27T02:03:00Z</TimetabledTime>
                </ServiceDeparture>
                <StopSeqNumber>13</StopSeqNumber>
              </CallAtStop>
            </ThisCall>
            <Service>
              <OperatingDayRef>2024-04-27</OperatingDayRef>
              <JourneyRef>ddb:96N43::H:j24:60403</JourneyRef>
              <LineRef>ddb:96N43::H</LineRef>
              <DirectionRef>outward</DirectionRef>
              <Mode>
                <PtMode>rail</PtMode>
                <RailSubmode>interregionalRail</RailSubmode>
                <Name>
                  <Text>InterCity</Text>
                  <Language>de</Language>
                </Name>
              </Mode>
              <PublishedLineName>
                <Text>IC 60403 InterCity</Text>
                <Language>de</Language>
              </PublishedLineName>
              <OperatorRef>ddb:</OperatorRef>
              <OriginStopPointRef>NL:S:asd</OriginStopPointRef>
              <OriginText>
                <Text>Arnhem Centraal</Text>
                <Language>de</Language>
              </OriginText>
              <DestinationText>
                <Text>Zürich HB</Text>
                <Language>de</Language>
              </DestinationText>
            </Service>
          </StopEvent>
        </StopEventResult>
        <StopEventResult>
          <ResultId>ID-BE808EA9-27C0-44BE-B513-AA91C6EC2014</ResultId>
          <StopEvent>
            <ThisCall>
              <CallAtStop>
                <StopPointRef>de:08212:98:1:1</StopPointRef>
                <StopPointName>
                  <Text>Karlsruhe Poststraße</Text>
                  <Language>de</Language>
                </StopPointName>
                <PlannedBay>
                  <Text>Gleis 1</Text>
                  <Language>de</Language>
                </PlannedBay>
                <ServiceDeparture>
                  <TimetabledTime>2024-09-23T19:39:00Z</TimetabledTime>
                </ServiceDeparture>
                <StopSeqNumber>35</StopSeqNumber>
                <NotServicedStop>true</NotServicedStop>
              </CallAtStop>
            </ThisCall>
            <Service>
              <OperatingDayRef>2024-09-23</OperatingDayRef>
              <JourneyRef>kvv:22304:E:H:s24:19926</JourneyRef>
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
                <Code>934952</Code>
                <Mandatory>false</Mandatory>
              </Attribute>
              <OriginStopPointRef>de:08125:5404:1:1</OriginStopPointRef>
              <OriginText>
                <Text>Eppingen</Text>
                <Language>de</Language>
              </OriginText>
              <DestinationText>
                <Text>Albtalbahnhof über Hbf</Text>
                <Language>de</Language>
              </DestinationText>
              <Cancelled>true</Cancelled>
            </Service>
          </StopEvent>
        </StopEventResult>
       </StopEventResponse>
    </DeliveryPayload>
  </ServiceDelivery>
</Trias>
"#;
        let (departures, situations) = parse_stop_event_response(xml).unwrap();
        assert!(departures.len() == 4);
        assert!(situations.len() == 2);
        println!("{:?}", situations);

        assert_eq!(
            &departures,
            &[
                Departure {
                    line: "S4".to_string(),
                    destination: "Albtalbahnhof über Hbf".to_string(),
                    bay: Some("Gleis 1 (U)".to_string()),
                    mode: "rail".to_string(),
                    mode_name: "S-Bahn".to_string(),
                    timetable_time: "2024-04-11T21:30:00Z".to_string(),
                    estimated_time: Some("2024-04-11T21:34:00Z".to_string()),
                    cancelled: false,
                    situations: [SituationRef {
                        participant_ref: "KVV".to_string(),
                        situation_number: "110008062_KVV_ICSKVV".to_string()
                    }]
                    .into(),
                },
                Departure {
                    line: "4".to_string(),
                    destination: "Oberreut (Umleitung)".to_string(),
                    bay: Some("Gleis 3".to_string()),
                    mode: "tram".to_string(),
                    mode_name: "Straßenbahn".to_string(),
                    timetable_time: "2024-04-11T21:32:30Z".to_string(),
                    estimated_time: Some("2024-04-11T21:32:30Z".to_string()),
                    cancelled: false,
                    situations: [SituationRef {
                        participant_ref: "KVV".to_string(),
                        situation_number: "110008317_KVV_ICSKVV".to_string()
                    }]
                    .into(),
                },
                Departure {
                    line: "IC 60403".to_string(),
                    destination: "Zürich HB".to_string(),
                    bay: None,
                    mode: "rail".to_string(),
                    mode_name: "InterCity".to_string(),
                    timetable_time: "2024-04-27T02:03:00Z".to_string(),
                    estimated_time: None,
                    cancelled: false,
                    situations: Vec::new(),
                },
                Departure {
                    line: "S4".to_string(),
                    destination: "Albtalbahnhof über Hbf".to_string(),
                    bay: Some("Gleis 1".to_string()),
                    mode: "rail".to_string(),
                    mode_name: "S-Bahn".to_string(),
                    timetable_time: "2024-09-23T19:39:00Z".to_string(),
                    estimated_time: None,
                    cancelled: true,
                    situations: Vec::new(),
                },
            ]
        );
    }
}
