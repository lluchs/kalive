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

//! Draw more complex but common widgets.
module WidgetUtil {

    // directions
    const _DIR_LEFT = 0;
    const _DIR_RIGHT = 1;
    const _DIR_UP = 2;
    const _DIR_DOWN = 3;

    // button positions
    const _BTN_START_DEG = 30;
    const _BTN_LIGHT_DEG = 150;
    const _BTN_UP_DEG = 180;
    const _BTN_DOWN_DEG = 210;
    const _BTN_BACK_DEG = 330;

    // text

    function drawDialog(dc as Graphics.Dc, text) {
        var fonts = [ Graphics.FONT_SMALL, Graphics.FONT_TINY ];
        var fh = Graphics.getFontHeight(fonts[0]);
        var w = dc.getWidth() - px(12);
        var h = dc.getHeight() / 2;

        Graphite.resetColor(dc);
        Graphite.drawTextArea(dc, Graphite.getCenterX(dc), Graphite.getCenterY(dc) - fh / 2,
            w, h, fonts, text, Graphics.TEXT_JUSTIFY_CENTER, AppColors.TEXT_PRIMARY);
    }

    function drawPreviewTitle(dc, text, rezId, smallIcon) {
        var yText = px(45);

        if (rezId != null) {
            var yIcon;

            if (smallIcon) {
                yIcon = px(25);
                yText = px(63);
            }
            else {
                yIcon = px(30);
                yText = px(68);
            }

            RezUtil.drawBitmap(dc, Graphite.getCenterX(dc), yIcon, rezId);
        }

        if (text != null && !text.equals("")) {
            dc.drawText(Graphite.getCenterX(dc), yText, Graphics.FONT_SMALL, text,
                Graphics.TEXT_JUSTIFY_CENTER|Graphics.TEXT_JUSTIFY_VCENTER);
        }
    }

    // header/footer

    function drawExclamationBanner(dc as Graphics.Dc) {
        drawHeader(dc, px(30), AppColors.ERROR, AppColors.BACKGROUND, "!", AppColors.TEXT_PRIMARY);
    }

    function drawActionFooter(dc as Graphics.Dc, message) {
        drawFooter(dc, px(42), AppColors.BACKGROUND_INVERTED, AppColors.BACKGROUND, message, AppColors.TEXT_INVERTED);

        Graphite.setColor(dc, AppColors.TEXT_INVERTED);
        drawBottomPageArrow(dc);
        Graphite.resetColor(dc);
    }

    function drawHeader(dc as Graphics.Dc, height, color, strokeColor, text, textColor) {
        Graphite.setColor(dc, color);
        dc.fillRectangle(0, 0, dc.getWidth(), height);

        if (strokeColor != null) {
            var strokeWidth = px(1);
            var y = height - strokeWidth;

            dc.setPenWidth(strokeWidth);
            Graphite.setColor(dc, strokeColor);
            dc.drawLine(0, y, dc.getWidth(), y);
            Graphite.resetPenWidth(dc);
        }

        if (text != null && !text.equals("")) {
            dc.setColor(textColor, color);
            dc.drawText(dc.getWidth() / 2, height / 2, Graphics.FONT_TINY, text,
                Graphics.TEXT_JUSTIFY_CENTER|Graphics.TEXT_JUSTIFY_VCENTER);
        }
    }

    function drawFooter(dc as Graphics.Dc, height, color, strokeColor, text, textColor) {
        Graphite.setColor(dc, color);
        dc.fillRectangle(0, dc.getHeight() - height, dc.getWidth(), height);

        if (strokeColor != null) {
            var strokeWidth = px(1);
            var y = dc.getHeight() - height - strokeWidth;

            dc.setPenWidth(strokeWidth);
            Graphite.setColor(dc, strokeColor);
            dc.drawLine(0, y, dc.getWidth(), y);
            Graphite.resetPenWidth(dc);
        }

        if (text != null && !text.equals("")) {
            var font = Graphics.FONT_TINY;
            // balance optically with 1/4 of font height
            var y = dc.getHeight() - height / 2 - dc.getFontHeight(font) / 4;

            dc.setColor(textColor, color);
            dc.drawText(dc.getWidth() / 2, y, font, text,
                Graphics.TEXT_JUSTIFY_CENTER|Graphics.TEXT_JUSTIFY_VCENTER);
        }
    }

