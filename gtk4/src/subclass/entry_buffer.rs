// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::EntryBuffer;
use glib::translate::*;
use glib::{Cast, GString, Object, ObjectExt};
use once_cell::sync::Lazy;

pub trait EntryBufferImpl: EntryBufferImplExt + ObjectImpl {
    fn delete_text(&self, entry_buffer: &Self::Type, position: u32, n_chars: Option<u32>) -> u32 {
        self.parent_delete_text(entry_buffer, position, n_chars)
    }

    fn deleted_text(&self, entry_buffer: &Self::Type, position: u32, n_chars: Option<u32>) {
        self.parent_deleted_text(entry_buffer, position, n_chars)
    }

    #[doc(alias = "get_length")]
    fn length(&self, entry_buffer: &Self::Type) -> u32 {
        self.parent_length(entry_buffer)
    }

    #[doc(alias = "get_text")]
    fn text(&self, entry_buffer: &Self::Type) -> GString {
        self.parent_text(entry_buffer)
    }
    fn insert_text(&self, entry_buffer: &Self::Type, position: u32, chars: &str) -> u32 {
        self.parent_insert_text(entry_buffer, position, chars)
    }

    fn inserted_text(&self, entry_buffer: &Self::Type, position: u32, chars: &str) {
        self.parent_inserted_text(entry_buffer, position, chars)
    }
}

pub trait EntryBufferImplExt: ObjectSubclass {
    fn parent_delete_text(
        &self,
        entry_buffer: &Self::Type,
        position: u32,
        n_chars: Option<u32>,
    ) -> u32;
    fn parent_deleted_text(&self, entry_buffer: &Self::Type, position: u32, n_chars: Option<u32>);
    fn parent_length(&self, entry_buffer: &Self::Type) -> u32;
    fn parent_text(&self, entry_buffer: &Self::Type) -> GString;
    fn parent_insert_text(&self, entry_buffer: &Self::Type, position: u32, chars: &str) -> u32;
    fn parent_inserted_text(&self, entry_buffer: &Self::Type, position: u32, chars: &str);
}

