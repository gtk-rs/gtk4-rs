// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{BlurNode, RenderNodeType};

define_render_node!(BlurNode, ffi::GskBlurNode, RenderNodeType::BlurNode);

impl std::fmt::Debug for BlurNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BlurNode")
            .field("child", &self.child())
            .field("radius", &self.radius())
            .finish()
    }
}
