// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{IsRenderNode, RenderNode, RenderNodeType};
use glib::translate::*;

define_render_node!(
    RepeatNode,
    ffi::GskRepeatNode,
    ffi::gsk_repeat_node_get_type,
    RenderNodeType::RepeatNode
);

impl RepeatNode {
    #[doc(alias = "gsk_repeat_node_new")]
    pub fn new<P: IsRenderNode>(
        bounds: &graphene::Rect,
        child: &P,
        child_bounds: Option<&graphene::Rect>,
    ) -> Self {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gsk_repeat_node_new(
                bounds.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                child_bounds.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_repeat_node_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<RenderNode> {
        unsafe { from_glib_none(ffi::gsk_repeat_node_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_repeat_node_get_child_bounds")]
    #[doc(alias = "get_child_bounds")]
    pub fn child_bounds(&self) -> Option<graphene::Rect> {
        unsafe { from_glib_none(ffi::gsk_repeat_node_get_child_bounds(self.to_glib_none().0)) }
    }
}
