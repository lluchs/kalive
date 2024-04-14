use kalive_api::trias;

const LOCATION_INFORMATION_RESPONSE_XML: &str = r#"
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
        <Location>
          <Location>
            <StopPoint>
              <StopPointRef>de:08212:3002</StopPointRef>
              <StopPointName>
                <Text>Studentenhaus</Text>
                <Language>de</Language>
              </StopPointName>
              <LocalityRef>8212000:15</LocalityRef>
            </StopPoint>
            <LocationName>
              <Text>Karlsruhe</Text>
              <Language>de</Language>
            </LocationName>
            <GeoPosition>
              <Longitude>8.41809</Longitude>
              <Latitude>49.01132</Latitude>
            </GeoPosition>
          </Location>
          <Complete>true</Complete>
          <Mode>
            <PtMode>bus</PtMode>
            <BusSubmode>unknown</BusSubmode>
          </Mode>
        </Location>
        <Location>
          <Location>
            <StopPoint>
              <StopPointRef>de:08212:4201</StopPointRef>
              <StopPointName>
                <Text>Kapellenstraße</Text>
                <Language>de</Language>
              </StopPointName>
              <LocalityRef>8212000:15</LocalityRef>
            </StopPoint>
            <LocationName>
              <Text>Karlsruhe</Text>
              <Language>de</Language>
            </LocationName>
            <GeoPosition>
              <Longitude>8.41356</Longitude>
              <Latitude>49.00694</Latitude>
            </GeoPosition>
          </Location>
          <Complete>true</Complete>
          <Mode>
            <PtMode>bus</PtMode>
            <BusSubmode>unknown</BusSubmode>
          </Mode>
        </Location>
        <Location>
          <Location>
            <StopPoint>
              <StopPointRef>de:08212:622</StopPointRef>
              <StopPointName>
                <Text>Ostendstraße</Text>
                <Language>de</Language>
              </StopPointName>
              <LocalityRef>8212000:15</LocalityRef>
            </StopPoint>
            <LocationName>
              <Text>Karlsruhe</Text>
              <Language>de</Language>
            </LocationName>
            <GeoPosition>
              <Longitude>8.41630</Longitude>
              <Latitude>49.00504</Latitude>
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
              <StopPointRef>de:08212:1002</StopPointRef>
              <StopPointName>
                <Text>Kronenplatz (U)</Text>
                <Language>de</Language>
              </StopPointName>
              <LocalityRef>8212000:15</LocalityRef>
            </StopPoint>
            <LocationName>
              <Text>Karlsruhe</Text>
              <Language>de</Language>
            </LocationName>
            <GeoPosition>
              <Longitude>8.41031</Longitude>
              <Latitude>49.00931</Latitude>
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
              <StopPointRef>de:08212:80</StopPointRef>
              <StopPointName>
                <Text>Kronenplatz</Text>
                <Language>de</Language>
              </StopPointName>
              <LocalityRef>8212000:15</LocalityRef>
            </StopPoint>
            <LocationName>
              <Text>Karlsruhe</Text>
              <Language>de</Language>
            </LocationName>
            <GeoPosition>
              <Longitude>8.41007</Longitude>
              <Latitude>49.00885</Latitude>
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
              <StopPointRef>de:08212:401</StopPointRef>
              <StopPointName>
                <Text>Karl-Wilhelm-Platz</Text>
                <Language>de</Language>
              </StopPointName>
              <LocalityRef>8212000:15</LocalityRef>
            </StopPoint>
            <LocationName>
              <Text>Karlsruhe</Text>
              <Language>de</Language>
            </LocationName>
            <GeoPosition>
              <Longitude>8.42320</Longitude>
              <Latitude>49.01132</Latitude>
            </GeoPosition>
          </Location>
          <Complete>true</Complete>
          <Mode>
            <PtMode>tram</PtMode>
            <TramSubmode>cityTram</TramSubmode>
          </Mode>
        </Location>
        <Location>
          <Location>
            <StopPoint>
              <StopPointRef>de:08212:3014</StopPointRef>
              <StopPointName>
                <Text>Emil-Gött-Straße</Text>
                <Language>de</Language>
              </StopPointName>
              <LocalityRef>8212000:15</LocalityRef>
            </StopPoint>
            <LocationName>
              <Text>Karlsruhe</Text>
              <Language>de</Language>
            </LocationName>
            <GeoPosition>
              <Longitude>8.42056</Longitude>
              <Latitude>49.01334</Latitude>
            </GeoPosition>
          </Location>
          <Complete>true</Complete>
          <Mode>
            <PtMode>bus</PtMode>
            <BusSubmode>unknown</BusSubmode>
          </Mode>
        </Location>
        <Location>
          <Location>
            <StopPoint>
              <StopPointRef>de:08212:6</StopPointRef>
              <StopPointName>
                <Text>Gottesauer Platz/BGV</Text>
                <Language>de</Language>
              </StopPointName>
              <LocalityRef>8212000:15</LocalityRef>
            </StopPoint>
            <LocationName>
              <Text>Karlsruhe</Text>
              <Language>de</Language>
            </LocationName>
            <GeoPosition>
              <Longitude>8.42489</Longitude>
              <Latitude>49.00749</Latitude>
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
            <PtSituation>
              <CreationTime xmlns="http://www.siri.org.uk/siri">2024-02-26T14:16:00Z</CreationTime>
              <ParticipantRef xmlns="http://www.siri.org.uk/siri">kvv</ParticipantRef>
              <SituationNumber xmlns="http://www.siri.org.uk/siri">110008177_kvv_ICSKVV</SituationNumber>
              <Version xmlns="http://www.siri.org.uk/siri">3</Version>
              <Source xmlns="http://www.siri.org.uk/siri">
                <SourceType>other</SourceType>
              </Source>
              <Progress xmlns="http://www.siri.org.uk/siri">open</Progress>
              <ValidityPeriod xmlns="http://www.siri.org.uk/siri">
                <StartTime>2024-02-03T03:00:00Z</StartTime>
                <EndTime>2024-06-08T22:00:00Z</EndTime>
              </ValidityPeriod>
              <UnknownReason xmlns="http://www.siri.org.uk/siri">unknown</UnknownReason>
              <Priority xmlns="http://www.siri.org.uk/siri">3</Priority>
              <Audience xmlns="http://www.siri.org.uk/siri">public</Audience>
              <ScopeType xmlns="http://www.siri.org.uk/siri">line</ScopeType>
              <Planned xmlns="http://www.siri.org.uk/siri">false</Planned>
              <Language xmlns="http://www.siri.org.uk/siri"/>
              <Summary xmlns="http://www.siri.org.uk/siri" overridden="true">Karlsruhe: Fahrzeitänderungen wegen Bauarbeiten</Summary>
              <Description xmlns="http://www.siri.org.uk/siri" overridden="true">Linien S5, S31, S32</Description>
              <Detail xmlns="http://www.siri.org.uk/siri" overridden="true">&lt;p&gt;(jpe)&lt;/p&gt;
