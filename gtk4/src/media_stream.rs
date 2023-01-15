// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, MediaStream};
use glib::translate::*;

pub trait MediaStreamExtManual: 'static {
    #[doc(alias = "gtk_media_stream_gerror")]
    #[doc(alias = "gtk_media_stream_error")]
    #[doc(alias = "gerror")]
    fn set_error(&self, error: glib::Error);
}

impl<O: IsA<MediaStream>> MediaStreamExtManual for O {
    fn set_error(&self, error: glib::Error) {
        unsafe {
            ffi::gtk_media_stream_gerror(
                self.as_ref().to_glib_none().0,
                mut_override(error.into_glib_ptr()),
            );
        }
    }
}
