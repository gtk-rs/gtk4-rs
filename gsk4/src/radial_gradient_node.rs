// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RadialGradientNode, RenderNodeType};

define_render_node!(
    RadialGradientNode,
    ffi::GskRadialGradientNode,
    RenderNodeType::RadialGradientNode
);
