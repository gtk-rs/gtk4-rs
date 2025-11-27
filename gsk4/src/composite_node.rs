// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{CompositeNode, RenderNodeType};

define_render_node!(
    CompositeNode,
    crate::ffi::GskCompositeNode,
    RenderNodeType::CompositeNode
);

impl std::fmt::Debug for CompositeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CompositeNode")
            .field("child", &self.child())
            .field("mask", &self.mask())
            .field("operator", &self.operator())
            .finish()
    }
}
