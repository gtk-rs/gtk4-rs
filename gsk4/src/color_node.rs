// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ColorNode, RenderNodeType};

define_render_node!(ColorNode, ffi::GskColorNode, RenderNodeType::ColorNode);

impl std::fmt::Debug for ColorNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ColorNode")
            .field("color", &self.color())
            .finish()
    }
}
