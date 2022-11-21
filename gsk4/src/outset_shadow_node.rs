// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{OutsetShadowNode, RenderNodeType};

define_render_node!(
    OutsetShadowNode,
    ffi::GskOutsetShadowNode,
    RenderNodeType::OutsetShadowNode
);

impl std::fmt::Debug for OutsetShadowNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("OutsetShadowNode")
            .field("blur_radius", &self.blur_radius())
            .field("color", &self.color())
            .field("dx", &self.dx())
            .field("dy", &self.dy())
            .field("outline", &self.outline())
            .field("spread", &self.spread())
            .finish()
    }
}