impl<T: EntryBufferImpl> EntryBufferImplExt for T {
    fn parent_delete_text(
        &self,
        entry_buffer: &Self::Type,
        position: u32,
        n_chars: Option<u32>,
    ) -> u32 {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkEntryBufferClass;
            let f = (*parent_class)
                .delete_text
                .expect("No parent class impl for \"delete_text\"");
            f(
                entry_buffer
                    .unsafe_cast_ref::<EntryBuffer>()
                    .to_glib_none()
                    .0,
                position,
                n_chars.unwrap_or(u32::MAX),
            )
        }
    }

    fn parent_deleted_text(&self, entry_buffer: &Self::Type, position: u32, n_chars: Option<u32>) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkEntryBufferClass;
            if let Some(f) = (*parent_class).deleted_text {
                f(
                    entry_buffer
                        .unsafe_cast_ref::<EntryBuffer>()
                        .to_glib_none()
                        .0,
                    position,
                    n_chars.unwrap_or(u32::MAX),
                )
            }
        }
    }

    fn parent_length(&self, entry_buffer: &Self::Type) -> u32 {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkEntryBufferClass;
            let f = (*parent_class)
                .get_length
                .expect("No parent class impl for \"get_length\"");
            f(entry_buffer
                .unsafe_cast_ref::<EntryBuffer>()
                .to_glib_none()
                .0)
        }
    }

    fn parent_text(&self, entry_buffer: &Self::Type) -> GString {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkEntryBufferClass;
            let f = (*parent_class)
                .get_text
                .expect("No parent class impl for \"get_text\"");
            let mut n_bytes = 0;
            let res = f(
                entry_buffer
                    .unsafe_cast_ref::<EntryBuffer>()
                    .to_glib_none()
                    .0,
                &mut n_bytes,
            );
            FromGlibContainer::from_glib_none_num(res, n_bytes as usize)
        }
    }

    fn parent_insert_text(&self, entry_buffer: &Self::Type, position: u32, text: &str) -> u32 {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkEntryBufferClass;
            let f = (*parent_class)
                .insert_text
                .expect("No parent class impl for \"insert_text\"");

            f(
                entry_buffer
                    .unsafe_cast_ref::<EntryBuffer>()
                    .to_glib_none()
                    .0,
                position,
                text.to_glib_none().0,
                text.chars().count() as u32,
            )
        }
    }

    fn parent_inserted_text(&self, entry_buffer: &Self::Type, position: u32, text: &str) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkEntryBufferClass;
            if let Some(f) = (*parent_class).inserted_text {
                f(
                    entry_buffer
                        .unsafe_cast_ref::<EntryBuffer>()
                        .to_glib_none()
                        .0,
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
        <Object as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
        klass.delete_text = Some(entry_buffer_delete_text::<T>);
        klass.deleted_text = Some(entry_buffer_deleted_text::<T>);
        klass.get_length = Some(entry_buffer_get_length::<T>);
        klass.get_text = Some(entry_buffer_get_text::<T>);
        klass.insert_text = Some(entry_buffer_insert_text::<T>);
        klass.inserted_text = Some(entry_buffer_inserted_text::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Object as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn entry_buffer_delete_text<T: EntryBufferImpl>(
    ptr: *mut ffi::GtkEntryBuffer,
    position: u32,
    n_chars: u32,
) -> u32 {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<EntryBuffer> = from_glib_borrow(ptr);

    let n_chars = if n_chars == u32::MAX {
        None
    } else {
        Some(n_chars)
    };

    imp.delete_text(wrap.unsafe_cast_ref(), position, n_chars)
}

unsafe extern "C" fn entry_buffer_deleted_text<T: EntryBufferImpl>(
    ptr: *mut ffi::GtkEntryBuffer,
    position: u32,
    n_chars: u32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<EntryBuffer> = from_glib_borrow(ptr);

    let n_chars = if n_chars == u32::MAX {
        None
    } else {
        Some(n_chars)
    };

    imp.deleted_text(wrap.unsafe_cast_ref(), position, n_chars)
}

static GET_TEXT_QUARK: Lazy<glib::Quark> =
    Lazy::new(|| glib::Quark::from_string("gtk4-rs-subclass-entry-buffer-text"));

unsafe extern "C" fn entry_buffer_get_text<T: EntryBufferImpl>(
    ptr: *mut ffi::GtkEntryBuffer,
    n_bytes: *mut usize,
) -> *const libc::c_char {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<EntryBuffer> = from_glib_borrow(ptr);

    let ret = imp.text(wrap.unsafe_cast_ref());
    *n_bytes = ret.len();
    // Ensures that the returned text stays alive for as long as
    // the entry buffer instance
    let fullptr = ret.to_glib_full();
    wrap.set_qdata(*GET_TEXT_QUARK, fullptr);
    fullptr
}

unsafe extern "C" fn entry_buffer_get_length<T: EntryBufferImpl>(
    ptr: *mut ffi::GtkEntryBuffer,
) -> u32 {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<EntryBuffer> = from_glib_borrow(ptr);

    imp.length(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn entry_buffer_insert_text<T: EntryBufferImpl>(
    ptr: *mut ffi::GtkEntryBuffer,
    position: u32,
    charsptr: *const libc::c_char,
    n_chars: u32,
) -> u32 {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<EntryBuffer> = from_glib_borrow(ptr);
    let text: Borrowed<GString> = from_glib_borrow(charsptr);

    let chars = text_n_chars(&text, n_chars);
    imp.insert_text(wrap.unsafe_cast_ref(), position, chars)
}

unsafe extern "C" fn entry_buffer_inserted_text<T: EntryBufferImpl>(
    ptr: *mut ffi::GtkEntryBuffer,
    position: u32,
    charsptr: *const libc::c_char,
    length: u32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<EntryBuffer> = from_glib_borrow(ptr);
    let text: Borrowed<GString> = from_glib_borrow(charsptr);

    let chars = text_n_chars(&text, length);
    imp.inserted_text(wrap.unsafe_cast_ref(), position, &chars)
}

#[doc(alias = "get_text_n_chars")]
fn text_n_chars(text: &str, n_chars: u32) -> &str {
    if n_chars != u32::MAX && n_chars > 0 {
        let mut iter = text
            .char_indices()
            .skip((n_chars - 1) as usize)
            .map(|(pos, _)| pos);
        iter
            .next()
            .expect(
                "The passed text to EntryBuffer contains fewer characters than what's passed as a length",
            );
        let pos_end = iter.next().unwrap_or_else(|| text.len());
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
    #[test]
    fn n_chars_max_length_ascii() {
        assert_eq!(text_n_chars("gtk-rs bindings", 6), "gtk-rs");
        assert_eq!(text_n_chars("gtk-rs bindings", u32::MAX), "gtk-rs bindings");
    }

    #[test]
    #[should_panic]
    fn n_chars_max_length_ascii_panic() {
        assert_eq!(text_n_chars("gtk-rs", 7), "gtk-rs");
    }

    #[test]
    fn n_chars_max_length_utf8() {
        assert_eq!(text_n_chars("👨👩👧👦", 2), "👨👩");
        assert_eq!(text_n_chars("👨👩👧👦", 0), "");
        assert_eq!(text_n_chars("👨👩👧👦", 4), "👨👩👧👦");
        assert_eq!(text_n_chars("👨👩👧👦", u32::MAX), "👨👩👧👦");
        assert_eq!(text_n_chars("كتاب", 2), "كت");
    }

    #[test]
    fn n_chars_max_length_utf8_ascii() {
        assert_eq!(text_n_chars("👨g👩t👧k👦", 2), "👨g");
        assert_eq!(text_n_chars("👨g👩t👧k👦", 5), "👨g👩t👧");
        assert_eq!(text_n_chars("كaتاب", 3), "كaت");
    }

    #[test]
    #[should_panic]
    fn n_chars_max_length_utf8_panic() {
        assert_eq!(text_n_chars("👨👩👧👦", 5), "👨👩");
    }
}
