// Take a look at the license at the top of the repository in the LICENSE file.

use crate::RenderNodeType;
use glib::translate::*;

define_render_node!(
    CairoNode,
    ffi::GskCairoNode,
    ffi::gsk_cairo_node_get_type,
    RenderNodeType::CairoNode
);

impl CairoNode {
    #[doc(alias = "gsk_cairo_node_new")]
    pub fn new(bounds: &graphene::Rect) -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gsk_cairo_node_new(bounds.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_cairo_node_get_draw_context")]
    #[doc(alias = "get_draw_context")]
    pub fn draw_context(&self) -> Option<cairo::Context> {
        unsafe { from_glib_full(ffi::gsk_cairo_node_get_draw_context(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_cairo_node_get_surface")]
    #[doc(alias = "get_surface")]
    pub fn surface(&self) -> Option<cairo::Surface> {
        unsafe { from_glib_none(ffi::gsk_cairo_node_get_surface(self.to_glib_none().0)) }
    }
}
