// Take a look at the license at the top of the repository in the LICENSE file.

use crate::RenderNodeType;
use glib::object::IsA;
use glib::translate::*;

define_render_node!(
    TextureNode,
    ffi::GskTextureNode,
    ffi::gsk_texture_node_get_type,
    RenderNodeType::TextureNode
);

impl TextureNode {
    #[doc(alias = "gsk_texture_node_new")]
    pub fn new<P: IsA<gdk::Texture>>(texture: &P, bounds: &graphene::Rect) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gsk_texture_node_new(
                texture.as_ref().to_glib_none().0,
                bounds.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_texture_node_get_texture")]
    #[doc(alias = "get_texture")]
    pub fn texture(&self) -> Option<gdk::Texture> {
        unsafe { from_glib_none(ffi::gsk_texture_node_get_texture(self.to_glib_none().0)) }
    }
}
