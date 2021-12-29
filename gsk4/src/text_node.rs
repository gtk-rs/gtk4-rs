// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNodeType, TextNode};

define_render_node!(TextNode, ffi::GskTextNode, RenderNodeType::TextNode);
