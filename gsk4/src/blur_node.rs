// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{BlurNode, RenderNodeType};

define_render_node!(BlurNode, ffi::GskBlurNode, RenderNodeType::BlurNode);
