// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNodeType, TextureNode};

define_render_node!(
    TextureNode,
    ffi::GskTextureNode,
    RenderNodeType::TextureNode
);

impl std::fmt::Debug for TextureNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TextureNode")
            .field("texture", &self.texture())
            .finish()
    }
}
