use crate::{ColorStop, ConicGradientNode};
use glib::translate::*;

impl ConicGradientNode {
    #[doc(alias = "gsk_conic_gradient_node_new")]
    pub fn new(
        bounds: &graphene::Rect,
        center: &graphene::Point,
        rotation: f32,
        color_stops: &[ColorStop],
    ) -> ConicGradientNode {
        assert_initialized_main_thread!();
        let n_color_stops = color_stops.len() as usize;
        unsafe {
            from_glib_full(ffi::gsk_conic_gradient_node_new(
                bounds.to_glib_none().0,
                center.to_glib_none().0,
                rotation,
                color_stops.to_glib_none().0,
                n_color_stops,
            ))
        }
    }
}
