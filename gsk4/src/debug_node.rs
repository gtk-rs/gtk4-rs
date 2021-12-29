// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{DebugNode, RenderNodeType};

define_render_node!(DebugNode, ffi::GskDebugNode, RenderNodeType::DebugNode);
