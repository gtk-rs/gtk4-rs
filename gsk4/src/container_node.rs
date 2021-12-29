// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ContainerNode, RenderNode, RenderNodeType};
use glib::translate::*;

define_render_node!(
    ContainerNode,
    ffi::GskContainerNode,
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
}
