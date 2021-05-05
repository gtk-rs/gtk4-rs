// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ColorStop, RenderNodeType};
use glib::translate::*;
use graphene::{Point, Rect};

define_render_node!(
    RepeatingLinearGradientNode,
    ffi::GskRepeatingLinearGradientNode,
    ffi::gsk_repeating_linear_gradient_node_get_type,
    RenderNodeType::RepeatingLinearGradientNode
);

impl RepeatingLinearGradientNode {
    #[doc(alias = "gsk_repeating_linear_gradient_node_new")]
    pub fn new(bounds: &Rect, start: &Point, end: &Point, color_stops: &[ColorStop]) -> Self {
        assert_initialized_main_thread!();
        let n_color_stops = color_stops.len() as usize;
        unsafe {
            from_glib_full(ffi::gsk_repeating_linear_gradient_node_new(
                bounds.to_glib_none().0,
                start.to_glib_none().0,
                end.to_glib_none().0,
                color_stops.to_glib_none().0,
                n_color_stops,
            ))
        }
    }
}
