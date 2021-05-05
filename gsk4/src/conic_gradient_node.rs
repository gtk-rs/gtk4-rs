// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ColorStop, RenderNodeType};
use glib::translate::*;
use std::mem;

define_render_node!(
    ConicGradientNode,
    ffi::GskConicGradientNode,
    ffi::gsk_conic_gradient_node_get_type,
    RenderNodeType::ConicGradientNode
);

impl ConicGradientNode {
    #[doc(alias = "gsk_conic_gradient_node_new")]
    pub fn new(
        bounds: &graphene::Rect,
        center: &graphene::Point,
        rotation: f32,
        color_stops: &[ColorStop],
    ) -> Self {
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

    #[doc(alias = "gsk_conic_gradient_node_get_center")]
    #[doc(alias = "get_center")]
    pub fn center(&self) -> graphene::Point {
        unsafe {
            from_glib_none(ffi::gsk_conic_gradient_node_get_center(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_conic_gradient_node_get_color_stops")]
    #[doc(alias = "get_color_stops")]
    pub fn color_stops(&self) -> Vec<ColorStop> {
        unsafe {
            let mut n_stops = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_none_num(
                ffi::gsk_conic_gradient_node_get_color_stops(
                    self.to_glib_none().0,
                    n_stops.as_mut_ptr(),
                ),
                n_stops.assume_init() as usize,
            );
            ret
        }
    }

    #[doc(alias = "gsk_conic_gradient_node_get_n_color_stops")]
    #[doc(alias = "get_n_color_stops")]
    pub fn n_color_stops(&self) -> usize {
        unsafe { ffi::gsk_conic_gradient_node_get_n_color_stops(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_conic_gradient_node_get_rotation")]
    #[doc(alias = "get_rotation")]
    pub fn rotation(&self) -> f32 {
        unsafe { ffi::gsk_conic_gradient_node_get_rotation(self.to_glib_none().0) }
    }
}
