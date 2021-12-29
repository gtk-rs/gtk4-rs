// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{LinearGradientNode, RenderNodeType};

define_render_node!(
    LinearGradientNode,
    ffi::GskLinearGradientNode,
    RenderNodeType::LinearGradientNode
);
