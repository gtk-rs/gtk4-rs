// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{OpacityNode, RenderNodeType};

define_render_node!(
    OpacityNode,
    ffi::GskOpacityNode,
    RenderNodeType::OpacityNode
);
