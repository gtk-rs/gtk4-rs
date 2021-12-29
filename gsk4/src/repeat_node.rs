// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNodeType, RepeatNode};

define_render_node!(RepeatNode, ffi::GskRepeatNode, RenderNodeType::RepeatNode);
