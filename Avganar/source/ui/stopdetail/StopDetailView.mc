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
using Toybox.Graphics;
using Toybox.Math;
using Toybox.Time;
using Toybox.WatchUi;

class StopDetailView extends WatchUi.View {

    hidden var _viewModel;

    // init

    function initialize(viewModel) {
        View.initialize();
        _viewModel = viewModel;
    }

    // override View

    function onShow() {
        _viewModel.enableRequests();
    }

    function onUpdate(dc as Graphics.Dc) {
        View.onUpdate(dc);

        // draw
        Graphite.enableAntiAlias(dc);
        _draw(dc);
    }

    function onHide() {
        _viewModel.disableRequests();
        _viewModel.stop.resetResponseError();
    }

    // draw

    hidden function _draw(dc as Graphics.Dc) {
        var stop = _viewModel.stop;

        // text
        _drawHeader(dc, stop);
        _drawFooter(dc, stop, false);

        // departures
        var response = _viewModel.getPageResponse();
        if (response instanceof Array) {
            _drawDepartures(dc, response);

            // indicator: page
            dc.setColor(AppColors.ON_PRIMARY, AppColors.PRIMARY);
            WidgetUtil.drawVerticalPageArrows(dc, _viewModel.pageCount, _viewModel.pageCursor,
                AppColors.TEXT_TERTIARY, AppColors.ON_PRIMARY_TERTIARY);
            WidgetUtil.drawVerticalScrollbarSmall(dc, _viewModel.pageCount, _viewModel.pageCursor);

            // stop deviation
            if (_viewModel.canNavigateToDeviation()) {
                Graphite.setColor(dc, AppColors.WARNING);
                WidgetUtil.drawTopPageArrow(dc);
                Graphite.resetColor(dc);
            }
        }

        // error/message
        else {
            // info
            WidgetUtil.drawDialog(dc, response == null
                ? rez(Rez.Strings.msg_i_departures_requesting)
                : (response instanceof ResponseError ? response.getTitle() : response));

            if (response instanceof ResponseError) {
                // banner
                if (!response.hasConnection()) {
                    WidgetUtil.drawExclamationBanner(dc);
                }

                // retry
                if (response.isUserRefreshable()) {
                    WidgetUtil.drawActionFooter(dc, rez(Rez.Strings.lbl_list_retry));
                }
            }
        }
    }

    hidden function _drawHeader(dc as Graphics.Dc, stop) {
        // 19 is font height for XTINY on fr745.
        // set y to half and justify to vcenter for the title to
        // look alright even on devices with different font size for XTINY.
        var y = px(23) + px(19) / 2;

        Graphite.setColor(dc, AppColors.TEXT_SECONDARY);
        dc.drawText(Graphite.getCenterX(dc), y, Graphics.FONT_XTINY, stop.name,
            Graphics.TEXT_JUSTIFY_CENTER|Graphics.TEXT_JUSTIFY_VCENTER);
    }

    hidden function _drawFooter(dc as Graphics.Dc, stop, noDetails) {
        var hFooter = px(42);
        var h = dc.getHeight();

        // background
        WidgetUtil.drawFooter(dc, hFooter, AppColors.PRIMARY, null, null, null);

        if (noDetails) {
            return;
        }

        // clock time

        // calc pos to align with page number
        var arrowEdgeOffset = px(4);
        var arrowHeight = px(8);
        var arrowTextOffset = px(8);

        var font = Graphics.FONT_TINY;
        var y = h - arrowEdgeOffset - arrowHeight - arrowTextOffset - dc.getFontHeight(font);
        var cx = Graphite.getCenterX(dc);

        // make sure the text is fully within the footer.
        y = MathUtil.max(y, h - hFooter);

        var info = Time.Gregorian.info(Time.now(), Time.FORMAT_SHORT);
        var text = info.hour.format("%02d") + ":" + info.min.format("%02d");

        dc.setColor(AppColors.ON_PRIMARY, AppColors.PRIMARY);
        dc.drawText(cx, y, font, text, Graphics.TEXT_JUSTIFY_CENTER);

        // progress bar

        if (DeparturesService.isRequesting || stop.getResponse() == null) {
            var hProgressBar = px(3);
            var yProgressBar = h - hFooter - hProgressBar;
            var progress = MathUtil.recursiveShare(0.33f, 0, stop.getFailedRequestCount());

            WidgetUtil.drawProgressBar(dc, yProgressBar, hProgressBar, progress,
                AppColors.PRIMARY_LT, AppColors.ON_PRIMARY_TERTIARY);
        }

        // mode letter

        var bay = _viewModel.currentBay;
        if (bay == null) {
            return;
        }
        bay = _shortenBay(bay);

        var xBay = cx + px(48);
        var yBay = y - px(7);
        var fontBay = Graphics.FONT_TINY;
        var fh = dc.getFontHeight(fontBay);
        var r = Math.ceil(fh / 2f);

        Graphite.setColor(dc, AppColors.BACKGROUND_INVERTED);
        dc.fillCircle(xBay, yBay + r, r + 2);

        dc.setColor(AppColors.PRIMARY_DK, AppColors.BACKGROUND_INVERTED);
        dc.drawText(xBay, yBay, fontBay, bay, Graphics.TEXT_JUSTIFY_CENTER);
    }

