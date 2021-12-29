// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{InsetShadowNode, RenderNodeType};

define_render_node!(
    InsetShadowNode,
    ffi::GskInsetShadowNode,
    RenderNodeType::InsetShadowNode
);
