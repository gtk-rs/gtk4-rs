// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{CrossFadeNode, RenderNodeType};

define_render_node!(
    CrossFadeNode,
    ffi::GskCrossFadeNode,
    RenderNodeType::CrossFadeNode
);