    (:round)
    function drawProgressBar(dc as Graphics.Dc, y, h, progress, activeColor, inactiveColor) {
        var r = Graphite.getRadius(dc);
        var start = MathUtil.minX(y, r) - h;
        var end = MathUtil.maxX(y, r) + h;
        var w = end - start;
        var middle = Math.round(w * progress);

        Graphite.setColor(dc, activeColor);
        dc.fillRectangle(start, y, middle, h);

        if (inactiveColor != null) {
            Graphite.setColor(dc, inactiveColor);
            dc.fillRectangle(start + middle, y, w - middle, h);
        }

        Graphite.resetColor(dc);
    }

    (:rectangle)
    function drawProgressBar(dc as Graphics.Dc, y, h, progress, activeColor, inactiveColor) {
        var start = 0;
        var end = dc.getWidth();
        var w = end - start;
        var middle = w * progress;

        Graphite.setColor(dc, activeColor);
        dc.fillRectangle(start, y, start + middle, h);

        if (inactiveColor != null) {
            Graphite.setColor(dc, inactiveColor);
            dc.fillRectangle(start + middle, y, end, h);
        }

        Graphite.resetColor(dc);
    }

    // start indicator

    (:round)
    function drawStartIndicatorWithBitmap(dc as Graphics.Dc, rezId) {
        var r = Graphite.getRadius(dc) - px(23);
        var pos = MathUtil.polarPos(r, MathUtil.rad(30), Graphite.getCenterX(dc), Graphite.getCenterY(dc));

        RezUtil.drawBitmap(dc, pos[0], pos[1], rezId);
        drawStartIndicator(dc);
    }

    (:rectangle)
    function drawStartIndicatorWithBitmap(dc as Graphics.Dc, rezId) {
        var x = dc.getWidth() - px(23);
        var y = 0.5 * dc.getHeight(); // sin(30) = 0.5

        RezUtil.drawBitmap(dc, x, y, rezId);
        drawStartIndicator(dc);
    }

    (:round)
    function drawStartIndicator(dc as Graphics.Dc) {
        var offset = px(5);
        var width = px(4);
        var strokeWidth = px(1);

        Graphite.strokeArcCentered(dc, offset, width, strokeWidth, 20, 40, AppColors.TEXT_PRIMARY, AppColors.BACKGROUND);
    }

    (:rectangle)
    function drawStartIndicator(dc as Graphics.Dc) {
        var offset = px(5);
        var width = px(4);
        var strokeWidth = px(1);

        var x = dc.getWidth() - offset;
        var y = 0.34 * dc.getHeight(); // sin(20) = 0.34
        var yBottom = 0.64 * dc.getHeight(); // sin(40) = 0.64
        var height = yBottom - y;

        Graphite.strokeRectangle(dc, x, y, width, height, strokeWidth, AppColors.TEXT_PRIMARY, AppColors.BACKGROUND);
    }

    // scrollbar

    function drawVerticalScrollbarSmall(dc as Graphics.Dc, pageCount, index) {
        _drawVerticalScrollbar(dc, 50, pageCount, index, index + 1);
    }

    (:round)
    function _drawVerticalScrollbar(dc, sizeDeg, itemCount, startIndex, endIndex) {
        if (itemCount <= 1) {
            return;
        }

        var edgeOffset = px(2);
        var startDeg = 180 - sizeDeg / 2;
        var endDeg = 180 + sizeDeg / 2;

        var railWidth = px(1);
        var outlineWidth = px(3);

        // rail
        Graphite.strokeArcCentered(dc, edgeOffset, railWidth, outlineWidth, startDeg, endDeg,
            AppColors.TEXT_TERTIARY, AppColors.BACKGROUND);

        var barDeltaDeg = (endDeg - startDeg) * (endIndex - startIndex) / itemCount.toFloat();
        var barStartDeg = startDeg + (endDeg - startDeg) * startIndex / itemCount.toFloat();
        var barEndDeg = barStartDeg + barDeltaDeg;

        // bar
        Graphite.resetColor(dc);
        dc.setPenWidth(px(3));
        Graphite.drawArcCentered(dc, edgeOffset, barStartDeg, barEndDeg);

        Graphite.resetPenWidth(dc);
    }

