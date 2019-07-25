// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use gtk_sys;
use std::ffi::CStr;
use std::ptr;

pub struct FileFilterInfo {
    filename: Option<String>,
    uri: Option<String>,
    display_name: Option<String>,
    mime_type: Option<String>,
}

impl FileFilterInfo {
    pub fn new(
        filename: Option<&str>,
        uri: Option<&str>,
        display_name: Option<&str>,
        mime_type: Option<&str>,
    ) -> FileFilterInfo {
        assert_initialized_main_thread!();
        FileFilterInfo {
            filename: filename.map(ToOwned::to_owned),
            uri: uri.map(ToOwned::to_owned),
            display_name: display_name.map(ToOwned::to_owned),
            mime_type: mime_type.map(ToOwned::to_owned),
        }
    }

    pub fn get_filename(&self) -> Option<&str> {
        self.filename.as_ref().map(AsRef::as_ref)
    }

    pub fn get_uri(&self) -> Option<&str> {
        self.uri.as_ref().map(AsRef::as_ref)
    }

    pub fn get_display_name(&self) -> Option<&str> {
        self.display_name.as_ref().map(AsRef::as_ref)
    }

    pub fn get_mime_type(&self) -> Option<&str> {
        self.mime_type.as_ref().map(AsRef::as_ref)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const gtk_sys::GtkFileFilterInfo> for FileFilterInfo {
    unsafe fn from_glib_borrow(ptr: *const gtk_sys::GtkFileFilterInfo) -> Self {
        FileFilterInfo {
            filename: if (*ptr).contains & gtk_sys::GTK_FILE_FILTER_FILENAME != 0 {
                Some(from_glib_none((*ptr).filename))
            } else {
                None
            },
            uri: if (*ptr).contains & gtk_sys::GTK_FILE_FILTER_URI != 0 {
                Some(from_glib_none((*ptr).uri))
            } else {
                None
            },
            display_name: if (*ptr).contains & gtk_sys::GTK_FILE_FILTER_DISPLAY_NAME != 0 {
                Some(from_glib_none((*ptr).display_name))
            } else {
                None
            },
            mime_type: if (*ptr).contains & gtk_sys::GTK_FILE_FILTER_MIME_TYPE != 0 {
                Some(from_glib_none((*ptr).mime_type))
            } else {
                None
            },
        }
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const gtk_sys::GtkFileFilterInfo> for FileFilterInfo {
    type Storage = (&'a Self, Box<gtk_sys::GtkFileFilterInfo>);

    fn to_glib_none(&'a self) -> Stash<*const gtk_sys::GtkFileFilterInfo, Self> {
        fn to_c_str(s: &Option<String>) -> *const i8 {
            s.as_ref().map_or_else(
                || ptr::null(),
                |t| {
                    CStr::from_bytes_with_nul(t.as_bytes())
                        .expect("Unexpected nul")
                        .as_ptr()
                },
            )
        }

        let sys = Box::new(gtk_sys::GtkFileFilterInfo {
            contains: self
                .filename
                .as_ref()
                .map_or_else(|| 0, |_| gtk_sys::GTK_FILE_FILTER_FILENAME)
                | self
                    .uri
                    .as_ref()
                    .map_or_else(|| 0, |_| gtk_sys::GTK_FILE_FILTER_URI)
                | self
                    .display_name
                    .as_ref()
                    .map_or_else(|| 0, |_| gtk_sys::GTK_FILE_FILTER_DISPLAY_NAME)
                | self
                    .mime_type
                    .as_ref()
                    .map_or_else(|| 0, |_| gtk_sys::GTK_FILE_FILTER_MIME_TYPE),
            filename: to_c_str(&self.filename),
            uri: to_c_str(&self.uri),
            display_name: to_c_str(&self.display_name),
            mime_type: to_c_str(&self.mime_type),
        });

        Stash(&*sys, (self, sys))
    }
}
