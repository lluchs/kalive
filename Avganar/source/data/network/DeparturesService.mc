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

//! Requests and handles departure data.
class DeparturesService {

    // API: SL Departures 4
    // Bronze: 10_000/month, 30/min

    hidden var _stop;

    static var isRequesting = false;

    // init

    function initialize(stop) {
        _stop = stop;
    }

    // request

    function requestDepartures() {
        if (_stop != null) {
            _requestDepartures();
        }
    }

    hidden function _requestDepartures() {
        DeparturesService.isRequesting = true;
        WatchUi.requestUpdate();

        var url = "https://api.sl.se/api2/realtimedeparturesv4.json";

        var params = {
            "key" => API_KEY_DEPARTURES,
            "siteid" => _stop.getId(),
            "timewindow" => _stop.getTimeWindow()
        };
        var options = {
            :method => Communications.HTTP_REQUEST_METHOD_GET,
            // NOTE: url doesnt work without ".json"; set type there instead of here
            //:responseType => Communications.HTTP_RESPONSE_CONTENT_TYPE_JSON,
            :headers => { "Content-Type" => Communications.REQUEST_CONTENT_TYPE_JSON }
        };

        Communications.makeWebRequest(url, params, options, method(:onReceiveDepartures));
    }

    // receive

    function onReceiveDepartures(responseCode, data) {
        DeparturesService.isRequesting = false;

        if (responseCode == ResponseError.HTTP_OK && DictUtil.hasValue(data, "ResponseData")) {
            _handleDeparturesResponseOk(data);
        }
        else {
            _stop.setResponse(new ResponseError(responseCode));

            // auto rerequest if too large
            if (_stop.shouldAutoRerequest()) {
                requestDepartures();
            }
        }

        WatchUi.requestUpdate();
    }

    hidden function _handleDeparturesResponseOk(data) {
        var statusCode = data["StatusCode"];

        // Trafiklab error
        if (statusCode != 0) {
            _stop.setResponse(new ResponseError(statusCode));

            // auto rerequest if server error
            if (_stop.shouldAutoRerequest()) {
                requestDepartures();
            }

            return;
        }

        // departure count per mode

        var modes = [ "Metros", "Buses", "Trains", "Trams", "Ships" ];
        var modeCount = 0;

        var maxDepartures = SettingsStorage.getMaxDepartures();
        var maxDeparturesPerMode = null;

        if (maxDepartures != -1) {
            // get the number of active modes
            // in order to calculate `maxDeparturesPerMode`
            for (var m = 0; m < modes.size(); m++) {
                var modeData = data["ResponseData"][modes[m]];

                if (modeData.size() > 0) {
                    modeCount++;
                }
            }

            maxDeparturesPerMode = modeCount != 0
                ? maxDepartures / modeCount
                : 0;
        }

        // departures

        var departures = [];

        for (var m = 0; m < modes.size(); m++) {
            var modeData = data["ResponseData"][modes[m]];
            var modeDepartures = [];

            var departureCount = maxDeparturesPerMode == null
                ? modeData.size()
                : MathUtil.min(maxDeparturesPerMode, modeData.size());

            for (var d = 0; d < departureCount; d++) {
                var departureData = modeData[d];

                var mode = departureData["TransportMode"];
                var group = DictUtil.get(departureData, "GroupOfLine", "");
                var line = departureData["LineNumber"];
                var destination = departureData["Destination"];
                var plannedDateTime = departureData["TimeTabledDateTime"];
                var expectedDateTime = departureData["ExpectedDateTime"];
                var deviations = DictUtil.get(departureData, "Deviations", []);

                var isRealTime = expectedDateTime != null && !expectedDateTime.equals(plannedDateTime);
                var moment = TimeUtil.localIso8601StrToMoment(expectedDateTime);
                var deviationLevel = 0;
                var deviationMessages = [];
                var cancelled = false;

                // NOTE: API limitation
                // remove duplicate "subline" in e.g. "571X X Arlandastad"
                if (destination.substring(0, 2).equals(StringUtil.charAt(line, line.length() - 1) + " ")) {
                    destination = destination.substring(2, destination.length());
                }

                // departure deviations
                for (var i = 0; i < deviations.size(); i++) {
                    var msg = DictUtil.get(deviations[i], "Text", null);
                    msg = _splitDeviationMessageByLang(msg); // (not often the case)
                    deviationMessages.add(msg);

                    if (deviations[i]["Consequence"].equals("CANCELLED")) {
                        cancelled = true;
                        continue;
                    }

                    deviationLevel = MathUtil.max(deviationLevel, deviations[i]["ImportanceLevel"]);
                }

                // remove empty messages
                deviationMessages.removeAll(null);

                var departure = new Departure(mode, group, line, destination, moment,
                    deviationLevel, deviationMessages, cancelled, isRealTime);
                modeDepartures.add(departure);
            }

            // add null because an ampty array is not matched with the `equals()` that `removeAll()` performs.
            departures.add(modeDepartures.size() != 0 ? modeDepartures : null);
        }

        // swap order of metros and buses
        ArrUtil.swap(departures, 0, 1);
        departures.removeAll(null);

        if (departures.size() != 0) {
            _stop.setResponse(departures);
        }
        else {
            _stop.setResponse(rez(Rez.Strings.msg_i_departures_none));
        }

        // stop point deviations

        var stopDeviations = data["ResponseData"]["StopPointDeviations"];
        var stopDeviationMessages = [];

        for (var i = 0; i < stopDeviations.size(); i++) {
            var msg = DictUtil.get(DictUtil.get(stopDeviations[i], "Deviation", null), "Text", null);
            msg = _splitDeviationMessageByLang(msg);
            msg = _cleanDeviationMessage(msg);

            // NOTE: API limitation
            // sometimes we get duplicate deviation messages. skip these.
            if (!ArrUtil.contains(stopDeviationMessages, msg)) {
                stopDeviationMessages.add(msg);
            }
        }

        _stop.setDeviation(stopDeviationMessages);
    }

    hidden function _splitDeviationMessageByLang(msg) {
        // some messages are in both Swedish and English,
        // separated by a " * "
        var langSeparator = " * ";
        var langSplitIndex = msg.find(langSeparator);

        if (langSplitIndex != null) {
            var isSwe = isLangSwe();

            msg = msg.substring(
                isSwe ? 0 : langSplitIndex + langSeparator.length(),
                isSwe ? langSplitIndex : msg.length());
        }

        return msg;
    }

    hidden function _cleanDeviationMessage(msg) {
        // NOTE: API limitation
        // remove references at the end of messages

        var references = [
            "Sök din resa på sl.se eller i appen.",
            "För mer information, se sl.se",
            "Se sl.se eller i appen.",
            "Läs mer på sl.se.",
            "Läs mer på sl.se",
            "Se sl.se.",
            "Se sl.se",
            ", se sl.se",
            "Läs mer på Trafikläget."
        ];

        for (var j = 0; j < references.size(); j++) {
            var refStartIndex = msg.find(references[j]);

            if (refStartIndex != null) {
                // the reference is always at the end
                msg = msg.substring(0, refStartIndex);
                // each message will contain max one reference
                break;
            }
        }

        // remove space and (less common) newline endings
        if (ArrUtil.contains([" ", "\n"], StringUtil.charAt(msg, msg.length() - 1))) {
            msg = msg.substring(0, msg.length() - 1);
        }

        return msg;
    }

}
