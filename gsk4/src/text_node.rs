// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNodeType, TextNode};

define_render_node!(TextNode, ffi::GskTextNode, RenderNodeType::TextNode);

impl std::fmt::Debug for TextNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[cfg(feature = "v4_2")]
        {
            f.debug_struct("TextNode")
                .field("color", &self.color())
                .field("glyphs", &self.glyphs())
                .field("num_glyphs", &self.num_glyphs())
                .field("font", &self.font())
                .field("offset", &self.offset())
                .field("has_color_glyphs", &self.has_color_glyphs())
                .finish()
        }
        #[cfg(not(feature = "v4_2"))]
        {
            f.debug_struct("TextNode")
                .field("color", &self.color())
                .field("glyphs", &self.glyphs())
                .field("num_glyphs", &self.num_glyphs())
                .field("font", &self.font())
                .field("offset", &self.offset())
                .finish()
        }
    }
}
