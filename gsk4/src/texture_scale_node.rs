// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNodeType, TextureScaleNode};

define_render_node!(
    TextureScaleNode,
    ffi::GskTextureScaleNode,
    RenderNodeType::TextureScaleNode
);

impl std::fmt::Debug for TextureScaleNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TextureScaleNode")
            .field("texture", &self.texture())
            .field("filter", &self.filter())
            .finish()
    }
}
