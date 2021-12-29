// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNodeType, TextureNode};

define_render_node!(
    TextureNode,
    ffi::GskTextureNode,
    RenderNodeType::TextureNode
);
