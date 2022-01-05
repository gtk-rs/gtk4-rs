// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNodeType, TransformNode};

define_render_node!(
    TransformNode,
    ffi::GskTransformNode,
    RenderNodeType::TransformNode
);

impl std::fmt::Debug for TransformNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TransformNode")
            .field("child", &self.child())
            .field("transform", &self.transform())
            .finish()
    }
}
