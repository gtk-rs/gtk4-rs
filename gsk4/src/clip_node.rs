// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ClipNode, RenderNodeType};

define_render_node!(ClipNode, ffi::GskClipNode, RenderNodeType::ClipNode);