&lt;p&gt;&lt;span style="color: #9b2321;"&gt;&lt;strong&gt;Von Samstag, den 03. Februar 2024 (ab 04:00 Uhr)&lt;br /&gt;&lt;/strong&gt;&lt;/span&gt;&lt;span style="color: #9b2321;"&gt;&lt;strong&gt;bis Samstag, den 08. Juni 2024 (bis 23:59 Uhr)&lt;/strong&gt;&lt;/span&gt;&lt;/p&gt;
&lt;p&gt;werden im Karlsruher Hauptbahnhof durch InfraGO die Bahnsteige erneuert. Aus diesem Grund &amp;auml;ndern sich zwischen Karlsruhe Hbf und Karlsruhe Durlach bei 5 Z&amp;uuml;gen geringf&amp;uuml;gig die Fahrzeiten.&lt;/p&gt;
&lt;p&gt;Bitte beachten Sie die ge&amp;auml;nderten Fahrzeiten.&lt;br /&gt;&lt;br /&gt;&lt;strong&gt;Betroffene Z&amp;uuml;ge:&lt;/strong&gt;&lt;br /&gt;S5&amp;nbsp;&amp;nbsp; 85006 Pforzheim (17:34 Uhr) - Karlsruhe Hbf (18:09 Uhr)&lt;br /&gt;S32 85111 Menzingen (06:47 Uhr) - Karlsruhe Hbf (07:44 Uhr)&lt;br /&gt;S31 85115 Odenheim (08:23 Uhr) - Karlsruhe Hbf (09:12 Uhr)&lt;br /&gt;S32 85144 Karlsruhe Hbf (16:43 Uhr) - Menzingen (17:38 Uhr)&lt;br /&gt;S31 85145 Odenheim (15:43 Uhr) - Karlsruhe Hbf (16:32 Uhr)&lt;br /&gt;&lt;br /&gt;&lt;br /&gt;&lt;br /&gt;&lt;/p&gt;
&lt;p&gt;&amp;nbsp;&lt;/p&gt;</Detail>
            </PtSituation>
            <PtSituation>
              <CreationTime xmlns="http://www.siri.org.uk/siri">2024-04-10T09:06:00Z</CreationTime>
              <ParticipantRef xmlns="http://www.siri.org.uk/siri">KVV</ParticipantRef>
              <SituationNumber xmlns="http://www.siri.org.uk/siri">110008329_KVV_ICSKVV</SituationNumber>
              <Version xmlns="http://www.siri.org.uk/siri">2</Version>
              <Source xmlns="http://www.siri.org.uk/siri">
                <SourceType>other</SourceType>
              </Source>
              <Progress xmlns="http://www.siri.org.uk/siri">open</Progress>
              <ValidityPeriod xmlns="http://www.siri.org.uk/siri">
                <StartTime>2024-04-01T03:00:00Z</StartTime>
                <EndTime>2024-04-28T03:00:59Z</EndTime>
              </ValidityPeriod>
              <UnknownReason xmlns="http://www.siri.org.uk/siri">unknown</UnknownReason>
              <Priority xmlns="http://www.siri.org.uk/siri">3</Priority>
              <Audience xmlns="http://www.siri.org.uk/siri">public</Audience>
              <ScopeType xmlns="http://www.siri.org.uk/siri">line</ScopeType>
              <Planned xmlns="http://www.siri.org.uk/siri">false</Planned>
              <Language xmlns="http://www.siri.org.uk/siri"/>
              <Summary xmlns="http://www.siri.org.uk/siri" overridden="true">Bilfingen: Haltausfälle aufgrund von Bauarbeiten</Summary>
              <Description xmlns="http://www.siri.org.uk/siri" overridden="true">Linie S5</Description>
              <Detail xmlns="http://www.siri.org.uk/siri" overridden="true">&lt;p&gt;Haltausf&amp;auml;lle in Bilfingen auf der Linie S5 &lt;br /&gt;&lt;br /&gt;&lt;span style="color: #9b2321;"&gt;&lt;strong&gt;von Montag , den 1. April 2024 (ab 05:00 Uhr)&lt;/strong&gt;&lt;/span&gt;&lt;br /&gt;&lt;span style="color: #9b2321;"&gt;&lt;strong&gt;bis Sonntag, den 28. April 2024 (bis 05:00 Uhr)&lt;/strong&gt;&lt;/span&gt;&lt;br /&gt;&lt;br /&gt;Im genannten Zeitraum werden Bahnsteigarbeiten am Haltepunkt Bilfingen durchgef&amp;uuml;hrt. Dadurch kommt es zu Haltausf&amp;auml;llen, da der Bahnsteig in Richtung Pforzheim nicht zur Verf&amp;uuml;gung steht:&lt;br /&gt;&lt;br /&gt;Tags&amp;uuml;ber zwischen 5 und 21 Uhr bestehen keine Einschr&amp;auml;nkungen, alle Z&amp;uuml;ge halten am Bahnsteig in Richtung Karlsruhe. &lt;br /&gt;&lt;br /&gt;Nachts zwischen 21 und 5 Uhr entf&amp;auml;llt der Halt Bilfingen in Richtung Pforzheim. Fahrg&amp;auml;ste aus Karlsruhe kommend steigen bitte in Wilferdingen-Singen in den Ersatzverkehr nach Bilfingen um. Fahrg&amp;auml;ste aus Bilfingen, die in Richtung Pforzheim zusteigen wollen, nutzen bitte den Ersatzverkehr oder die S5 in Richtung Karlsruhe, um in Wilferdingen-Singen in die S5 nach Pforzheim zu steigen.&lt;/p&gt;</Detail>
            </PtSituation>
            <PtSituation>
              <CreationTime xmlns="http://www.siri.org.uk/siri">2024-03-27T11:42:00Z</CreationTime>
              <ParticipantRef xmlns="http://www.siri.org.uk/siri">KVV</ParticipantRef>
              <SituationNumber xmlns="http://www.siri.org.uk/siri">110008327_KVV_ICSKVV</SituationNumber>
              <Version xmlns="http://www.siri.org.uk/siri">2</Version>
              <Source xmlns="http://www.siri.org.uk/siri">
                <SourceType>other</SourceType>
              </Source>
              <Progress xmlns="http://www.siri.org.uk/siri">open</Progress>
              <ValidityPeriod xmlns="http://www.siri.org.uk/siri">
                <StartTime>2024-04-01T03:00:00Z</StartTime>
                <EndTime>2024-04-28T19:00:59Z</EndTime>
              </ValidityPeriod>
              <UnknownReason xmlns="http://www.siri.org.uk/siri">unknown</UnknownReason>
              <Priority xmlns="http://www.siri.org.uk/siri">3</Priority>
              <Audience xmlns="http://www.siri.org.uk/siri">public</Audience>
              <ScopeType xmlns="http://www.siri.org.uk/siri">line</ScopeType>
              <Planned xmlns="http://www.siri.org.uk/siri">false</Planned>
              <Language xmlns="http://www.siri.org.uk/siri"/>
              <Summary xmlns="http://www.siri.org.uk/siri" overridden="true">Pforzheim - Wilferdingen-Singen: Zugausfälle aufgrund von Streckensperrung</Summary>
              <Description xmlns="http://www.siri.org.uk/siri" overridden="true">Linie S5</Description>
              <Detail xmlns="http://www.siri.org.uk/siri" overridden="true">&lt;p&gt;Zugausf&amp;auml;lle auf der Linie S5 zwischen Pforzheim und Wilferdingen-Singen &lt;br /&gt;&lt;br /&gt;&lt;span style="color: #9b2321;"&gt;&lt;strong&gt;Von Montag , den 1. April 2024 (jeweils ab 05:00 Uhr)&lt;/strong&gt;&lt;/span&gt;&lt;br /&gt;&lt;span style="color: #9b2321;"&gt;&lt;strong&gt;bis Sonntag, den 28. April 2024 (jeweils bis 21:00 Uhr)&lt;/strong&gt;&lt;/span&gt;&lt;br /&gt;&lt;br /&gt;Im genannten Zeitraum ist ein Gleis der Strecke zwischen Pforzheim Hbf und Wilferdingen-Singen wegen Arbeiten an der Infrastruktur der Deutschen Bahn gesperrt. Aus diesem Grund entf&amp;auml;llt tags&amp;uuml;ber ungef&amp;auml;hr jede zweite Stadtbahn der Linie S5 zwischen Pforzheim Hbf und Wilferdingen-Singen. Dar&amp;uuml;ber hinaus kann es im genannten Zeitraum zu unregelm&amp;auml;&amp;szlig;igen Zugausf&amp;auml;llen kommen. Fahrg&amp;auml;ste informieren sich bitte tagesaktuell in der Verbindungsauskunft. &lt;br /&gt;Ein Ersatzverkehr mit Bussen wird zwischen Pforzheim Hbf und Wilferdingen-Singen eingerichtet.&lt;br /&gt;&lt;br /&gt;Hinweise:&lt;br /&gt;- am 1. und 2. April fallen zus&amp;auml;tzlich Z&amp;uuml;ge zwischen Pforzheim und S&amp;ouml;llingen Reetzstra&amp;szlig;e aus. &lt;br /&gt;- vom 1. bis 28. April kommt es zu Haltausf&amp;auml;llen in Bilfingen.&lt;/p&gt;</Detail>
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
          <ResultId>ID-7BA8D821-60CF-4A1D-AD25-C28A106A7A8E</ResultId>
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
                  <TimetabledTime>2024-04-11T21:34:00Z</TimetabledTime>
                  <EstimatedTime>2024-04-11T21:35:00Z</EstimatedTime>
                </ServiceDeparture>
                <StopSeqNumber>24</StopSeqNumber>
              </CallAtStop>
            </ThisCall>
            <Service>
              <OperatingDayRef>2024-04-11</OperatingDayRef>
              <JourneyRef>kvv:22305:E:H:j24:19554</JourneyRef>
              <LineRef>kvv:22305:E:H</LineRef>
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
                <Text>S-Bahn S5</Text>
                <Language>de</Language>
              </PublishedLineName>
              <OperatorRef>kvv:01</OperatorRef>
              <RouteDescription>
                <Text>Pforzheim -  Wörth</Text>
                <Language>de</Language>
              </RouteDescription>
              <Attribute>
                <Text>
                  <Text>Stufenloses Fahrzeug, WLAN</Text>
                  <Language>de</Language>
                </Text>
                <Code>1102873</Code>
                <Mandatory>false</Mandatory>
              </Attribute>
              <OriginStopPointRef>de:08231:50:2:3</OriginStopPointRef>
              <OriginText>
                <Text>Pforzheim Hauptbahnhof</Text>
                <Language>de</Language>
              </OriginText>
              <DestinationText>
                <Text>Rheinbergstraße</Text>
                <Language>de</Language>
              </DestinationText>
              <SituationFullRef>
                <ParticipantRef xmlns="http://www.siri.org.uk/siri">kvv</ParticipantRef>
                <SituationNumber xmlns="http://www.siri.org.uk/siri">110008177_kvv_ICSKVV</SituationNumber>
              </SituationFullRef>
              <SituationFullRef>
                <ParticipantRef xmlns="http://www.siri.org.uk/siri">KVV</ParticipantRef>
                <SituationNumber xmlns="http://www.siri.org.uk/siri">110008062_KVV_ICSKVV</SituationNumber>
              </SituationFullRef>
              <SituationFullRef>
                <ParticipantRef xmlns="http://www.siri.org.uk/siri">KVV</ParticipantRef>
                <SituationNumber xmlns="http://www.siri.org.uk/siri">110008329_KVV_ICSKVV</SituationNumber>
              </SituationFullRef>
              <SituationFullRef>
                <ParticipantRef xmlns="http://www.siri.org.uk/siri">KVV</ParticipantRef>
                <SituationNumber xmlns="http://www.siri.org.uk/siri">110008327_KVV_ICSKVV</SituationNumber>
              </SituationFullRef>
            </Service>
          </StopEvent>
        </StopEventResult>
        <StopEventResult>
          <ResultId>ID-612961EF-AED5-464B-9E79-38EA2EBAB852</ResultId>
          <StopEvent>
            <ThisCall>
              <CallAtStop>
                <StopPointRef>de:08212:1001:92:2</StopPointRef>
                <StopPointName>
                  <Text>KA Durlacher Tor/KIT-Campus Süd (U)</Text>
                  <Language>de</Language>
                </StopPointName>
                <PlannedBay>
                  <Text>Gleis 2 (U)</Text>
                  <Language>de</Language>
                </PlannedBay>
                <ServiceDeparture>
                  <TimetabledTime>2024-04-11T21:34:00Z</TimetabledTime>
                  <EstimatedTime>2024-04-11T21:35:00Z</EstimatedTime>
                </ServiceDeparture>
                <StopSeqNumber>33</StopSeqNumber>
              </CallAtStop>
            </ThisCall>
            <Service>
              <OperatingDayRef>2024-04-11</OperatingDayRef>
              <JourneyRef>kvv:22351:E:R:j24:19407</JourneyRef>
              <LineRef>kvv:22351:E:R</LineRef>
              <DirectionRef>return</DirectionRef>
              <Mode>
                <PtMode>rail</PtMode>
                <RailSubmode>suburbanRailway</RailSubmode>
                <Name>
                  <Text>S-Bahn</Text>
                  <Language>de</Language>
                </Name>
              </Mode>
              <PublishedLineName>
                <Text>S-Bahn S51</Text>
                <Language>de</Language>
              </PublishedLineName>
              <OperatorRef>kvv:01</OperatorRef>
              <RouteDescription>
                <Text>Germersheim - Pforzheim</Text>
                <Language>de</Language>
              </RouteDescription>
              <Attribute>
                <Text>
                  <Text>Stufenloses Fahrzeug, WLAN, WC, Klimaanlage</Text>
                  <Language>de</Language>
                </Text>
                <Code>1102874</Code>
                <Mandatory>false</Mandatory>
              </Attribute>
              <OriginStopPointRef>de:07334:32637:4:4</OriginStopPointRef>
              <OriginText>
                <Text>Germersheim, Bahnhof</Text>
                <Language>de</Language>
              </OriginText>
              <DestinationText>
                <Text>Söllingen Reetzstraße</Text>
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
          <ResultId>ID-E635FD4A-08DC-4498-806E-3EF69C77BA97</ResultId>
          <StopEvent>
            <ThisCall>
              <CallAtStop>
                <StopPointRef>de:08212:1001:1:1</StopPointRef>
                <StopPointName>
                  <Text>KA Durlacher Tor/KIT-Campus Süd (U)</Text>
                  <Language>de</Language>
                </StopPointName>
                <PlannedBay>
                  <Text>Gleis 1 (U)</Text>
                  <Language>de</Language>
                </PlannedBay>
                <ServiceDeparture>
                  <TimetabledTime>2024-04-11T21:35:42Z</TimetabledTime>
                  <EstimatedTime>2024-04-11T21:36:42Z</EstimatedTime>
                </ServiceDeparture>
                <StopSeqNumber>14</StopSeqNumber>
              </CallAtStop>
            </ThisCall>
            <Service>
              <OperatingDayRef>2024-04-11</OperatingDayRef>
              <JourneyRef>kvv:21002:E:H:j24:613</JourneyRef>
              <LineRef>kvv:21002:E:H</LineRef>
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
                <Text>Straßenbahn 2</Text>
                <Language>de</Language>
              </PublishedLineName>
              <OperatorRef>kvv:02</OperatorRef>
              <RouteDescription>
                <Text>Wolfartsweier - Siemensallee</Text>
                <Language>de</Language>
              </RouteDescription>
              <Attribute>
                <Text>
                  <Text>Niederflurwagen</Text>
                  <Language>de</Language>
                </Text>
                <Code>1102875</Code>
                <Mandatory>false</Mandatory>
              </Attribute>
              <OriginStopPointRef>de:08212:4500:1:1</OriginStopPointRef>
              <OriginText>
                <Text>Wolfartsweier Nord</Text>
                <Language>de</Language>
              </OriginText>
              <DestinationText>
                <Text>Knielingen Nord über Hbf (Umleitung)</Text>
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
          <ResultId>ID-416C847C-F246-471E-94B2-A891FEA16E5F</ResultId>
          <StopEvent>
            <ThisCall>
              <CallAtStop>
                <StopPointRef>de:08212:1001:2:2</StopPointRef>
                <StopPointName>
                  <Text>KA Durlacher Tor/KIT-Campus Süd (U)</Text>
                  <Language>de</Language>
                </StopPointName>
                <PlannedBay>
                  <Text>Gleis 2 (U)</Text>
                  <Language>de</Language>
                </PlannedBay>
                <ServiceDeparture>
                  <TimetabledTime>2024-04-11T21:39:06Z</TimetabledTime>
                  <EstimatedTime>2024-04-11T21:39:06Z</EstimatedTime>
                </ServiceDeparture>
                <StopSeqNumber>26</StopSeqNumber>
              </CallAtStop>
            </ThisCall>
            <Service>
              <OperatingDayRef>2024-04-11</OperatingDayRef>
              <JourneyRef>kvv:21012:E:R:j24:300</JourneyRef>
              <LineRef>kvv:21012:E:R</LineRef>
              <DirectionRef>return</DirectionRef>
              <Mode>
                <PtMode>rail</PtMode>
                <RailSubmode>suburbanRailway</RailSubmode>
                <Name>
                  <Text>S-Bahn</Text>
                  <Language>de</Language>
                </Name>
              </Mode>
              <PublishedLineName>
                <Text>S-Bahn S2</Text>
                <Language>de</Language>
              </PublishedLineName>
              <OperatorRef>kvv:02</OperatorRef>
              <RouteDescription>
                <Text>Rheinstetten - Spöck</Text>
                <Language>de</Language>
              </RouteDescription>
              <Attribute>
                <Text>
                  <Text>Niederflurwagen</Text>
                  <Language>de</Language>
                </Text>
                <Code>1102876</Code>
                <Mandatory>false</Mandatory>
              </Attribute>
              <OriginStopPointRef>de:08215:1501:2:2</OriginStopPointRef>
              <OriginText>
                <Text>Mörsch Bach-West</Text>
                <Language>de</Language>
              </OriginText>
              <DestinationText>
                <Text>Reitschulschlag</Text>
                <Language>de</Language>
              </DestinationText>
            </Service>
          </StopEvent>
        </StopEventResult>
        <StopEventResult>
          <ResultId>ID-F47059B4-CBD5-4F05-8D89-15B774CD9130</ResultId>
          <StopEvent>
            <ThisCall>
              <CallAtStop>
                <StopPointRef>de:08212:1001:1:1</StopPointRef>
                <StopPointName>
                  <Text>KA Durlacher Tor/KIT-Campus Süd (U)</Text>
                  <Language>de</Language>
                </StopPointName>
                <PlannedBay>
                  <Text>Gleis 1 (U)</Text>
                  <Language>de</Language>
                </PlannedBay>
                <ServiceDeparture>
                  <TimetabledTime>2024-04-11T21:39:12Z</TimetabledTime>
                  <EstimatedTime>2024-04-11T21:39:12Z</EstimatedTime>
                </ServiceDeparture>
                <StopSeqNumber>23</StopSeqNumber>
              </CallAtStop>
            </ThisCall>
            <Service>
              <OperatingDayRef>2024-04-11</OperatingDayRef>
              <JourneyRef>kvv:21012:E:H:j24:864</JourneyRef>
              <LineRef>kvv:21012:E:H</LineRef>
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
                <Text>S-Bahn S2</Text>
                <Language>de</Language>
              </PublishedLineName>
              <OperatorRef>kvv:02</OperatorRef>
              <RouteDescription>
                <Text>Spöck - Rheinstetten</Text>
                <Language>de</Language>
              </RouteDescription>
              <Attribute>
                <Text>
                  <Text>Niederflurwagen</Text>
                  <Language>de</Language>
                </Text>
                <Code>1102877</Code>
                <Mandatory>false</Mandatory>
              </Attribute>
              <OriginStopPointRef>de:08215:32296:1:1</OriginStopPointRef>
              <OriginText>
                <Text>Spöck Richard-Hecht-Schule</Text>
                <Language>de</Language>
              </OriginText>
              <DestinationText>
                <Text>Rheinstetten</Text>
                <Language>de</Language>
              </DestinationText>
            </Service>
          </StopEvent>
        </StopEventResult>
        <StopEventResult>
          <ResultId>ID-1CE08C22-6289-4AEE-B20A-56A6F0B40F09</ResultId>
          <StopEvent>
            <ThisCall>
              <CallAtStop>
                <StopPointRef>de:08212:1001:2:2</StopPointRef>
                <StopPointName>
                  <Text>KA Durlacher Tor/KIT-Campus Süd (U)</Text>
                  <Language>de</Language>
                </StopPointName>
                <PlannedBay>
                  <Text>Gleis 2 (U)</Text>
                  <Language>de</Language>
                </PlannedBay>
                <ServiceDeparture>
                  <TimetabledTime>2024-04-11T21:42:06Z</TimetabledTime>
                  <EstimatedTime>2024-04-11T21:42:06Z</EstimatedTime>
                </ServiceDeparture>
                <StopSeqNumber>28</StopSeqNumber>
              </CallAtStop>
            </ThisCall>
            <Service>
              <OperatingDayRef>2024-04-11</OperatingDayRef>
              <JourneyRef>kvv:21002:E:R:j24:293</JourneyRef>
              <LineRef>kvv:21002:E:R</LineRef>
              <DirectionRef>return</DirectionRef>
              <Mode>
                <PtMode>tram</PtMode>
                <TramSubmode>cityTram</TramSubmode>
                <Name>
                  <Text>Straßenbahn</Text>
                  <Language>de</Language>
                </Name>
              </Mode>
              <PublishedLineName>
                <Text>Straßenbahn 2</Text>
                <Language>de</Language>
              </PublishedLineName>
              <OperatorRef>kvv:02</OperatorRef>
              <RouteDescription>
                <Text>Siemensallee - Wolfartsweier</Text>
                <Language>de</Language>
              </RouteDescription>
              <Attribute>
                <Text>
                  <Text>Niederflurwagen</Text>
                  <Language>de</Language>
                </Text>
                <Code>1102878</Code>
                <Mandatory>false</Mandatory>
              </Attribute>
              <OriginStopPointRef>de:08212:240:2:2</OriginStopPointRef>
              <OriginText>
                <Text>Knielingen Nord</Text>
                <Language>de</Language>
              </OriginText>
              <DestinationText>
                <Text>Wolfartsweier</Text>
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
          <ResultId>ID-30E0974C-A971-4E30-824F-5C9D0FFF1612</ResultId>
          <StopEvent>
            <ThisCall>
              <CallAtStop>
                <StopPointRef>de:08212:3:4:4</StopPointRef>
                <StopPointName>
                  <Text>KA Durlacher Tor/KIT-Campus Süd</Text>
                  <Language>de</Language>
                </StopPointName>
                <PlannedBay>
                  <Text>Gleis 4</Text>
                  <Language>de</Language>
                </PlannedBay>
                <ServiceDeparture>
                  <TimetabledTime>2024-04-11T21:45:30Z</TimetabledTime>
                  <EstimatedTime>2024-04-11T21:45:30Z</EstimatedTime>
                </ServiceDeparture>
                <StopSeqNumber>17</StopSeqNumber>
              </CallAtStop>
            </ThisCall>
            <Service>
              <OperatingDayRef>2024-04-11</OperatingDayRef>
              <JourneyRef>kvv:21004:E:R:j24:210</JourneyRef>
              <LineRef>kvv:21004:E:R</LineRef>
              <DirectionRef>return</DirectionRef>
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
                <Text>Oberreut - Waldstadt</Text>
                <Language>de</Language>
              </RouteDescription>
              <Attribute>
                <Text>
                  <Text>Niederflurwagen</Text>
                  <Language>de</Language>
                </Text>
                <Code>1102879</Code>
                <Mandatory>false</Mandatory>
              </Attribute>
              <OriginStopPointRef>de:08212:611:2:2</OriginStopPointRef>
              <OriginText>
                <Text>Oberreut Badeniaplatz</Text>
                <Language>de</Language>
              </OriginText>
              <DestinationText>
                <Text>Waldstadt</Text>
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
          <ResultId>ID-B26F679A-7E12-4CFE-B143-5D4D69E80348</ResultId>
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
                  <TimetabledTime>2024-04-11T21:46:06Z</TimetabledTime>
                  <EstimatedTime>2024-04-11T21:46:06Z</EstimatedTime>
                </ServiceDeparture>
                <StopSeqNumber>7</StopSeqNumber>
              </CallAtStop>
            </ThisCall>
            <Service>
              <OperatingDayRef>2024-04-11</OperatingDayRef>
              <JourneyRef>kvv:21003:E:H:j24:1069</JourneyRef>
              <LineRef>kvv:21003:E:H</LineRef>
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
                <Text>Straßenbahn 3</Text>
                <Language>de</Language>
              </PublishedLineName>
              <OperatorRef>kvv:02</OperatorRef>
              <RouteDescription>
                <Text>Rintheim - Daxlanden/Rappenwört</Text>
                <Language>de</Language>
              </RouteDescription>
              <Attribute>
                <Text>
                  <Text>Niederflurwagen</Text>
                  <Language>de</Language>
                </Text>
                <Code>1102880</Code>
                <Mandatory>false</Mandatory>
              </Attribute>
              <OriginStopPointRef>de:08212:314:1:1</OriginStopPointRef>
              <OriginText>
                <Text>Rintheim</Text>
                <Language>de</Language>
              </OriginText>
              <DestinationText>
                <Text>Daxlanden über Hbf (Umleitung)</Text>
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

fn main() {
    let departures = trias::parse_stop_event_response(STOP_EVENT_RESPONSE_XML).unwrap();
    println!("{:#?}", departures);

    let locations =
        trias::parse_location_information_response(LOCATION_INFORMATION_RESPONSE_XML).unwrap();
    println!("{:#?}", locations);
}
