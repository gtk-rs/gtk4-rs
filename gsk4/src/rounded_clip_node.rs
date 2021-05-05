// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{IsRenderNode, RenderNode, RenderNodeType, RoundedRect};
use glib::translate::*;

define_render_node!(
    RoundedClipNode,
    ffi::GskRoundedClipNode,
    ffi::gsk_rounded_clip_node_get_type,
    RenderNodeType::RoundedClipNode
);

impl RoundedClipNode {
    #[doc(alias = "gsk_rounded_clip_node_new")]
    pub fn new<P: IsRenderNode>(child: &P, clip: &RoundedRect) -> Self {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gsk_rounded_clip_node_new(
                child.as_ref().to_glib_none().0,
                clip.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_rounded_clip_node_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<RenderNode> {
        unsafe { from_glib_none(ffi::gsk_rounded_clip_node_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_rounded_clip_node_get_clip")]
    #[doc(alias = "get_clip")]
    pub fn clip(&self) -> Option<RoundedRect> {
        unsafe { from_glib_none(ffi::gsk_rounded_clip_node_get_clip(self.to_glib_none().0)) }
    }
}
