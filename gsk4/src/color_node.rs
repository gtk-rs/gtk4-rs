// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ColorNode, RenderNodeType};

define_render_node!(ColorNode, ffi::GskColorNode, RenderNodeType::ColorNode);
