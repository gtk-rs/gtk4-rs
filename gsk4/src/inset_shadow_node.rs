// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{InsetShadowNode, RenderNodeType};

define_render_node!(
    InsetShadowNode,
    ffi::GskInsetShadowNode,
    RenderNodeType::InsetShadowNode
);

impl std::fmt::Debug for InsetShadowNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InsetShadowNode")
            .field("blur_radius", &self.blur_radius())
            .field("color", &self.color())
            .field("dx", &self.dx())
            .field("dy", &self.dy())
            .field("outline", &self.outline())
            .field("spread", &self.spread())
            .finish()
    }
}
