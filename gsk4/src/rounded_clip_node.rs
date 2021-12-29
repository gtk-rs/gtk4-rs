// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNodeType, RoundedClipNode};

define_render_node!(
    RoundedClipNode,
    ffi::GskRoundedClipNode,
    RenderNodeType::RoundedClipNode
);
