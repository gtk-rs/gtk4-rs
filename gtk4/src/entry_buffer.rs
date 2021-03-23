// Take a look at the license at the top of the repository in the LICENSE file.

use crate::EntryBuffer;
use glib::translate::*;
use glib::{object::IsA, SignalHandlerId};
use glib::{signal::connect_raw, Cast};
use libc::{c_int, c_uint};
use std::boxed::Box as Box_;
use std::mem::transmute;

impl EntryBuffer {
    #[doc(alias = "gtk_entry_buffer_new")]
    pub fn new(initial_chars: Option<&str>) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_entry_buffer_new(
                initial_chars.to_glib_none().0,
                -1,
            ))
        }
    }
}

pub trait EntryBufferExtManual: 'static {
    #[doc(alias = "gtk_entry_buffer_emit_deleted_text")]
    fn emit_deleted_text(&self, position: u16, n_chars: u16);

    #[doc(alias = "gtk_entry_buffer_emit_inserted_text")]
    fn emit_inserted_text(&self, position: u16, chars: &str, n_chars: u16);

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
    fn text(&self) -> glib::GString;

    #[doc(alias = "gtk_entry_buffer_insert_text")]
    fn insert_text(&self, position: u16, chars: &str) -> u16;

    #[doc(alias = "gtk_entry_buffer_set_max_length")]
    fn set_max_length(&self, max_length: Option<u16>);

    #[doc(alias = "gtk_entry_buffer_set_text")]
    fn set_text(&self, chars: &str);

    fn connect_deleted_text<F: Fn(&Self, u16, u16) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_inserted_text<F: Fn(&Self, u16, &str, u16) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
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
    fn emit_deleted_text(&self, position: u16, n_chars: u16) {
        unsafe {
            ffi::gtk_entry_buffer_emit_deleted_text(
                self.as_ref().to_glib_none().0,
                position as c_uint,
                n_chars as c_uint,
            );
        }
    }

    fn emit_inserted_text(&self, position: u16, chars: &str, n_chars: u16) {
        unsafe {
            ffi::gtk_entry_buffer_emit_inserted_text(
                self.as_ref().to_glib_none().0,
                position as c_uint,
                chars.to_glib_none().0,
                n_chars as c_uint,
            );
        }
    }

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
        unsafe { ffi::gtk_entry_buffer_get_bytes(self.as_ref().to_glib_none().0) as usize }
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

    fn text(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::gtk_entry_buffer_get_text(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn insert_text(&self, position: u16, chars: &str) -> u16 {
        unsafe {
            to_u16!(ffi::gtk_entry_buffer_insert_text(
                self.as_ref().to_glib_none().0,
                position as c_uint,
                chars.to_glib_none().0,
                -1
            ))
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

    fn set_text(&self, chars: &str) {
        unsafe {
            ffi::gtk_entry_buffer_set_text(
                self.as_ref().to_glib_none().0,
                chars.to_glib_none().0,
                -1,
            );
        }
    }

    fn connect_deleted_text<F: Fn(&Self, u16, u16) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn deleted_text_trampoline<P, F: Fn(&P, u16, u16) + 'static>(
            this: *mut ffi::GtkEntryBuffer,
            position: libc::c_uint,
            n_chars: libc::c_uint,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<EntryBuffer>,
        {
            let f: &F = &*(f as *const F);
            f(
                &EntryBuffer::from_glib_borrow(this).unsafe_cast_ref(),
                to_u16!(position),
                to_u16!(n_chars),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"deleted-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    deleted_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_inserted_text<F: Fn(&Self, u16, &str, u16) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn inserted_text_trampoline<P, F: Fn(&P, u16, &str, u16) + 'static>(
            this: *mut ffi::GtkEntryBuffer,
            position: libc::c_uint,
            chars: *mut libc::c_char,
            n_chars: libc::c_uint,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<EntryBuffer>,
        {
            let f: &F = &*(f as *const F);
            f(
                &EntryBuffer::from_glib_borrow(this).unsafe_cast_ref(),
                to_u16!(position),
                &glib::GString::from_glib_borrow(chars),
                to_u16!(n_chars),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"inserted-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    inserted_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::EntryBufferExt;
    use crate::TEST_THREAD_WORKER;

    #[test]
    fn test_entry_buffer() {
        TEST_THREAD_WORKER
            .push(move || {
                let text = crate::Text::new();
                let buffer = text.buffer().unwrap();
                buffer.insert_text(0, "Hello world");
                assert_eq!(buffer.text(), "Hello world");
                buffer.connect_inserted_text(move |buffer, pos, text, length| {
                    assert_eq!(length, 11);
                    assert_eq!(pos, 1);
                });
                buffer.emit_inserted_text(0, "hello world", 11);
                std::thread::sleep(std::time::Duration::from_secs(10));
            })
            .expect("Failed to schedule a test call");
    }
}