    (:rectangle)
    function _drawVerticalScrollbar(dc as Graphics.Dc, sizeDeg, itemCount, startIndex, endIndex) {
        if (itemCount <= 1) {
            return;
        }

        var x = px(3);
        var startDeg = 180 - sizeDeg / 2;
        var endDeg = 180 + sizeDeg / 2;
        var yStart = Graphite.degToY(dc, startDeg);
        var yEnd = Graphite.degToY(dc, endDeg);
        var height = MathUtil.abs(yEnd - yStart);

        var railWidth = px(1);
        var outlineWidth = px(3);

        // rail
        Graphite.strokeRectangleCentered(dc, x, Graphite.getCenterY(dc), railWidth, height, outlineWidth,
            AppColors.TEXT_TERTIARY, AppColors.BACKGROUND);

        var barHeight = height * (endIndex - startIndex) / itemCount.toFloat();
        var barStartY = yStart + height * startIndex / itemCount.toFloat();

        // bar
        Graphite.resetColor(dc);
        dc.setPenWidth(px(3));
        Graphite.fillRectangleCentered(dc, x, barStartY + barHeight / 2, px(3), barHeight);

        Graphite.resetPenWidth(dc);
    }

    // page indicator

    (:round)
    function drawHorizontalPageIndicator(dc as Graphics.Dc, pageCount, index) {
        if (pageCount <= 1) {
            return;
        }

        var lengthDeg = 3; // length in degrees of one indicator
        var deltaDeg = lengthDeg + 2;
        var centerDeg = _BTN_START_DEG;
        var maxDeg = centerDeg + deltaDeg * (pageCount - 1) / 2f;
        var minDeg = maxDeg - pageCount * deltaDeg;
        var edgeOffset = px(5);
        var stroke = px(4);

        var outlineWidth = px(3);
        var outlineWidthDeg = Math.ceil(Graphite.pxToDeg(outlineWidth, Graphite.getRadius(dc) - edgeOffset));
        var bgStroke = stroke + 2 * outlineWidth;
        var bgMinDeg = minDeg + deltaDeg - outlineWidthDeg;
        var bgMaxDeg = maxDeg + lengthDeg + outlineWidthDeg;

        // bg outline
        Graphite.setColor(dc, AppColors.BACKGROUND);
        dc.setPenWidth(bgStroke);
        Graphite.drawArcCentered(dc, edgeOffset, bgMinDeg, bgMaxDeg);

        // indicator

        dc.setPenWidth(stroke);

        for (var i = 0; i < pageCount; i++) {
            var startDeg = maxDeg - i * deltaDeg;
            var endDeg = startDeg + lengthDeg;

            if (i == index) {
                Graphite.resetColor(dc);
            }
            else {
                Graphite.setColor(dc, AppColors.TEXT_TERTIARY);
            }

            Graphite.drawArcCentered(dc, edgeOffset, startDeg, endDeg);
        }

        Graphite.resetPenWidth(dc);
    }

    (:rectangle)
    function drawHorizontalPageIndicator(dc as Graphics.Dc, pageCount, index) {
        if (pageCount <= 1) {
            return;
        }

        var length = px(6); // length of one indicator
        var delta = length + px(3);
        var center = Graphite.degToY(dc, _BTN_START_DEG);
        var max = center + delta * (pageCount - 1) / 2f;
        var min = max - pageCount * delta;
        var edgeOffset = px(5);
        var stroke = px(4);

        var outlineWidth = px(3);
        var bgStroke = stroke + px(2) * outlineWidth;

        // bg outline
        Graphite.setColor(dc, AppColors.BACKGROUND);
        dc.setPenWidth(bgStroke);
        Graphite.fillRectangleCentered(dc, dc.getWidth() - edgeOffset, center - 2 * outlineWidth, stroke + 2 * outlineWidth, max - min + 2 * outlineWidth);

        // indicator
        for (var i = 0; i < pageCount; i++) {
            var start = min + i * delta;
            var end = start + length;

            var y = (end + start) / 2;
            var height = MathUtil.abs(end - start);

            if (i == index) {
                Graphite.resetColor(dc);
            }
            else {
                Graphite.setColor(dc, AppColors.TEXT_TERTIARY);
            }

            Graphite.fillRectangleCentered(dc, dc.getWidth() - edgeOffset, y, stroke, height);
        }
    }

    // page arrow

    function drawVerticalPageArrows(dc as Graphics.Dc, pageCount, index, topColor, bottomColor) {
        if (pageCount <= 1) {
            return;
        }

        if (index != 0) {
            Graphite.setColor(dc, topColor);
            drawTopPageArrow(dc);
        }
        if (index != pageCount - 1) {
            Graphite.setColor(dc, bottomColor);
            drawBottomPageArrow(dc);
        }

        Graphite.resetColor(dc);
    }

