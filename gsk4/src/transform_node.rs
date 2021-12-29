// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNodeType, TransformNode};

define_render_node!(
    TransformNode,
    ffi::GskTransformNode,
    RenderNodeType::TransformNode
);
