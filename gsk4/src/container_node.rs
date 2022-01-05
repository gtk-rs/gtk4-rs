// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ContainerNode, RenderNode, RenderNodeType};
use glib::translate::*;

define_render_node!(
    ContainerNode,
    ffi::GskContainerNode,
    RenderNodeType::ContainerNode
);

impl ContainerNode {
    #[doc(alias = "gsk_container_node_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self, idx: u32) -> RenderNode {
        assert!(idx < self.n_children());
        unsafe {
            from_glib_none(ffi::gsk_container_node_get_child(
                self.to_glib_none().0,
                idx,
            ))
        }
    }
}

impl std::fmt::Debug for ContainerNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ContainerNode")
            .field("n_children", &self.n_children())
            .finish()
    }
}
