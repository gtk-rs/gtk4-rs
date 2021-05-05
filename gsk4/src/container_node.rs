// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNode, RenderNodeType};
use glib::translate::*;

define_render_node!(
    ContainerNode,
    ffi::GskContainerNode,
    ffi::gsk_container_node_get_type,
    RenderNodeType::ContainerNode
);

impl ContainerNode {
    #[doc(alias = "gsk_container_node_new")]
    pub fn new(children: &[RenderNode]) -> Self {
        assert_initialized_main_thread!();
        let n_children = children.len() as u32;
        unsafe {
            from_glib_full(ffi::gsk_container_node_new(
                children.to_glib_none().0,
                n_children,
            ))
        }
    }

    #[doc(alias = "gsk_container_node_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self, idx: u32) -> Option<RenderNode> {
        unsafe {
            from_glib_none(ffi::gsk_container_node_get_child(
                self.to_glib_none().0,
                idx,
            ))
        }
    }

    #[doc(alias = "gsk_container_node_get_n_children")]
    #[doc(alias = "get_n_children")]
    pub fn n_children(&self) -> u32 {
        unsafe { ffi::gsk_container_node_get_n_children(self.to_glib_none().0) }
    }
}
