// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, TextBuffer, TextIter, TextTag};
use glib::{
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use libc::{c_char, c_int};
use std::{boxed::Box as Box_, mem::transmute, slice, str};

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::TextBuffer>> Sealed for T {}
}

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`TextBuffer`](crate::TextBuffer).
pub trait TextBufferExtManual: sealed::Sealed + IsA<TextBuffer> + 'static {
    // rustdoc-stripper-ignore-next
    /// # Panics
    ///
    /// If the properties don't exists or are not writeable.
    #[doc(alias = "gtk_text_buffer_create_tag")]
    fn create_tag(
        &self,
        tag_name: Option<&str>,
        properties: &[(&str, &dyn ToValue)],
    ) -> Option<TextTag> {
        let tag = TextTag::new(tag_name);
        tag.set_properties(properties);
        if self.as_ref().tag_table().add(&tag) {
            Some(tag)
        } else {
            None
        }
    }

    #[doc(alias = "gtk_text_buffer_insert_with_tags")]
    fn insert_with_tags(&self, iter: &mut TextIter, text: &str, tags: &[&TextTag]) {
        let start_offset = iter.offset();
        self.as_ref().insert(iter, text);
        let start_iter = self.as_ref().iter_at_offset(start_offset);
        tags.iter().for_each(|tag| {
            self.as_ref().apply_tag(&(*tag).clone(), &start_iter, iter);
        });
    }

    #[doc(alias = "gtk_text_buffer_insert_with_tags_by_name")]
    fn insert_with_tags_by_name(&self, iter: &mut TextIter, text: &str, tags_names: &[&str]) {
        let start_offset = iter.offset();
        self.as_ref().insert(iter, text);
        let start_iter = self.as_ref().iter_at_offset(start_offset);
        let tag_table = self.as_ref().tag_table();
        tags_names.iter().for_each(|tag_name| {
            if let Some(tag) = tag_table.lookup(tag_name) {
                self.as_ref().apply_tag(&tag, &start_iter, iter);
            } else {
                glib::g_warning!("TextBuffer", "No tag with name {}!", tag_name);
            }
        });
    }

    fn connect_insert_text<F: Fn(&Self, &mut TextIter, &str) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            unsafe extern "C" fn insert_text_trampoline<
                T,
                F: Fn(&T, &mut TextIter, &str) + 'static,
            >(
                this: *mut ffi::GtkTextBuffer,
                location: *mut ffi::GtkTextIter,
                text: *mut c_char,
                len: c_int,
                f: glib::ffi::gpointer,
            ) where
                T: IsA<TextBuffer>,
            {
                let mut location_copy = from_glib_none(location);
                let f: &F = &*(f as *const F);
                let text = if len <= 0 {
                    &[]
                } else {
                    slice::from_raw_parts(text as *const u8, len as usize)
                };

                f(
                    TextBuffer::from_glib_borrow(this).unsafe_cast_ref(),
                    &mut location_copy,
                    str::from_utf8(text).unwrap(),
                )
            }
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.to_glib_none().0 as *mut _,
                b"insert-text\0".as_ptr() as *mut _,
                Some(transmute(insert_text_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<TextBuffer>> TextBufferExtManual for O {}

impl std::fmt::Write for TextBuffer {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let mut iter = self.end_iter();
        self.insert(&mut iter, s);
        Ok(())
    }
}
