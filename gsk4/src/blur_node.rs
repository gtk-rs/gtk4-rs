// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{IsRenderNode, RenderNode, RenderNodeType};
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct BlurNode(Shared<ffi::GskBlurNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

define_render_node!(
    BlurNode,
    ffi::GskBlurNode,
    ffi::gsk_blur_node_get_type,
    RenderNodeType::BlurNode
);

impl BlurNode {
    #[doc(alias = "gsk_blur_node_new")]
    pub fn new<P: IsRenderNode>(child: &P, radius: f32) -> Self {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gsk_blur_node_new(
                child.as_ref().to_glib_none().0,
                radius,
            ))
        }
    }

    #[doc(alias = "gsk_blur_node_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<RenderNode> {
        unsafe { from_glib_none(ffi::gsk_blur_node_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_blur_node_get_radius")]
    #[doc(alias = "get_radius")]
    pub fn radius(&self) -> f32 {
        unsafe { ffi::gsk_blur_node_get_radius(self.to_glib_none().0) }
    }
}
