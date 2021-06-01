// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{IsRenderNode, RenderNode, RenderNodeType};
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GskClipNode")]
    pub struct ClipNode(Shared<ffi::GskClipNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

define_render_node!(
    ClipNode,
    ffi::GskClipNode,
    ffi::gsk_clip_node_get_type,
    RenderNodeType::ClipNode
);

impl ClipNode {
    #[doc(alias = "gsk_clip_node_new")]
    pub fn new<P: IsRenderNode>(child: &P, clip: &graphene::Rect) -> Self {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gsk_clip_node_new(
                child.as_ref().to_glib_none().0,
                clip.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_clip_node_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<RenderNode> {
        unsafe { from_glib_none(ffi::gsk_clip_node_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_clip_node_get_clip")]
    #[doc(alias = "get_clip")]
    pub fn clip(&self) -> Option<graphene::Rect> {
        unsafe { from_glib_none(ffi::gsk_clip_node_get_clip(self.to_glib_none().0)) }
    }
}
