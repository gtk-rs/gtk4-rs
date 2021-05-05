// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{IsRenderNode, RenderNode, RenderNodeType};
use glib::translate::*;

define_render_node!(
    DebugNode,
    ffi::GskDebugNode,
    ffi::gsk_debug_node_get_type,
    RenderNodeType::DebugNode
);

impl DebugNode {
    #[doc(alias = "gsk_debug_node_new")]
    pub fn new<P: IsRenderNode>(child: &P, message: &str) -> Self {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gsk_debug_node_new(
                child.as_ref().to_glib_none().0,
                message.to_glib_full(),
            ))
        }
    }

    #[doc(alias = "gsk_debug_node_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<RenderNode> {
        unsafe { from_glib_none(ffi::gsk_debug_node_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_debug_node_get_message")]
    #[doc(alias = "get_message")]
    pub fn message(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gsk_debug_node_get_message(self.to_glib_none().0)) }
    }
}
