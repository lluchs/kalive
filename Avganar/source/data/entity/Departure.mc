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

using Toybox.Math;
using Toybox.Time;

class Departure {

    static const BIT_BUS = 8;
    static const BIT_TRAIN = 1;
    static const BIT_TRAM = 4;

    static const MODE_BUS = "bus";
    static const MODE_TRAIN = "rail";
    static const MODE_TRAM = "tram";

    static const MODE_TO_BIT = {
        MODE_BUS => BIT_BUS,
        MODE_TRAIN => BIT_TRAIN,
        MODE_TRAM => BIT_TRAM,
    };

    static const TRAIN_LINE_COLOR = {
        "S1" => 0x1cb78d,
        "S11" => 0x1cb78d,
        "S12" => 0x1cb78d,
        "S2" => 0xa066aa,
        "S3" => 0xffdd00,
        "S31" => 0x00a99d,
        "S32" => 0x00a99d,
        "S33" => 0x8d5ca6,
        "S4" => 0x9f184c,
        "S5" => 0xf8b0ac,
        "S51" => 0xf8b0ac,
        "S52" => 0xf8b0ac,
        "S6" => 0x282268,
        "S7" => 0xfff200,
        "S8" => 0x918c55,
        "S9" => 0xa6ce42,
    };

    static const TRAM_LINE_COLOR = {
        "1" => 0xf26649,
        "2" => 0x0071bc,
        "3" => 0x947139,
        "4" => 0xffcb04,
        "5" => 0x00c0f3,
        "6" => 0xa6ce42,
        "8" => 0xf7931d,
        "17" => 0x660000,
        "18" => 0x197248,
    };

    var mode;
    hidden var _line;
    hidden var _destination;
    hidden var _moment;
    hidden var _deviationLevel;
    hidden var _deviationMessages = [];

    var cancelled;
    var isRealTime;

    // init

    function initialize(mode, line, destination, moment, deviationLevel, deviationMessages,
        cancelled, isRealTime) {

        me.mode = mode;
        _line = line;
        _destination = destination;
        _moment = moment;
        _deviationLevel = deviationLevel;
        _deviationMessages = deviationMessages;

        me.cancelled = cancelled;
        me.isRealTime = isRealTime;
    }

    static function getModesKeysByBits(bits) {
        var modes = [];

        if (bits&BIT_BUS != 0) {
            modes.add(MODE_BUS);
        }
        if (bits&BIT_TRAIN != 0) {
            modes.add(MODE_TRAIN);
        }
        if (bits&BIT_TRAM != 0) {
            modes.add(MODE_TRAM);
        }

        return modes;
    }

    static function getModesStringsByBits(bits) {
        var modes = [];

        if (bits&BIT_BUS != 0) {
            modes.add(rez(Rez.Strings.itm_modes_bus));
        }
        if (bits&BIT_TRAIN != 0) {
            modes.add(rez(Rez.Strings.itm_modes_train));
        }
        if (bits&BIT_TRAM != 0) {
            modes.add(rez(Rez.Strings.itm_modes_tram));
        }

        return modes;
    }

    // get

    function toString() {
        return displayTime() + " " + _line + " " + _destination;
    }

    function displayTime() {
        if (_moment == null) {
            return rez(Rez.Strings.itm_detail_departure_null);
        }

        var now = TimeUtil.now();
        var duration = now.subtract(_moment);
        var minutes = Math.round(duration.value() / 60.0).toNumber();
        var info = Time.Gregorian.info(_moment, Time.FORMAT_SHORT);

        // NOTE: `Moment#subtract` returns a positive value. we don't need to
        // negate it here, however, because the departure is removed in
        // `Stop#_removeDepartedDepartures` after 30 seconds, i.e. before it should be negative.

        return minutes == 0
            ? rez(Rez.Strings.itm_detail_departure_now)
            : minutes > 9
            ? (info.hour.format("%02d") + ":" + info.min.format("%02d"))
            : (minutes + SettingsStorage.getMinuteSymbol());
    }

    function hasDeparted() {
        if (_moment == null) {
            return false;
        }

        // we will keep displaying "now" until 30 seconds after departure
        var margin = new Time.Duration(30);
        return TimeUtil.now().greaterThan(_moment.add(margin));
    }

    function getTextColor() {
        if (_deviationLevel >= 8) {
            return Graphene.COLOR_RED;
        }
        else if (_deviationLevel >= 6) {
            return Graphene.COLOR_VERMILION;
        }
        else if (_deviationLevel >= 4) {
            return Graphene.COLOR_AMBER;
        }
        else if (_deviationLevel >= 3) {
            return Graphene.COLOR_YELLOW;
        }
        else if (_deviationLevel >= 2) {
            return Graphene.COLOR_LT_YELLOW;
        }
        else if (_deviationLevel >= 1) {
            return Graphene.COLOR_LR_YELLOW;
        }

        return AppColors.TEXT_PRIMARY;
    }

    function getModeLetter() {
        if (mode.equals(MODE_BUS)) {
            return rez(Rez.Strings.lbl_detail_mode_letter_bus);
        }
        else if (mode.equals(MODE_TRAIN)) {
            return rez(Rez.Strings.lbl_detail_mode_letter_train);
        }
        else if (mode.equals(MODE_TRAM)) {
            return rez(Rez.Strings.lbl_detail_mode_letter_tram);
        }
        else {
            return rez(Rez.Strings.lbl_detail_mode_letter_unknown);
        }
    }

    function getModeColor() {
        if (mode.equals(MODE_BUS)) {
            return 0x91278f;
        }
        else if (mode.equals(MODE_TRAIN)) {
            return TRAIN_LINE_COLOR[_line] || (_line.substring(0, 3).equals("RB") ? 0x9d9fa1 : 0x6d6e70);
        }
        else if (mode.equals(MODE_TRAM)) {
            return TRAM_LINE_COLOR[_line] || AppColors.DEPARTURE_UNKNOWN;
        }
        else {
            return AppColors.DEPARTURE_UNKNOWN;
        }
    }

    function getDeviationMessages() {
        return _deviationMessages;
    }

}
