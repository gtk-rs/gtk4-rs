// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{BlendNode, RenderNodeType};

define_render_node!(BlendNode, ffi::GskBlendNode, RenderNodeType::BlendNode);

impl std::fmt::Debug for BlendNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BlendNode")
            .field("blend_mode", &self.blend_mode())
            .field("bottom_child", &self.bottom_child())
            .field("top_child", &self.top_child())
            .finish()
    }
}
