// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNodeType, StrokeNode};

define_render_node!(
    StrokeNode,
    crate::ffi::GskStrokeNode,
    RenderNodeType::StrokeNode
);

impl std::fmt::Debug for StrokeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StrokeNode")
            .field("child", &self.child())
            .field("stroke", &self.stroke())
            .field("path", &self.path())
            .finish()
    }
}
