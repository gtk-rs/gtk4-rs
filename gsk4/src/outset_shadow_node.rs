// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{OutsetShadowNode, RenderNodeType};

define_render_node!(
    OutsetShadowNode,
    crate::ffi::GskOutsetShadowNode,
    RenderNodeType::OutsetShadowNode
);

impl OutsetShadowNode {
    #[cfg(feature = "v4_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_24")))]
    #[doc(alias = "gsk_outset_shadow_node_get_snap")]
    #[doc(alias = "get_snap")]
    pub fn snap(&self) -> crate::RectSnap {
        unsafe {
            glib::translate::from_glib(crate::ffi::gsk_outset_shadow_node_get_snap(
                glib::translate::ToGlibPtr::to_glib_none(self).0,
            ))
        }
    }
}

impl std::fmt::Debug for OutsetShadowNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("OutsetShadowNode")
            .field("blur_radius", &self.blur_radius())
            .field("color", &self.color())
            .field("dx", &self.dx())
            .field("dy", &self.dy())
            .field("outline", &self.outline())
            .field("spread", &self.spread())
            .finish()
    }
}
