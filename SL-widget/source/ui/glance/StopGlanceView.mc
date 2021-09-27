import Toybox.Lang;

using Toybox.WatchUi;
using Toybox.Graphics;
using Carbon.Graphite as Graphite;
using Carbon.Graphene as Graphene;

(:glance)
class StopGlanceView extends WatchUi.GlanceView {

    private var _model as StopGlanceViewModel;

    //

    function initialize(container as Container) as Void {
        GlanceView.initialize();
        _model = container.stopGlanceViewModel;
    }

    // override GlanceView

    //! Load resources
    function onLayout(dc as Dc) as Void {
        setLayout(Rez.Layouts.glance_layout(dc));
    }

    //! Called when this View is brought to the foreground. Restore
    //! the state of this View and prepare it to be shown. This includes
    //! loading resources into memory.
    function onShow() as Void {
        _model.enableRequests();
    }

    //! Update the view
    function onUpdate(dc as Dc) as Void {
        // Call the parent onUpdate function to redraw the layout
        GlanceView.onUpdate(dc);

        // draw
        dc.setAntiAlias(true);
        _draw(new DcCompat(dc));
    }

    //! Called when this View is removed from the screen. Save the
    //! state of this View here. This includes freeing resources from
    //! memory.
    function onHide() as Void {
        _model.disableRequests();
    }

    // draw

    private function _draw(dcc as DcCompat) as Void {
        dcc.drawGlanceTitle(_model.getStopString());
    }

}