    // Gleis 1 (U) => 1
    hidden function _shortenBay(bay) {
        var prefixes = ["Gleis ", "Bstg. "];
        var suffixes = [" (U)"];
        for (var i = 0; i < prefixes.size(); i++) {
            if (bay.substring(0, prefixes[i].length()).equals(prefixes[i])) {
                bay = bay.substring(prefixes[i].length(), null);
                break; // assume that there's only one prefix
            }
        }
        for (var i = 0; i < suffixes.size(); i++) {
            if (bay.substring(-suffixes[i].length(), null).equals(suffixes[i])) {
                bay = bay.substring(0, -suffixes[i].length());
                break; // assume that there's only one suffix
            }
        }
        return bay;
    }

    hidden function _drawDepartures(dc as Graphics.Dc, pageDepartures as Array<Departure>) {
        var font = Graphics.FONT_TINY;
        var xOffset = px(10);
        var yOffset = px(68);

        var linePad = 3;
        var wdtSpace = dc.getTextWidthInPixels(" ", font);
        var dimTime = dc.getTextDimensions("00:00", font);
        var hgtLine = dimTime[1] + 1; // + 1 necessary due to text rendering, somehow

        var h = dc.getHeight() - yOffset * 2;
        var lineHeightPx = h / (StopDetailViewModel.DEPARTURES_PER_PAGE - 1);

        for (var d = 0; d < StopDetailViewModel.DEPARTURES_PER_PAGE && d < pageDepartures.size(); d++) {
            var departure = pageDepartures[d];
            var line = departure.line();

            var y = yOffset + d * lineHeightPx;
            var xTime = xOffset;
            var xLine = xTime + wdtSpace + dimTime[0];
            var wdtLine = dc.getTextWidthInPixels(line, font) + 2*linePad;
            if ((wdtLine + 2*linePad) < hgtLine) {
                // make at least a square
                wdtLine = hgtLine - 2*linePad;
            }
            var xText = xLine + wdtSpace + wdtLine;

            // draw time
            if (departure.isRealTime) {
                Graphite.setColor(dc, AppColors.DEPARTURE_REALTIME);
            } else {
                Graphite.setColor(dc, AppColors.TEXT_PRIMARY);
            }
            dc.drawText(xTime + dimTime[0], y, font, departure.displayTime(), Graphics.TEXT_JUSTIFY_RIGHT|Graphics.TEXT_JUSTIFY_VCENTER);

            // draw line background
            var lineColor = departure.getLineColor();
            dc.setColor(lineColor[1], lineColor[1]);
            if (line.substring(0, 1).equals("S")) {
                dc.fillRoundedRectangle(xLine, y - dimTime[1]/2, wdtLine+2*linePad, hgtLine, 2*linePad);
            } else {
                dc.fillRectangle(xLine, y - dimTime[1]/2, wdtLine+2*linePad, hgtLine);
            }
            // draw line name
            dc.setColor(lineColor[0], lineColor[1]);
            dc.drawText(xLine + wdtLine/2 + linePad, y, font, line, Graphics.TEXT_JUSTIFY_CENTER|Graphics.TEXT_JUSTIFY_VCENTER);

            // highlight selected departure
            var isSelected = _viewModel.isDepartureState && _viewModel.departureCursor == d;

            // draw text
            var textColor = isSelected ? AppColors.DEPARTURE_SELECTED : departure.getTextColor();
            Graphite.setColor(dc, textColor);
            dc.drawText(xText, y, font, departure.destination(), Graphics.TEXT_JUSTIFY_LEFT|Graphics.TEXT_JUSTIFY_VCENTER);

            // strikethrough
            if (departure.cancelled) {
                Graphite.strokeRectangle(dc, xText, y, dc.getWidth() - xText, px(1), px(2), textColor, AppColors.BACKGROUND);
            }
        }
    }

}