    function drawTopPageArrow(dc as Graphics.Dc) {
        _drawPageArrow(dc, [ Graphite.getCenterX(dc), px(4) ], _DIR_UP);
    }

    function drawBottomPageArrow(dc as Graphics.Dc) {
        _drawPageArrow(dc, [ Graphite.getCenterX(dc), dc.getHeight() - px(4) ], _DIR_DOWN);
    }

    function drawUpArrow(dc as Graphics.Dc, bottomTo) {
        _drawPageArrow(dc, [ Graphite.getCenterX(dc), bottomTo - px(4 + 8) ], _DIR_UP);
    }

    function drawDownArrow(dc as Graphics.Dc, bottomTo) {
        _drawPageArrow(dc, [ Graphite.getCenterX(dc), bottomTo - px(4) ], _DIR_DOWN);
    }

    function _drawPageArrow(dc as Graphics.Dc, point1, direction) {
        var width = px(8);
        var height = px(8);

        var point2;
        var point3;

        if (direction ==_DIR_LEFT) {
            point2 = ArrUtil.add(point1, [ width, height ]);
            point3 = ArrUtil.add(point1, [ width, -height ]);
        }
        else if (direction ==_DIR_RIGHT) {
            point2 = ArrUtil.add(point1, [ -width, height ]);
            point3 = ArrUtil.add(point1, [ -width, -height ]);
        }
        else if (direction ==_DIR_UP) {
            point2 = ArrUtil.add(point1, [ -width, height ]);
            point3 = ArrUtil.add(point1, [ width, height ]);
        }
        else if (direction ==_DIR_DOWN) {
            point2 = ArrUtil.add(point1, [ -width, -height ]);
            point3 = ArrUtil.add(point1, [ width, -height ]);
        }
        else {
            point2 = [ 0, 0 ];
            point3 = [ 0, 0 ];
        }

        dc.fillPolygon([ point1, point2, point3 ]);
    }

    // list

    function drawPanedList(dc as Graphics.Dc, items as Array, paneSize, cursor, paneHints as Array, mainHints as Array, topHint, paneColors as Array, mainColors as Array) {
        var paneHint = paneHints[0];
        var mainHint = mainHints[0];

        var hasPane = paneSize != 0;
        var hasMain = paneSize != items.size();

        var paneStrokeColor = null;

        // pane is empty
        if (!hasPane) {
            paneHint = paneHints[1];
            paneColors = mainColors;
            paneStrokeColor = mainColors[3];
        }

        // main is empty
        if (!hasMain) {
            mainHint = mainHints[1];
        }

        // draw panes + page arrows

        // inside pane
        if (cursor < paneSize) {
            Graphite.fillBackground(dc, paneColors[0]);

            // top header
            if (cursor == 0) {
                drawHeader(dc, px(84), AppColors.BACKGROUND, paneStrokeColor, topHint, AppColors.TEXT_PRIMARY);
            }
            else if (cursor == 1) {
                drawHeader(dc, px(42), mainColors[0], paneStrokeColor, null, null);
                Graphite.setColor(dc, mainColors[3]);
                drawUpArrow(dc, px(42));
            }
            else {
                Graphite.setColor(dc, paneColors[3]);
                drawTopPageArrow(dc);
            }

            // bottom header
            if (cursor == paneSize - 2) {
                drawFooter(dc, px(42), mainColors[0], paneStrokeColor, null, null);
                Graphite.setColor(dc, paneColors[3]);
                drawDownArrow(dc, dc.getHeight() - px(42));
            }
            else if (cursor == paneSize - 1) {
                drawFooter(dc, px(84), mainColors[0], paneStrokeColor, mainHint, mainColors[3]);
                Graphite.setColor(dc, paneColors[3]);
                drawDownArrow(dc, dc.getHeight() - px(84));
            }
            else {
                Graphite.setColor(dc, paneColors[3]);
                drawBottomPageArrow(dc);
            }
        }

        // outside pane
        else {
            Graphite.fillBackground(dc, mainColors[0]);

            // top header
            if (cursor == paneSize) {
                drawHeader(dc, px(84), paneColors[0], paneStrokeColor, paneHint, paneColors[3]);
                Graphite.setColor(dc, paneColors[3]);
                // (app specific) show up arrow even if pane is empty,
                // to indicate navigation to empty page dialog
                drawUpArrow(dc, px(84));
            }
            else if (cursor == paneSize + 1) {
                drawHeader(dc, px(42), paneColors[0], paneStrokeColor, null, null);
                Graphite.setColor(dc, paneColors[3]);
                drawUpArrow(dc, px(42));
            }
            else {
                Graphite.setColor(dc, mainColors[3]);
                drawTopPageArrow(dc);
            }

            // bottom header
            if (hasMain && cursor != items.size() - 1) {
                Graphite.setColor(dc, mainColors[3]);
                drawBottomPageArrow(dc);
            }
        }

        // draw items

        var fontsSelected = [ Graphics.FONT_LARGE, Graphics.FONT_MEDIUM, Graphics.FONT_SMALL, Graphics.FONT_TINY, Graphics.FONT_XTINY ];
        var font = Graphics.FONT_TINY;
        var h = dc.getHeight() - 2 * px(36);
        var lineHeightPx = h / 4;

        var bgColor = cursor >= paneSize ? mainColors[0] : paneColors[0];
        var selectedColor;
        var unselectedColor;

        // only draw 2 items above and 2 below cursor
        var itemOffset = 2;
        var firstItemIndex = MathUtil.max(0, cursor - itemOffset);
        var lastItemIndex = MathUtil.min(items.size(), cursor + itemOffset + 1);

        // only draw one list at a time
        if (cursor < paneSize) {
            lastItemIndex = MathUtil.min(lastItemIndex, paneSize);
            selectedColor = paneColors[1];
            unselectedColor = paneColors[2];
        }
        else {
            firstItemIndex = MathUtil.max(firstItemIndex, paneSize);
            selectedColor = mainColors[1];
            unselectedColor = mainColors[2];
        }

        // draw the items
        for (var i = firstItemIndex; i < lastItemIndex; i++) {
            var item = items[i];

            var justification = Graphics.TEXT_JUSTIFY_CENTER|Graphics.TEXT_JUSTIFY_VCENTER;

            if (i == cursor) {
                var margin = px(4);
                var width = dc.getWidth() - 2 * margin;
                var height = dc.getFontHeight(fontsSelected[0]);

                Graphite.drawTextArea(dc, Graphite.getCenterX(dc), Graphite.getCenterY(dc), width, height,
                    fontsSelected, item, justification, selectedColor);
            }
            else {
                var yText = Graphite.getCenterY(dc) + (i - cursor) * lineHeightPx;

                dc.setColor(unselectedColor, bgColor);
                dc.drawText(Graphite.getCenterX(dc), yText, font, item, justification);
            }
        }
    }

