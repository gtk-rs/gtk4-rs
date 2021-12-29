// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{CairoNode, RenderNodeType};

define_render_node!(CairoNode, ffi::GskCairoNode, RenderNodeType::CairoNode);
