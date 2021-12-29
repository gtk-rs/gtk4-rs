// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{OutsetShadowNode, RenderNodeType};

define_render_node!(
    OutsetShadowNode,
    ffi::GskOutsetShadowNode,
    RenderNodeType::OutsetShadowNode
);
