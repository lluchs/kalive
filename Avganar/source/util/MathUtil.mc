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

module MathUtil {

    const TAU = 6.28318530717958647692;

    // geometry

    //! Get carteesian coordinates from polar coordinates
    //! @param angle The argument of the position, in radians
    //! @param x0, y0 The x- and y-coordinates of the center
    function polarPos(amp, angle, x0, y0) as [Number, Number] {
        var x = amp * Math.cos(angle) + x0;
        // multiply by -1 to handle screen y increase downwards
        var y = -1 * amp * Math.sin(angle) + y0;
        return [x, y];
    }

    //! Calculate the leftmost x-coordinate of a circular screen at a specific y
    function minX(y, r) {
        if (y < 0 || y > 2 * r) {
            return null;
        }
        return -Math.sqrt(Math.pow(r, 2) - Math.pow(y - r, 2)) + r;
    }

    //! Calculate the rightmost x-coordinate of a circular screen at a specific y
    function maxX(y, r) {
        if (y < 0 || y > 2 * r) {
            return null;
        }
        return Math.sqrt(Math.pow(r, 2) - Math.pow(y - r, 2)) + r;
    }

    function deg(rad) {
        return 360 * rad / TAU;
    }


    function rad(deg) {
        return TAU * deg / 360;
    }

    // misc

    function min(a, b) {
        return a <= b ? a : b;
    }

    function max(a, b) {
        return a >= b ? a : b;
    }

    function coerceIn(value, min, max) {
        return min > max ? null : (value < min ? min : (value > max ? max : value));
    }

    function abs(x) {
        return x < 0 ? -x : x;
    }

    //! The Monkey C modulo operator uses truncated division, which gives the remainder with same sign as the dividend.
    //! This uses floored division, which gives the remainder with same sign as the divisor.
    function mod(dividend, divisor) {
        var quotient = Math.floor(dividend.toFloat() / divisor.toFloat()).toNumber();
        var remainder = dividend - divisor * quotient;
        return remainder;
    }

    //

    function recursiveShare(share, prevVal, level) {
        var newVal = prevVal + (1 - prevVal) * share;
        return level <= 0 ? newVal : recursiveShare(share, newVal, level - 1);
    }

}
