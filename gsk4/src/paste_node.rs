// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{PasteNode, RenderNodeType};

define_render_node!(
    PasteNode,
    crate::ffi::GskPasteNode,
    RenderNodeType::PasteNode
);

impl PasteNode {
    #[cfg(feature = "v4_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_24")))]
    #[doc(alias = "gsk_paste_node_get_snap")]
    #[doc(alias = "get_snap")]
    pub fn snap(&self) -> crate::RectSnap {
        unsafe {
            glib::translate::from_glib(crate::ffi::gsk_paste_node_get_snap(
                glib::translate::ToGlibPtr::to_glib_none(self).0,
            ))
        }
    }
}

impl std::fmt::Debug for PasteNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PasteNode")
            .field("depth", &self.depth())
            .finish()
    }
}
