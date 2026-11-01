// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ComponentTransferNode, RenderNodeType};

define_render_node!(
    ComponentTransferNode,
    crate::ffi::GskComponentTransferNode,
    RenderNodeType::ComponentTransferNode
);

impl ComponentTransferNode {
    #[cfg(feature = "v4_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_26")))]
    #[doc(alias = "gsk_component_transfer_node_get_snap")]
    #[doc(alias = "get_snap")]
    pub fn snap(&self) -> crate::RectSnap {
        unsafe {
            glib::translate::from_glib(crate::ffi::gsk_component_transfer_node_get_snap(
                glib::translate::ToGlibPtr::to_glib_none(self).0,
            ))
        }
    }
}

impl std::fmt::Debug for ComponentTransferNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ComponentTransferNode")
            .field("child", &self.child())
            .finish()
    }
}
