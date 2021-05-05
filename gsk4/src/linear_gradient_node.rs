// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ColorStop, RenderNodeType};
use glib::translate::*;
use graphene::{Point, Rect};
use std::mem;

define_render_node!(
    LinearGradientNode,
    ffi::GskLinearGradientNode,
    ffi::gsk_linear_gradient_node_get_type,
    RenderNodeType::LinearGradientNode
);

impl LinearGradientNode {
    #[doc(alias = "gsk_linear_gradient_node_new")]
    pub fn new(bounds: &Rect, start: &Point, end: &Point, color_stops: &[ColorStop]) -> Self {
        assert_initialized_main_thread!();
        let n_color_stops = color_stops.len() as usize;
        unsafe {
            from_glib_full(ffi::gsk_linear_gradient_node_new(
                bounds.to_glib_none().0,
                start.to_glib_none().0,
                end.to_glib_none().0,
                color_stops.to_glib_none().0,
                n_color_stops,
            ))
        }
    }

    #[doc(alias = "gsk_linear_gradient_node_get_color_stops")]
    #[doc(alias = "get_color_stops")]
    pub fn color_stops(&self) -> Vec<ColorStop> {
        unsafe {
            let mut n_stops = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_none_num(
                ffi::gsk_linear_gradient_node_get_color_stops(
                    self.to_glib_none().0,
                    n_stops.as_mut_ptr(),
                ),
                n_stops.assume_init() as usize,
            );
            ret
        }
    }

    #[doc(alias = "gsk_linear_gradient_node_get_end")]
    #[doc(alias = "get_end")]
    pub fn end(&self) -> graphene::Point {
        unsafe { from_glib_none(ffi::gsk_linear_gradient_node_get_end(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_linear_gradient_node_get_n_color_stops")]
    #[doc(alias = "get_n_color_stops")]
    pub fn n_color_stops(&self) -> usize {
        unsafe { ffi::gsk_linear_gradient_node_get_n_color_stops(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_linear_gradient_node_get_start")]
    #[doc(alias = "get_start")]
    pub fn start(&self) -> graphene::Point {
        unsafe {
            from_glib_none(ffi::gsk_linear_gradient_node_get_start(
                self.to_glib_none().0,
            ))
        }
    }
}
