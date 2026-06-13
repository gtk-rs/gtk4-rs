// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ComponentTransferNode, RenderNodeType};

define_render_node!(
    ComponentTransferNode,
    crate::ffi::GskComponentTransferNode,
    RenderNodeType::ComponentTransferNode
);

impl std::fmt::Debug for ComponentTransferNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ComponentTransferNode")
            .field("child", &self.child())
            .finish()
    }
}
