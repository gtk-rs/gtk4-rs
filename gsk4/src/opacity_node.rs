// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{IsRenderNode, RenderNode, RenderNodeType};
use glib::translate::*;

define_render_node!(
    OpacityNode,
    ffi::GskOpacityNode,
    ffi::gsk_opacity_node_get_type,
    RenderNodeType::OpacityNode
);

impl OpacityNode {
    #[doc(alias = "gsk_opacity_node_new")]
    pub fn new<P: IsRenderNode>(child: &P, opacity: f32) -> Self {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gsk_opacity_node_new(
                child.as_ref().to_glib_none().0,
                opacity,
            ))
        }
    }

    #[doc(alias = "gsk_opacity_node_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<RenderNode> {
        unsafe { from_glib_none(ffi::gsk_opacity_node_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_opacity_node_get_opacity")]
    #[doc(alias = "get_opacity")]
    pub fn opacity(&self) -> f32 {
        unsafe { ffi::gsk_opacity_node_get_opacity(self.to_glib_none().0) }
    }
}
