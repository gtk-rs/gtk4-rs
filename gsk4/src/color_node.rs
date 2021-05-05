// Take a look at the license at the top of the repository in the LICENSE file.

use crate::RenderNodeType;
use glib::translate::*;

define_render_node!(
    ColorNode,
    ffi::GskColorNode,
    ffi::gsk_color_node_get_type,
    RenderNodeType::ColorNode
);

impl ColorNode {
    #[doc(alias = "gsk_color_node_new")]
    pub fn new(rgba: &gdk::RGBA, bounds: &graphene::Rect) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gsk_color_node_new(
                rgba.to_glib_none().0,
                bounds.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_color_node_get_color")]
    #[doc(alias = "get_color")]
    pub fn color(&self) -> Option<gdk::RGBA> {
        unsafe { from_glib_none(ffi::gsk_color_node_get_color(self.to_glib_none().0)) }
    }
}
