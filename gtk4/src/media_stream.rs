// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{prelude::*, MediaStream};

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::MediaStream>> Sealed for T {}
}

pub trait MediaStreamExtManual: sealed::Sealed + IsA<MediaStream> + 'static {
    #[doc(alias = "gtk_media_stream_gerror")]
    #[doc(alias = "gtk_media_stream_error")]
    #[doc(alias = "gerror")]
    fn set_error(&self, error: glib::Error) {
        unsafe {
            crate::ffi::gtk_media_stream_gerror(
                self.as_ref().to_glib_none().0,
                mut_override(error.into_glib_ptr()),
            );
        }
    }
}

impl<O: IsA<MediaStream>> MediaStreamExtManual for O {}
