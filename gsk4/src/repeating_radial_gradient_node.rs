// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNodeType, RepeatingRadialGradientNode};

define_render_node!(
    RepeatingRadialGradientNode,
    ffi::GskRepeatingRadialGradientNode,
    RenderNodeType::RepeatingRadialGradientNode
);
