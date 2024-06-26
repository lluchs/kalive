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

//! The colors used in the app.
module AppColors {

    const BACKGROUND = Graphene.COLOR_BLACK;
    const BACKGROUND_INVERTED = Graphene.COLOR_WHITE;

    const TEXT_PRIMARY = Graphene.COLOR_WHITE;
    const TEXT_SECONDARY = Graphene.COLOR_LT_GRAY;
    const TEXT_TERTIARY = Graphene.COLOR_DK_GRAY;
    const TEXT_INVERTED = Graphene.COLOR_BLACK;

    const PRIMARY = 0x782121;
    const PRIMARY_DK = 0x501616;
    const PRIMARY_LT = 0xc83737;

    const ON_PRIMARY = Graphene.COLOR_WHITE;
    const ON_PRIMARY_SECONDARY = 0xd35f5f;
    const ON_PRIMARY_TERTIARY = 0xd35f5f;

    const WARNING = Graphene.COLOR_VERMILION;
    const ERROR = Graphene.COLOR_RED;

    // departure

    const DEPARTURE_BUS_RED = Graphene.COLOR_RED;
    const DEPARTURE_BUS_BLUE = Graphene.COLOR_BLUE;
    const DEPARTURE_BUS_REPLACEMENT = WARNING;

    const DEPARTURE_METRO_RED = Graphene.COLOR_DR_RED;
    const DEPARTURE_METRO_BLUE = Graphene.COLOR_DR_BLUE;
    const DEPARTURE_METRO_GREEN = Graphene.COLOR_DR_GREEN;

    const DEPARTURE_TRAM_SPÅRVÄGCITY = Graphene.COLOR_DK_GRAY;
    const DEPARTURE_TRAM_NOCKEBYBANAN = Graphene.COLOR_LT_GRAY;
    const DEPARTURE_TRAM_LIDINGÖBANAN = Graphene.COLOR_AMBER;
    const DEPARTURE_TRAM_TVÄRBANAN = Graphene.COLOR_DK_ORANGE;
    const DEPARTURE_TRAM_SALTSJÖBANAN = Graphene.COLOR_DK_CYAN;
    const DEPARTURE_TRAM_ROSLAGSBANAN = Graphene.COLOR_DK_VIOLET;

    const DEPARTURE_TRAIN = Graphene.COLOR_CERISE;
    const DEPARTURE_SHIP = Graphene.COLOR_CAPRI;
    const DEPARTURE_NONE = Graphene.COLOR_BLACK;
    const DEPARTURE_UNKNOWN = Graphene.COLOR_BLACK;

    const DEPARTURE_REALTIME = Graphene.COLOR_GREEN;
    const DEPARTURE_SELECTED = Graphene.COLOR_GREEN;

}
