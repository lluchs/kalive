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

using Toybox.Activity;
using Toybox.Lang;
using Toybox.Math;
using Toybox.Position;
using Toybox.WatchUi;

//! The Footprint module provides extended position functionality
(:glance)
module Footprint {

    var onRegisterPosition = null;
    var isPositionRegistered = false;

    // position, in radians
    var _lat = 0.0;
    var _lon = 0.0;

    // set

    function setPosLoc(positionLocation as Position.Location or Null) {
        if (positionLocation != null) {
            _lat = positionLocation.toRadians()[0].toDouble();
            _lon = positionLocation.toRadians()[1].toDouble();
        }
    }

    // get

    function isPositioned() {
        return _lat != 0.0 || _lon != 0.0;
    }

    //! Get latitude in radians
    function getLatRad() {
        return _lat;
    }

    //! Get longitude in radians
    function getLonRad() {
        return _lon;
    }

    //! Get latitude in degrees
    function getLatDeg() {
        return MathUtil.deg(_lat);
    }

    //! Get longitude in degrees
    function getLonDeg() {
        return MathUtil.deg(_lon);
    }

    function distanceTo(lat, lon) {
        return distanceBetween(lat, lon, _lat, _lon);
    }

    // static

    //! Radians to meters
    function distanceBetween(lat1, lon1, lat2, lon2) {
        var R = 6371000;

        var phi1 = lat1 - Math.PI / 2;
        var phi2 = lat2 - Math.PI / 2;

        var x1 = R * Math.sin(phi1) * Math.cos(lon1);
        var y1 = R * Math.sin(phi1) * Math.sin(lon1);
        var z1 = R * Math.cos(phi1);

        var x2 = R * Math.sin(phi2) * Math.cos(lon2);
        var y2 = R * Math.sin(phi2) * Math.sin(lon2);
        var z2 = R * Math.cos(phi2);

        var dx = x2 - x1;
        var dy = y2 - y1;
        var dz = z2 - z1;

        var distance = Math.sqrt(dx * dx + dy * dy + dz * dz);

        return distance;
    }

    // registration

    function enableLocationEvents(continuous) {
        Position.enableLocationEvents(continuous ? Position.LOCATION_CONTINUOUS : Position.LOCATION_ONE_SHOT,
            new Lang.Method(Footprint, :registerPosition));
    }

    function disableLocationEvents() {
        onRegisterPosition = null;
        Position.enableLocationEvents(Position.LOCATION_DISABLE, null);
    }

    //! Get last location while waiting for location event
    //! @param info Activity info
    function registerLastKnownPosition() {
        var activityInfo = Activity.getActivityInfo();
        setPosLoc(activityInfo.currentLocation);

        if (onRegisterPosition != null) {
            onRegisterPosition.invoke();
        }
    }

    //! Location event listener delegation
    function registerPosition(positionInfo) {
        setPosLoc(positionInfo.position);

        if (onRegisterPosition != null) {
            onRegisterPosition.invoke();
        }
        isPositionRegistered = true;

        WatchUi.requestUpdate();
    }

}
