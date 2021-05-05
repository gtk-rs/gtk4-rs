// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNodeType, RoundedRect};
use glib::translate::*;

define_render_node!(
    BorderNode,
    ffi::GskBorderNode,
    ffi::gsk_border_node_get_type,
    RenderNodeType::BorderNode
);

impl BorderNode {
    #[doc(alias = "gsk_border_node_new")]
    pub fn new(
        outline: &RoundedRect,
        border_width: &[f32; 4],
        border_color: &[gdk::RGBA; 4],
    ) -> Self {
        unsafe {
            from_glib_full(ffi::gsk_border_node_new(
                outline.to_glib_none().0,
                border_width.as_ptr() as *const [f32; 4],
                border_color.as_ptr() as *const [gdk::ffi::GdkRGBA; 4],
            ))
        }
    }

    #[doc(alias = "gsk_border_node_get_colors")]
    #[doc(alias = "get_colors")]
    pub fn colors(&self) -> &[gdk::RGBA; 4] {
        unsafe {
            &*(ffi::gsk_border_node_get_colors(self.to_glib_none().0) as *const [gdk::RGBA; 4])
        }
    }

    #[doc(alias = "gsk_border_node_get_outline")]
    #[doc(alias = "get_outline")]
    pub fn outline(&self) -> RoundedRect {
        unsafe { from_glib_none(ffi::gsk_border_node_get_outline(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_border_node_get_widths")]
    #[doc(alias = "get_widths")]
    pub fn widths(&self) -> &[f32; 4] {
        unsafe { &*ffi::gsk_border_node_get_widths(self.to_glib_none().0) }
    }
}
