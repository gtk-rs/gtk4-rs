// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`EntryBuffer`](crate::EntryBuffer).

use crate::{prelude::*, subclass::prelude::*, EntryBuffer};
use glib::{translate::*, GString};
use once_cell::sync::Lazy;

use super::PtrHolder;

pub trait EntryBufferImpl: EntryBufferImplExt + ObjectImpl {
    fn delete_text(&self, position: u32, n_chars: Option<u32>) -> u32 {
        self.parent_delete_text(position, n_chars)
    }

    fn deleted_text(&self, position: u32, n_chars: Option<u32>) {
        self.parent_deleted_text(position, n_chars)
    }

    #[doc(alias = "get_length")]
    fn length(&self) -> u32 {
        self.parent_length()
    }

    #[doc(alias = "get_text")]
    fn text(&self) -> GString {
        self.parent_text()
    }
    fn insert_text(&self, position: u32, chars: &str) -> u32 {
        self.parent_insert_text(position, chars)
    }

    fn inserted_text(&self, position: u32, chars: &str) {
        self.parent_inserted_text(position, chars)
    }
}

pub trait EntryBufferImplExt: ObjectSubclass {
    fn parent_delete_text(&self, position: u32, n_chars: Option<u32>) -> u32;
    fn parent_deleted_text(&self, position: u32, n_chars: Option<u32>);
    fn parent_length(&self) -> u32;
    fn parent_text(&self) -> GString;
    fn parent_insert_text(&self, position: u32, chars: &str) -> u32;
    fn parent_inserted_text(&self, position: u32, chars: &str);
}

impl<T: EntryBufferImpl> EntryBufferImplExt for T {
    fn parent_delete_text(&self, position: u32, n_chars: Option<u32>) -> u32 {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkEntryBufferClass;
            let f = (*parent_class)
                .delete_text
                .expect("No parent class impl for \"delete_text\"");
            f(
                self.obj().unsafe_cast_ref::<EntryBuffer>().to_glib_none().0,
                position,
                n_chars.unwrap_or(u32::MAX),
            )
        }
    }

    fn parent_deleted_text(&self, position: u32, n_chars: Option<u32>) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkEntryBufferClass;
            if let Some(f) = (*parent_class).deleted_text {
                f(
                    self.obj().unsafe_cast_ref::<EntryBuffer>().to_glib_none().0,
                    position,
                    n_chars.unwrap_or(u32::MAX),
                )
            }
        }
    }

    fn parent_length(&self) -> u32 {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkEntryBufferClass;
            let f = (*parent_class)
                .get_length
                .expect("No parent class impl for \"get_length\"");
            f(self.obj().unsafe_cast_ref::<EntryBuffer>().to_glib_none().0)
        }
    }

    fn parent_text(&self) -> GString {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkEntryBufferClass;
            let f = (*parent_class)
                .get_text
                .expect("No parent class impl for \"get_text\"");
            let mut n_bytes = 0;
            let res = f(
                self.obj().unsafe_cast_ref::<EntryBuffer>().to_glib_none().0,
                &mut n_bytes,
            );
            FromGlibContainer::from_glib_none_num(res, n_bytes as _)
        }
    }

    fn parent_insert_text(&self, position: u32, text: &str) -> u32 {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkEntryBufferClass;
            let f = (*parent_class)
                .insert_text
                .expect("No parent class impl for \"insert_text\"");

            f(
                self.obj().unsafe_cast_ref::<EntryBuffer>().to_glib_none().0,
                position,
                text.to_glib_none().0,
                text.chars().count() as u32,
            )
        }
    }

    fn parent_inserted_text(&self, position: u32, text: &str) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkEntryBufferClass;
            if let Some(f) = (*parent_class).inserted_text {
                f(
                    self.obj().unsafe_cast_ref::<EntryBuffer>().to_glib_none().0,
                    position,
                    text.to_glib_none().0,
                    text.chars().count() as u32,
                )
            }
        }
    }
}

unsafe impl<T: EntryBufferImpl> IsSubclassable<T> for EntryBuffer {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        assert_initialized_main_thread!();

        let klass = class.as_mut();
        klass.delete_text = Some(entry_buffer_delete_text::<T>);
        klass.deleted_text = Some(entry_buffer_deleted_text::<T>);
        klass.get_length = Some(entry_buffer_get_length::<T>);
        klass.get_text = Some(entry_buffer_get_text::<T>);
        klass.insert_text = Some(entry_buffer_insert_text::<T>);
        klass.inserted_text = Some(entry_buffer_inserted_text::<T>);
    }
}

