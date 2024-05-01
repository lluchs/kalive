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

import Toybox.Lang;
using Toybox.Communications;
using Toybox.WatchUi;

// Requests and handles stop data.
module NearbyStopsService {

    var isRequesting = false;

    // request

    function requestNearbyStops(lat, lon) {
        // final check if location use is turned off
        if (!SettingsStorage.getUseLocation()) {
            NearbyStopsStorage.setResponseError(null);
            WatchUi.requestUpdate();
        } else if (lat == 0 && lon == 0) {
            // lat = 49.009;
            // lon = 8.417;
            WatchUi.requestUpdate();
        } else {
            _requestNearbyStops(lat, lon);
        }
    }

    function _requestNearbyStops(lat, lon) {
        isRequesting = true;

        var url = "https://kalive-api.lwrl.de/stops";

        var params = {
            "latitude" => lat,
            "longitude" => lon,
            // TODO: Configurable number of results?
            //"maxNo" => def(NearbyStopsStorage.maxStops, SettingsStorage.getMaxStops()),
        };
        var options = {
            :method => Communications.HTTP_REQUEST_METHOD_GET,
            :responseType => Communications.HTTP_RESPONSE_CONTENT_TYPE_JSON,
            :headers => {
                "Authorization" => "Bearer " + API_SECRET,
            }
        };

        Communications.makeWebRequest(url, params, options, new Method(NearbyStopsService, :onReceiveNearbyStops));
    }

    // receive

    function onReceiveNearbyStops(responseCode, data) {
        isRequesting = false;

        if (responseCode == ResponseError.HTTP_OK && data != null) {
            _handleNearbyStopsResponseOk(data);
        }
        else {
            NearbyStopsStorage.setResponseError(new ResponseError(DictUtil.get(data, "error", responseCode)));

            // auto-refresh if too large
            if (NearbyStopsStorage.shouldAutoRefresh()) {
                requestNearbyStops(Footprint.getLatDeg(), Footprint.getLonDeg());
            }
        }

        WatchUi.requestUpdate();
    }

    function _handleNearbyStopsResponseOk(data as Dictionary) {

        // no stops were found
        if (!DictUtil.hasValue(data, "stops") || data["stops"].size() == 0) {
            NearbyStopsStorage.setResponseError(rez(Rez.Strings.msg_i_stops_none));
            return;
        }

        // stops were found

        var stopIds = [];
        var stopNames = [];
        var stopProducts = [];
        var stops = [];

        var stopsData = data["stops"];
        for (var i = 0; i < stopsData.size(); i++) {
            var stopData = stopsData[i];

            var id = stopData["id"];
            var name = stopData["name"];
            // TODO: Do something with `modes`?
            var products = null;

            // null if duplicate
            var stop = NearbyStopsStorage.createStop(id, name, products, stops, stopIds, stopNames);
            if (stop == null) {
                continue;
            }

            stopIds.add(id);
            stopNames.add(name);
            stopProducts.add(products);
            stops.add(stop);
        }

        NearbyStopsStorage.setResponse(stopIds, stopNames, stopProducts, stops);
    }

}
