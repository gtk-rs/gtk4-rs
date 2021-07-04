// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNode, Renderer};
use glib::object::IsA;
use glib::translate::*;

pub trait RendererExtManual: 'static {
    #[doc(alias = "gsk_renderer_render")]
    fn render<P: AsRef<RenderNode>>(&self, root: &P, region: Option<&cairo::Region>);

    #[doc(alias = "gsk_renderer_render_texture")]
    fn render_texture<P: AsRef<RenderNode>>(
        &self,
        root: &P,
        viewport: Option<&graphene::Rect>,
    ) -> Option<gdk::Texture>;
}

impl<O: IsA<Renderer>> RendererExtManual for O {
    fn render<P: AsRef<RenderNode>>(&self, root: &P, region: Option<&cairo::Region>) {
        unsafe {
            ffi::gsk_renderer_render(
                self.as_ref().to_glib_none().0,
                root.as_ref().to_glib_none().0,
                region.to_glib_none().0,
            );
        }
    }

    fn render_texture<P: AsRef<RenderNode>>(
        &self,
        root: &P,
        viewport: Option<&graphene::Rect>,
    ) -> Option<gdk::Texture> {
        unsafe {
            from_glib_full(ffi::gsk_renderer_render_texture(
                self.as_ref().to_glib_none().0,
                root.as_ref().to_glib_none().0,
                viewport.to_glib_none().0,
            ))
        }
    }
}