unsafe extern "C" fn entry_buffer_delete_text<T: EntryBufferImpl>(
    ptr: *mut ffi::GtkEntryBuffer,
    position: u32,
    n_chars: u32,
) -> u32 {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let n_chars = if n_chars == u32::MAX {
        None
    } else {
        Some(n_chars)
    };

    imp.delete_text(position, n_chars)
}

unsafe extern "C" fn entry_buffer_deleted_text<T: EntryBufferImpl>(
    ptr: *mut ffi::GtkEntryBuffer,
    position: u32,
    n_chars: u32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let n_chars = if n_chars == u32::MAX {
        None
    } else {
        Some(n_chars)
    };

    imp.deleted_text(position, n_chars)
}

static GET_TEXT_QUARK: Lazy<glib::Quark> =
    Lazy::new(|| glib::Quark::from_str("gtk4-rs-subclass-entry-buffer-text"));

unsafe extern "C" fn entry_buffer_get_text<T: EntryBufferImpl>(
    ptr: *mut ffi::GtkEntryBuffer,
    n_bytes: *mut usize,
) -> *const libc::c_char {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let ret = imp.text();
    if !n_bytes.is_null() {
        *n_bytes = ret.len();
    }
    // Ensures that the returned text stays alive for as long as
    // the entry buffer instance
    let fullptr = ret.into_glib_ptr();
    imp.obj().set_qdata(
        *GET_TEXT_QUARK,
        PtrHolder(fullptr, |ptr| {
            glib::ffi::g_free(ptr as *mut _);
        }),
    );
    fullptr
}

unsafe extern "C" fn entry_buffer_get_length<T: EntryBufferImpl>(
    ptr: *mut ffi::GtkEntryBuffer,
) -> u32 {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.length()
}

unsafe extern "C" fn entry_buffer_insert_text<T: EntryBufferImpl>(
    ptr: *mut ffi::GtkEntryBuffer,
    position: u32,
    charsptr: *const libc::c_char,
    n_chars: u32,
) -> u32 {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let text: Borrowed<GString> = from_glib_borrow(charsptr);

    let chars = text_n_chars(&text, n_chars);
    imp.insert_text(position, chars)
}

unsafe extern "C" fn entry_buffer_inserted_text<T: EntryBufferImpl>(
    ptr: *mut ffi::GtkEntryBuffer,
    position: u32,
    charsptr: *const libc::c_char,
    length: u32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let text: Borrowed<GString> = from_glib_borrow(charsptr);

    let chars = text_n_chars(&text, length);
    imp.inserted_text(position, chars)
}

#[doc(alias = "get_text_n_chars")]
fn text_n_chars(text: &str, n_chars: u32) -> &str {
    if n_chars != u32::MAX && n_chars > 0 {
        let mut iter = text
            .char_indices()
            .skip((n_chars - 1) as _)
            .map(|(pos, _)| pos);
        iter
            .next()
            .expect(
                "The passed text to EntryBuffer contains fewer characters than what's passed as a length",
            );
        let pos_end = iter.next().unwrap_or(text.len());
        &text[..pos_end]
    } else if n_chars == 0 {
        // Avoid doing skipping to -1 char
        ""
    } else {
        text
    }
}

#[cfg(test)]
mod test {
    use super::text_n_chars;
    #[std::prelude::v1::test]
    fn n_chars_max_length_ascii() {
        assert_eq!(text_n_chars("gtk-rs bindings", 6), "gtk-rs");
        assert_eq!(text_n_chars("gtk-rs bindings", u32::MAX), "gtk-rs bindings");
    }

    #[std::prelude::v1::test]
    #[should_panic]
    fn n_chars_max_length_ascii_panic() {
        assert_eq!(text_n_chars("gtk-rs", 7), "gtk-rs");
    }

    #[std::prelude::v1::test]
    fn n_chars_max_length_utf8() {
        assert_eq!(text_n_chars("👨👩👧👦", 2), "👨👩");
        assert_eq!(text_n_chars("👨👩👧👦", 0), "");
        assert_eq!(text_n_chars("👨👩👧👦", 4), "👨👩👧👦");
        assert_eq!(text_n_chars("👨👩👧👦", u32::MAX), "👨👩👧👦");
        assert_eq!(text_n_chars("كتاب", 2), "كت");
    }

    #[std::prelude::v1::test]
    fn n_chars_max_length_utf8_ascii() {
        assert_eq!(text_n_chars("👨g👩t👧k👦", 2), "👨g");
        assert_eq!(text_n_chars("👨g👩t👧k👦", 5), "👨g👩t👧");
        assert_eq!(text_n_chars("كaتاب", 3), "كaت");
    }

    #[std::prelude::v1::test]
    #[should_panic]
    fn n_chars_max_length_utf8_panic() {
        assert_eq!(text_n_chars("👨👩👧👦", 5), "👨👩");
    }
}
