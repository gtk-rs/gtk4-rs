// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{BlendNode, RenderNodeType};

define_render_node!(BlendNode, ffi::GskBlendNode, RenderNodeType::BlendNode);
