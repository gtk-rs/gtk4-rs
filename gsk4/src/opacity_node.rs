// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{OpacityNode, RenderNodeType};

define_render_node!(
    OpacityNode,
    ffi::GskOpacityNode,
    RenderNodeType::OpacityNode
);

impl std::fmt::Debug for OpacityNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("OpacityNode")
            .field("child", &self.child())
            .field("opacity", &self.opacity())
            .finish()
    }
}