    function drawSideList(dc as Graphics.Dc, items as Array, cursor, blackBg) {
        var colorBg = blackBg ? AppColors.BACKGROUND : AppColors.BACKGROUND_INVERTED;
        var colorSelected = blackBg ? AppColors.TEXT_PRIMARY : AppColors.TEXT_INVERTED;
        var colorUnselected = blackBg ? AppColors.TEXT_SECONDARY: AppColors.TEXT_TERTIARY;

        var h = dc.getHeight();
        var w = dc.getWidth();
        var xBg = px(62);
        var wBorder = px(2);
        var wIndicator = px(4);
        var hIndicator = px(42);

        // bg
        Graphite.setColor(dc, colorBg);
        dc.fillRectangle(xBg, 0, w - xBg, h);

        // border
        Graphite.setColor(dc, colorSelected);
        dc.fillRectangle(xBg - wBorder, 0, wBorder, dc.getHeight());

        // indicator
        dc.fillRectangle(xBg + px(3), h / 2 - hIndicator / 2, wIndicator, hIndicator);

        // draw items

        var fontSelected =  Graphics.FONT_MEDIUM;
        var font = Graphics.FONT_SMALL;
        var lineHeight = (h - 2 * px(20)) / 5;
        var xText = xBg + px(10);

        // only draw 2 items above and 2 below cursor
        var itemOffset = 2;
        var firstItemIndex = MathUtil.max(0, cursor - itemOffset);
        var lastItemIndex = MathUtil.min(items.size(), cursor + itemOffset + 1);

        // draw the items
        for (var i = firstItemIndex; i < lastItemIndex; i++) {
            var item = items[i];

            var justification = Graphics.TEXT_JUSTIFY_LEFT|Graphics.TEXT_JUSTIFY_VCENTER;
            var yText = Graphite.getCenterY(dc) + (i - cursor) * lineHeight;

            if (i == cursor) {
                dc.setColor(colorSelected, colorBg);
                dc.drawText(xText, yText, fontSelected, item, justification);
            }
            else {
                dc.setColor(colorUnselected, colorBg);
                dc.drawText(xText, yText, font, item, justification);
            }
        }
    }

}
