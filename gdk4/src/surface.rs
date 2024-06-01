// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{ffi, prelude::*, Surface};

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Surface>> Sealed for T {}
}

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of
/// [`Surface`](crate::Surface).
pub trait SurfaceExtManual: sealed::Sealed + IsA<Surface> + 'static {
    #[doc(alias = "gdk_surface_create_similar_surface")]
    fn create_similar_surface(
        &self,
        content: cairo::Content,
        width: i32,
        height: i32,
    ) -> cairo::Surface {
        unsafe {
            from_glib_full(ffi::gdk_surface_create_similar_surface(
                self.as_ref().to_glib_none().0,
                content.into(),
                width,
                height,
            ))
        }
    }

    // Returns true if the coordinates were successfully translated
    #[doc(alias = "gdk_surface_translate_coordinates")]
    fn translate_coordinates(&self, to: &Surface, mut x: f64, mut y: f64) -> bool {
        unsafe {
            from_glib(ffi::gdk_surface_translate_coordinates(
                self.as_ref().to_glib_none().0,
                to.to_glib_none().0,
                &mut x,
                &mut y,
            ))
        }
    }
}

impl<O: IsA<Surface>> SurfaceExtManual for O {}
