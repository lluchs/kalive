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

using Toybox.System;
using Toybox.Time;
using Toybox.Test;
import Toybox.Lang;

module TimeUtil {

    function now() {
        return new Time.Moment(Time.now().value());
    }

    function parseRFC3339(str as String) as Time.Moment or Null {
        var year = str.substring(0, 4).toNumber();
        if (!str.substring(4, 5).equals("-")) { return null; }
        var month = str.substring(5, 7).toNumber();
        if (!str.substring(7, 8).equals("-")) { return null; }
        var day = str.substring(8, 10).toNumber();
        if (!str.substring(10, 11).equals("T")) { return null; }
        var hour = str.substring(11, 13).toNumber();
        if (!str.substring(13, 14).equals(":")) { return null; }
        var minute = str.substring(14, 16).toNumber();
        if (!str.substring(16, 17).equals(":")) { return null; }
        var second = str.substring(17, 19).toNumber();
        var tzHour = 0;
        var tzMinute = 0;
        var sign = 1;
        switch (str.substring(19, 20)) {
            case "-":
                sign = -1;
            case "+":
                tzHour = sign * str.substring(20, 22).toNumber();
                if (!str.substring(22, 23).equals(":")) { return null; }
                tzMinute = sign * str.substring(23, 25).toNumber();
                break;
            case "Z":
                break;
            default:
                return null;
        }
        
        return Time.Gregorian.moment({
            :year => year,
            :month => month,
            :day => day,
            :hour => hour,
            :minute => minute,
            :second => second
        }).subtract(new Time.Duration(tzHour * 3600 + tzMinute * 60));
    
    }

    (:test)
    function testParseRFC3339(logger as Test.Logger) as Boolean {
        var parsed = parseRFC3339("2024-04-18T04:43:00Z");
        return parsed.value() == 1713415380;
    }

}
