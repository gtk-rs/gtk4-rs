// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNodeType, TextureScaleNode};

define_render_node!(
    TextureScaleNode,
    crate::ffi::GskTextureScaleNode,
    RenderNodeType::TextureScaleNode
);

impl TextureScaleNode {
    #[cfg(feature = "v4_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_24")))]
    #[doc(alias = "gsk_texture_scale_node_get_snap")]
    #[doc(alias = "get_snap")]
    pub fn snap(&self) -> crate::RectSnap {
        unsafe {
            glib::translate::from_glib(crate::ffi::gsk_texture_scale_node_get_snap(
                glib::translate::ToGlibPtr::to_glib_none(self).0,
            ))
        }
    }
}

impl std::fmt::Debug for TextureScaleNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TextureScaleNode")
            .field("texture", &self.texture())
            .field("filter", &self.filter())
            .finish()
    }
}
