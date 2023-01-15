// Take a look at the license at the top of the repository in the LICENSE file.

use std::borrow::Borrow;

use crate::{prelude::*, Snapshot};
use glib::{translate::*, IntoGStr};

pub trait SnapshotExtManual {
    #[doc(alias = "gtk_snapshot_append_border")]
    fn append_border(
        &self,
        outline: &gsk::RoundedRect,
        border_width: &[f32; 4],
        border_color: &[gdk::RGBA; 4],
    );

    #[doc(alias = "gtk_snapshot_push_debug")]
    fn push_debug(&self, message: impl IntoGStr);
}

impl<T: IsA<Snapshot>> SnapshotExtManual for T {
    fn append_border(
        &self,
        outline: &gsk::RoundedRect,
        border_width: &[f32; 4],
        border_color: &[gdk::RGBA; 4],
    ) {
        unsafe {
            let border_color_ptr: Vec<gdk::ffi::GdkRGBA> =
                border_color.iter().map(|c| *c.to_glib_none().0).collect();
            ffi::gtk_snapshot_append_border(
                self.as_ref().to_glib_none().0,
                outline.to_glib_none().0,
                border_width,
                border_color_ptr.as_ptr() as *const _,
            )
        }
    }

    fn push_debug(&self, message: impl IntoGStr) {
        unsafe {
            message.run_with_gstr(|message| {
                ffi::gtk_snapshot_push_debug(self.as_ref().to_glib_none().0, message.as_ptr())
            })
        }
    }
}

impl AsRef<Snapshot> for gdk::Snapshot {
    #[inline]
    fn as_ref(&self) -> &Snapshot {
        self.downcast_ref().unwrap()
    }
}

impl From<gdk::Snapshot> for Snapshot {
    #[inline]
    fn from(e: gdk::Snapshot) -> Snapshot {
        assert_initialized_main_thread!();
        e.downcast().unwrap()
    }
}

impl Borrow<Snapshot> for gdk::Snapshot {
    #[inline]
    fn borrow(&self) -> &Snapshot {
        self.downcast_ref().unwrap()
    }
}

unsafe impl IsA<Snapshot> for gdk::Snapshot {}
