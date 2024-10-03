// This file is part of Avgånär.
//
// Avgånär is free software: you can redistribute it and/or modify it under the terms of
// the GNU General Public License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any later version.
//
// Avgånär is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with Avgånär.
// If not, see <https://www.gnu.org/licenses/>.

using Toybox.Communications;
using Toybox.WatchUi;
import Toybox.Lang;

//! Requests and handles departure data.
class DeparturesService {

    hidden var _stop;

    static var isRequesting = false;

    // init

    function initialize(stop) {
        _stop = stop;
    }

    // request

    function requestDepartures() {
        DeparturesService.isRequesting = true;
        WatchUi.requestUpdate();

        var url = "https://kalive-api.lwrl.de/departures";

        var params = {
            "stop" => _stop.getId()
        };

        var options = {
            :method => Communications.HTTP_REQUEST_METHOD_GET,
            :responseType => Communications.HTTP_RESPONSE_CONTENT_TYPE_JSON,
            :headers => {
                "Authorization" => "Bearer " + API_SECRET,
            }
        };

        Communications.makeWebRequest(url, params, options, method(:onReceiveDepartures));
    }

    // receive

    function onReceiveDepartures(responseCode as Number, data as Null or Dictionary or String) as Void {
        DeparturesService.isRequesting = false;

        if (responseCode != ResponseError.HTTP_OK) {
            _stop.setResponse(new ResponseError(responseCode));

            // auto-refresh if too large
            if (_stop.shouldAutoRefresh()) {
                requestDepartures();
            }
        }
        else if (!DictUtil.hasValue(data, "departures")) {
            var errorMsg = DictUtil.get(data, "error", "no error msg");
            _stop.setResponse(new ResponseError(errorMsg));

            // auto-refresh if server error
            // TODO: probably can't happen with new API
            // – but look for messages which might correspond
            // with the previous server errors
            /*if (_stop.shouldAutoRefresh()) {
                requestDepartures();
            }*/
        }
        else {
            _handleDeparturesResponseOk(data);
        }

        WatchUi.requestUpdate();
    }

    hidden function _findDeviation(deviations as Array<Dictionary>, ref as Dictionary) as Dictionary or Null {
        for (var i = 0; i < deviations.size(); i++) {
            if (deviations[i]["participant_ref"] == ref["participant_ref"] && 
                deviations[i]["participant_ref"] == ref["participant_ref"]) {
                return deviations[i];
            }
        }
        return null;
    }

    hidden function _handleDeparturesResponseOk(data as Dictionary) {
        var deviationsData = data["situations"] as Array<Dictionary>;
        var departuresData = data["departures"] as Array<Dictionary>;

        if (departuresData.size() == 0) {
            _stop.setResponse(rez(Rez.Strings.msg_i_departures_none));
        }

        var maxDepartures = SettingsStorage.getMaxDepartures();
        var departureCount = maxDepartures == -1
            ? departuresData.size()
            : MathUtil.min(departuresData.size(), maxDepartures);

        var departures = [];

        for (var d = 0; d < departureCount; d++) {
            var departureData = departuresData[d];

            var mode = departureData["mode"];
            var bay = departureData["bay"];
            var line = departureData["line"];
            var destination = departureData["destination"];
            var plannedDateTime = departureData["timetable_time"];
            var expectedDateTime = departureData["estimated_time"];
            var cancelled = departureData["cancelled"];
            var deviations = departureData["situations"] as Array<Dictionary>;

            var isRealTime = expectedDateTime != null;
            var moment = TimeUtil.parseRFC3339(plannedDateTime);
            if (isRealTime) {
                moment = TimeUtil.parseRFC3339(expectedDateTime);
            }
            var deviationLevel = 0;
            var deviationMessages = [];
            for (var i = 0; i < deviations.size(); i++) {
                var deviation = _findDeviation(deviationsData, deviations[i]);
                if (deviation == null) { continue; }
                deviationLevel = MathUtil.max(deviationLevel, deviation["priority"]);
                deviationMessages.add(deviation["description"] + "\n" + deviation["summary"]);
            }

            var departure = new Departure(mode, bay, line, destination, moment,
                deviationLevel, deviationMessages, cancelled, isRealTime);

            departures.add(departure);
        }

        departures.sort(new ArrUtil.SortComparator());

        if (departures.size() != 0) {
            _stop.setResponse(departures);
        }
        else {
            _stop.setResponse(rez(Rez.Strings.msg_i_departures_none));
        }

    }


}
