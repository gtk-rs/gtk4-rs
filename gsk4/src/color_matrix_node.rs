// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ColorMatrixNode, RenderNodeType};

define_render_node!(
    ColorMatrixNode,
    ffi::GskColorMatrixNode,
    RenderNodeType::ColorMatrixNode
);
