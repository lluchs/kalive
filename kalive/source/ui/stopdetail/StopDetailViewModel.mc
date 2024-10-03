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
using Toybox.Math;
using Toybox.Timer;
using Toybox.WatchUi;

class StopDetailViewModel {

    static hidden const _REFRESH_TIME_INTERVAL = 15 * 1000;
    static hidden const _REQUEST_TIME_INTERVAL = 2 * 60 * 1000;

    static const DEPARTURES_PER_PAGE = 4;

    var stop as Stop;
    var pageCount = 1;
    var pageCursor = 0;
    var currentBay = null;
    var departureCursor = 0;
    var isDepartureState = false;

    hidden var _lastPageDepartureCount = 0;
    hidden var _delayTimer = new Timer.Timer();
    hidden var _repeatTimer = new TimerWrapper();

    // init

    function initialize(stop as Stop) {
        me.stop = stop;

        if (stop.getResponse() == null) {
            _requestDepartures();
        }
    }

    // request

    function enableRequests() {
        _requestDeparturesDelayed();
    }

    function disableRequests() {
        _delayTimer.stop();
        _repeatTimer.stop();
    }

    hidden function _requestDeparturesDelayed() {
        var age = stop.getDataAgeMillis();
        // never request more frequently than _REQUEST_TIME_INTERVAL.
        var delay = age == null ? 0 : _REQUEST_TIME_INTERVAL - age;

        // 50 ms is the minimum time value
        if (delay <= 50) {
            onDelayedDeparturesRequest();
        }
        else {
            _delayTimer.start(method(:onDelayedDeparturesRequest), delay, false);
        }
    }

    function onDelayedDeparturesRequest() {
        _requestDepartures();
        _startRepeatTimer();
    }

    hidden function _startRepeatTimer() {
        var screenTimer = new TimerRepr(new Method(WatchUi, :requestUpdate), 1);
        var requestTimer = new TimerRepr(method(:onTimer), _REQUEST_TIME_INTERVAL / _REFRESH_TIME_INTERVAL);

        _repeatTimer.start(_REFRESH_TIME_INTERVAL, [ screenTimer, requestTimer ]);
    }

    function onTimer() {
        if (stop.getResponse() instanceof ResponseError
            && !stop.getResponse().isTimerRefreshable()) {
            return;
        }

        _requestDepartures();
    }

    hidden function _requestDepartures() {
        new DeparturesService(stop).requestDepartures();
    }

    // read

    //! Get only the departures that should be
    //! displayed on the current page
    function getPageResponse() {
        var departures = stop.getDepartures(currentBay);

        if (!(departures instanceof Array)) {
            pageCount = 1;
            isDepartureState = false;
            return departures;
        }

        _lastPageDepartureCount = departures.size() % DEPARTURES_PER_PAGE;
        if (_lastPageDepartureCount == 0) {
            _lastPageDepartureCount = DEPARTURES_PER_PAGE;
        }
        pageCount = Math.ceil(departures.size().toFloat() / DEPARTURES_PER_PAGE).toNumber();

        // coerce cursor
        pageCursor = MathUtil.min(pageCursor, pageCount - 1);

        // get page range
        var startIndex = pageCursor * DEPARTURES_PER_PAGE;
        var endIndex = startIndex + DEPARTURES_PER_PAGE;

        // slice to page range
        return departures.slice(startIndex, endIndex);
    }

    function canNavigateToDeviation() {
        return !isDepartureState
            && pageCursor == 0
            && stop.getDeviationMessages().size() != 0;
    }

    // write

    function toggleDepartureState() {
        isDepartureState = !isDepartureState;
        departureCursor = 0;
        WatchUi.requestUpdate();
    }

    //! Scrolling down
    function incCursor() {
        if (stop.getResponse() instanceof ResponseError
            && stop.getResponse().isUserRefreshable()) {

            // refresh
            stop.resetResponse();
            _requestDepartures();
            WatchUi.requestUpdate();
        }
        else if (isDepartureState) {
            if (departureCursor < DEPARTURES_PER_PAGE - 1
                && (pageCursor < pageCount - 1 || departureCursor < _lastPageDepartureCount - 1)) {

                departureCursor++;
            }
            else if (_incPageCursor()) {
                departureCursor = 0;
            }
        }
        else {
            _incPageCursor();
        }

        WatchUi.requestUpdate();
    }

    //! Scrolling up
    function decCursor() {
        if (isDepartureState) {
            if (departureCursor > 0) {
                departureCursor--;
            }
            else if (_decPageCursor()) {
                departureCursor = DEPARTURES_PER_PAGE - 1;
            }
        }
        else {
            _decPageCursor();
        }

        WatchUi.requestUpdate();
    }

    hidden function _incPageCursor() {
        if (pageCursor < pageCount - 1) {
            pageCursor++;
            return true;
        }

        return false;
    }

    hidden function _decPageCursor() {
        if (pageCursor > 0) {
            pageCursor--;
            return true;
        }
        else if (canNavigateToDeviation()) {
            DialogView.push(null, stop.getDeviationMessages(), Rez.Drawables.ic_warning, WatchUi.SLIDE_DOWN);
        }

        return false;
    }

    function onSelect() {
        if (isDepartureState) {
            var departures = stop.getDepartures(currentBay);
            var selectedDeparture = departures[pageCursor * DEPARTURES_PER_PAGE + departureCursor];
            var messages = selectedDeparture.getDeviationMessages();

            if (messages.size() == 0) {
                messages.add(rez(Rez.Strings.lbl_detail_deviation_none));
            }

            DialogView.push(null, messages, Rez.Drawables.ic_warning, WatchUi.SLIDE_LEFT);
        }
        else {
            onNextBay();
        }
    }

    function onNextBay() {
        var bays = stop.getBays();
        if (bays.size() > 1 || currentBay != null) {
            if (currentBay == null) {
                currentBay = bays[0];
            } else {
                var idx = bays.indexOf(currentBay);
                if (idx == -1 || idx == bays.size() - 1) {
                    currentBay = null;
                } else {
                    currentBay = bays[idx + 1];
                }
            }
            pageCursor = 0;
            WatchUi.requestUpdate();
        }
    }

}
