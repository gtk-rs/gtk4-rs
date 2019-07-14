use glib::object::IsA;
use glib::translate::*;
use Surface;

pub trait SurfaceExtManual: 'static {
    fn create_similar_surface(&self, content: cairo::Content, width: i32, height: i32) -> Option<cairo::Surface>;
}

impl<O: IsA<Surface>> SurfaceExtManual for O {
    fn create_similar_surface(
        &self,
        content: cairo::Content,
        width: i32,
        height: i32,
    ) -> Option<cairo::Surface> {
        unsafe {
            from_glib_full(gdk_sys::gdk_surface_create_similar_surface(
                self.as_ref().to_glib_none().0,
                content.into(),
                width,
                height,
            ))
        }
    }
}
