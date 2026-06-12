// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ColorNode, RenderNodeType};

define_render_node!(
    ColorNode,
    crate::ffi::GskColorNode,
    RenderNodeType::ColorNode
);

impl ColorNode {
    #[cfg(feature = "v4_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_24")))]
    #[doc(alias = "gsk_color_node_get_snap")]
    #[doc(alias = "get_snap")]
    pub fn snap(&self) -> crate::RectSnap {
        unsafe {
            glib::translate::from_glib(crate::ffi::gsk_color_node_get_snap(
                glib::translate::ToGlibPtr::to_glib_none(self).0,
            ))
        }
    }
}

impl std::fmt::Debug for ColorNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ColorNode")
            .field("color", &self.color())
            .finish()
    }
}
