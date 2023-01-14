// Take a look at the license at the top of the repository in the LICENSE file.

use crate::PadActionType;

use glib::translate::*;
use std::ffi::CStr;

glib::wrapper! {
    #[doc(alias = "GtkPadActionEntry")]
    pub struct PadActionEntry(BoxedInline<ffi::GtkPadActionEntry>);

    match fn {
        init => |ptr| init_action_entry(ptr),
        copy_into => |dest, src| copy_into_action_entry(dest, src),
        clear => |ptr| clear_action_entry(ptr),
    }
}

impl PadActionEntry {
    #[inline]
    pub fn new(
        type_: PadActionType,
        index: i32,
        mode: i32,
        label: &str,
        action_name: &str,
    ) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            Self::unsafe_from(ffi::GtkPadActionEntry {
                type_: type_.into_glib(),
                index,
                mode,
                label: label.to_glib_full(),
                action_name: action_name.to_glib_full(),
            })
        }
    }

    #[inline]
    pub fn type_(&self) -> PadActionType {
        unsafe { from_glib(self.inner.type_) }
    }

    #[inline]
    pub fn index(&self) -> i32 {
        self.inner.index
    }

    #[inline]
    pub fn mode(&self) -> i32 {
        self.inner.mode
    }

    #[inline]
    pub fn label(&self) -> &str {
        unsafe { CStr::from_ptr(self.inner.label).to_str().unwrap() }
    }

    #[inline]
    pub fn action_name(&self) -> &str {
        unsafe { CStr::from_ptr(self.inner.action_name).to_str().unwrap() }
    }
}
unsafe fn init_action_entry(action_entry: *mut ffi::GtkPadActionEntry) {
    std::ptr::write(action_entry, std::mem::zeroed());
}

unsafe fn copy_into_action_entry(
    dest: *mut ffi::GtkPadActionEntry,
    src: *const ffi::GtkPadActionEntry,
) {
    init_action_entry(dest);
    (*dest).action_name = glib::ffi::g_strdup((*src).action_name);
    (*dest).label = glib::ffi::g_strdup((*src).label);
    (*dest).type_ = (*src).type_;
    (*dest).index = (*src).index;
    (*dest).mode = (*src).mode;
}

unsafe fn clear_action_entry(action_entry: *mut ffi::GtkPadActionEntry) {
    glib::ffi::g_free((*action_entry).label as *mut _);
    glib::ffi::g_free((*action_entry).action_name as *mut _);
}
