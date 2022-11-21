// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ColorMatrixNode, RenderNodeType};

define_render_node!(
    ColorMatrixNode,
    ffi::GskColorMatrixNode,
    RenderNodeType::ColorMatrixNode
);

impl std::fmt::Debug for ColorMatrixNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ColorMatrixNode")
            .field("color_offset", &self.color_offset())
            .field("color_matrix", &self.color_matrix())
            .field("child", &self.child())
            .finish()
    }
}
