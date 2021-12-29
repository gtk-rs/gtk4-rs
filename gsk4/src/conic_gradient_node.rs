// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ConicGradientNode, RenderNodeType};

define_render_node!(
    ConicGradientNode,
    ffi::GskConicGradientNode,
    RenderNodeType::ConicGradientNode
);
