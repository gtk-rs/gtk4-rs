// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{MaskNode, RenderNodeType};

define_render_node!(MaskNode, ffi::GskMaskNode, RenderNodeType::MaskNode);

impl std::fmt::Debug for MaskNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MaskNode")
            .field("mask", &self.mask())
            .field("mask_mode", &self.mask_mode())
            .field("source", &self.source())
            .finish()
    }
}
