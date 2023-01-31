// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, EntryBuffer};
use glib::{translate::*, GString, IntoGStr, IntoOptionalGStr};
use libc::{c_int, c_uint};

impl EntryBuffer {
    #[doc(alias = "gtk_entry_buffer_new")]
    pub fn new(initial_chars: impl IntoOptionalGStr) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            initial_chars.run_with_gstr(|initial_chars| {
                from_glib_full(ffi::gtk_entry_buffer_new(
                    initial_chars.to_glib_none().0,
                    -1,
                ))
            })
        }
    }
}

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`EntryBuffer`](crate::EntryBuffer).
pub trait EntryBufferExtManual: 'static {
    #[doc(alias = "gtk_entry_buffer_delete_text")]
    fn delete_text(&self, position: u16, n_chars: Option<u16>) -> u16;

    #[doc(alias = "gtk_entry_buffer_get_bytes")]
    #[doc(alias = "get_bytes")]
    fn bytes(&self) -> usize;

    #[doc(alias = "gtk_entry_buffer_get_length")]
    #[doc(alias = "get_length")]
    fn length(&self) -> u16;

    #[doc(alias = "gtk_entry_buffer_get_max_length")]
    #[doc(alias = "get_max_length")]
    fn max_length(&self) -> Option<u16>;

    #[doc(alias = "gtk_entry_buffer_get_text")]
    #[doc(alias = "get_text")]
    fn text(&self) -> GString;

    #[doc(alias = "gtk_entry_buffer_insert_text")]
    fn insert_text(&self, position: u16, chars: impl IntoGStr) -> u16;

    #[doc(alias = "gtk_entry_buffer_set_max_length")]
    fn set_max_length(&self, max_length: Option<u16>);

    #[doc(alias = "gtk_entry_buffer_set_text")]
    fn set_text(&self, chars: impl IntoGStr);
}

macro_rules! to_u16 {
    ($e:expr) => (
        {
            let x = $e;
            assert!(x as usize <= u16::max_value() as usize,
                "Unexpected value exceeding `u16` range");
            x as u16
        }
    )
}

impl<O: IsA<EntryBuffer>> EntryBufferExtManual for O {
    fn delete_text(&self, position: u16, n_chars: Option<u16>) -> u16 {
        unsafe {
            to_u16!(ffi::gtk_entry_buffer_delete_text(
                self.as_ref().to_glib_none().0,
                position as c_uint,
                n_chars.map(|n| n as c_int).unwrap_or(-1)
            ))
        }
    }

    fn bytes(&self) -> usize {
        unsafe { ffi::gtk_entry_buffer_get_bytes(self.as_ref().to_glib_none().0) as _ }
    }

    fn length(&self) -> u16 {
        unsafe {
            to_u16!(ffi::gtk_entry_buffer_get_length(
                self.as_ref().to_glib_none().0
            ))
        }
    }

    fn max_length(&self) -> Option<u16> {
        unsafe {
            match ffi::gtk_entry_buffer_get_max_length(self.as_ref().to_glib_none().0) {
                0 => None,
                x => Some(to_u16!(x)),
            }
        }
    }

    fn text(&self) -> GString {
        unsafe {
            from_glib_none(ffi::gtk_entry_buffer_get_text(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn insert_text(&self, position: u16, chars: impl IntoGStr) -> u16 {
        unsafe {
            chars.run_with_gstr(|chars| {
                to_u16!(ffi::gtk_entry_buffer_insert_text(
                    self.as_ref().to_glib_none().0,
                    position as c_uint,
                    chars.as_ptr(),
                    -1
                ))
            })
        }
    }

    fn set_max_length(&self, max_length: Option<u16>) {
        unsafe {
            assert_ne!(max_length, Some(0), "Zero maximum length not supported");
            ffi::gtk_entry_buffer_set_max_length(
                self.as_ref().to_glib_none().0,
                max_length.unwrap_or(0) as c_int,
            );
        }
    }

    fn set_text(&self, chars: impl IntoGStr) {
        unsafe {
            chars.run_with_gstr(|chars| {
                ffi::gtk_entry_buffer_set_text(self.as_ref().to_glib_none().0, chars.as_ptr(), -1);
            })
        }
    }
}

impl Default for EntryBuffer {
    fn default() -> Self {
        glib::Object::new()
    }
}
