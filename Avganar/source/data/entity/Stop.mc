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

//! Must have the same interface as `StopDouble` since we often don't
//! know whether our stops are of `Stop` or `StopDouble`.
class Stop {

    hidden static var _SERVER_AUTO_REQUEST_LIMIT = 4;

    // NOTE: instead of adding public fields, add getters.
    // and when adding functions, remember to add
    // corresponding ones to ´StopDouble´

    var name;

    hidden var _id;
    hidden var _products = null;
    hidden var _response;
    hidden var _bays;
    hidden var _failedRequestCount = 0;
    hidden var _deviationMessages = [];
    hidden var _timeStamp;

    // init

    function initialize(id, name, products) {
        _id = id;
        _products = products;
        me.name = name;
    }

    function equals(other) {
        return (other instanceof Stop || other instanceof StopDouble || other instanceof StopDummy)
            && other.getId() == _id && other.name.equals(name);
    }

    // set

    function setProducts(products) {
        _products = products;
    }

    function setResponse(response) {
        _response = response;
        _timeStamp = TimeUtil.now();

        if (_response instanceof ResponseError && _response.isServerError()) {
            _failedRequestCount++;
            return;
        }

        // only vibrate if we are not auto-refreshing
        vibrate();
        _failedRequestCount = 0;

        _bays = {};
        for (var i = 0; i < _response.size(); i++) {
            var bay = _response[i].getBay();
            if (_bays.hasKey(bay)) {
                _bays[bay].add(_response[i]);
            } else {
                _bays.put(bay, [_response[i]]);
            }
        }
    }

    function resetResponse() {
        _response = null;
        _timeStamp = null;
    }

    function resetResponseError() {
        if (_response instanceof ResponseError) {
            resetResponse();
        }
    }

    function setDeviation(messages) {
        _deviationMessages = messages;
    }

    // get

    function getId() {
        return _id;
    }

    function getProducts() {
        return _products;
    }

    function getResponse() {
        return _response;
    }

    function getFailedRequestCount() {
        return _failedRequestCount;
    }

    function getDeviationMessages() {
        return _deviationMessages;
    }

    function getBays() as Array<String> {
        var bays = _bays.keys();
        bays.sort(null);
        return bays;
    }

    function getDepartures(bay as String or Null) as Array<Departure> {
        if (bay) {
            return _bays.get(bay);
        } else {
            return _response;
        }
    }

    function shouldAutoRefresh() {
        if (!(_response instanceof ResponseError)) {
            return false;
        }

        if (_failedRequestCount >= _SERVER_AUTO_REQUEST_LIMIT && _response.isServerError()) {
            setResponse(new ResponseError(ResponseError.CODE_AUTO_REQUEST_LIMIT_SERVER));
            return false;
        }

        return _response.isAutoRefreshable();
    }

    function getDataAgeMillis() {
        return _response instanceof Array || _response instanceof String
            ? TimeUtil.now().subtract(_timeStamp).value() * 1000
            : null;
    }

    hidden function _removeDepartedDepartures(mode) {
        if (_response[mode] == null || _response[mode].size() == 0 || !_response[mode][0].hasDeparted()) {
            return;
        }

        var firstIndex = -1;

        for (var i = 1; i < _response[mode].size(); i++) {
            // once we get the first departure that has not departed,
            // add it and everything after
            if (!_response[mode][i].hasDeparted()) {
                firstIndex = i;
                break;
            }
        }

        if (firstIndex != -1) {
            _response[mode] = _response[mode].slice(firstIndex, null);
        }
        else {
            // add null because an ampty array is not matched with the equals() that removeAll() performes.
            _response[mode] = null;
        }
    }

}
