// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNodeType, RepeatNode};

define_render_node!(
    RepeatNode,
    crate::ffi::GskRepeatNode,
    RenderNodeType::RepeatNode
);

impl RepeatNode {
    #[cfg(feature = "v4_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_24")))]
    #[doc(alias = "gsk_repeat_node_get_child_snap")]
    #[doc(alias = "get_child_snap")]
    pub fn child_snap(&self) -> crate::RectSnap {
        unsafe {
            glib::translate::from_glib(crate::ffi::gsk_repeat_node_get_child_snap(
                glib::translate::ToGlibPtr::to_glib_none(self).0,
            ))
        }
    }

    #[cfg(feature = "v4_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_24")))]
    #[doc(alias = "gsk_repeat_node_get_snap")]
    #[doc(alias = "get_snap")]
    pub fn snap(&self) -> crate::RectSnap {
        unsafe {
            glib::translate::from_glib(crate::ffi::gsk_repeat_node_get_snap(
                glib::translate::ToGlibPtr::to_glib_none(self).0,
            ))
        }
    }
}

impl std::fmt::Debug for RepeatNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RepeatNode")
            .field("child_bounds", &self.child_bounds())
            .field("child", &self.child())
            .finish()
    }
}
