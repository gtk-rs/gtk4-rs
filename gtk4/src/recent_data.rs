// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{collections::StrV, translate::*, GStringPtr};
use std::ffi::CStr;

glib::wrapper! {
    #[doc(alias = "GtkRecentData")]
    pub struct RecentData(BoxedInline<ffi::GtkRecentData>);

    match fn {
        init => |ptr| init_recent_data(ptr),
        copy_into => |dest, src| copy_into_recent_data(dest, src),
        clear => |ptr| clear_recent_data(ptr),
    }
}

impl RecentData {
    #[inline]
    pub fn new(
        display_name: Option<&str>,
        description: Option<&str>,
        mime_type: &str,
        app_name: &str,
        app_exec: &str,
        groups: &[&str],
        is_private: bool,
    ) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            Self::unsafe_from(ffi::GtkRecentData {
                display_name: display_name.to_glib_full(),
                description: description.to_glib_full(),
                mime_type: mime_type.to_glib_full(),
                app_name: app_name.to_glib_full(),
                app_exec: app_exec.to_glib_full(),
                groups: groups.to_glib_full(),
                is_private: is_private.into_glib(),
            })
        }
    }

    #[inline]
    pub fn display_name(&self) -> Option<&str> {
        unsafe {
            if self.inner.display_name.is_null() {
                None
            } else {
                CStr::from_ptr(self.inner.display_name).to_str().ok()
            }
        }
    }

    #[inline]
    pub fn description(&self) -> Option<&str> {
        unsafe {
            if self.inner.description.is_null() {
                None
            } else {
                CStr::from_ptr(self.inner.description).to_str().ok()
            }
        }
    }

    #[inline]
    pub fn mime_type(&self) -> &str {
        unsafe { CStr::from_ptr(self.inner.mime_type).to_str().unwrap() }
    }

    #[inline]
    pub fn app_name(&self) -> &str {
        unsafe { CStr::from_ptr(self.inner.app_name).to_str().unwrap() }
    }

    #[inline]
    pub fn app_exec(&self) -> &str {
        unsafe { CStr::from_ptr(self.inner.app_exec).to_str().unwrap() }
    }

    #[inline]
    pub fn groups<'a>(&self) -> &'a [GStringPtr] {
        unsafe { StrV::from_glib_borrow(self.inner.groups as *const _) }
    }

    #[inline]
    pub fn is_private(&self) -> bool {
        unsafe { from_glib(self.inner.is_private) }
    }
}

unsafe fn init_recent_data(recent_data: *mut ffi::GtkRecentData) {
    std::ptr::write(recent_data, std::mem::zeroed());
}

unsafe fn copy_into_recent_data(dest: *mut ffi::GtkRecentData, src: *const ffi::GtkRecentData) {
    init_recent_data(dest);
    (*dest).display_name = glib::ffi::g_strdup((*src).display_name);
    (*dest).description = glib::ffi::g_strdup((*src).description);
    (*dest).mime_type = glib::ffi::g_strdup((*src).mime_type);
    (*dest).app_name = glib::ffi::g_strdup((*src).app_name);
    (*dest).app_exec = glib::ffi::g_strdup((*src).app_exec);
    (*dest).groups = glib::ffi::g_strdupv((*src).groups);
    (*dest).is_private = (*src).is_private;
}

unsafe fn clear_recent_data(recent_data: *mut ffi::GtkRecentData) {
    glib::ffi::g_free((*recent_data).display_name as *mut _);
    glib::ffi::g_free((*recent_data).description as *mut _);
    glib::ffi::g_free((*recent_data).mime_type as *mut _);
    glib::ffi::g_free((*recent_data).app_name as *mut _);
    glib::ffi::g_free((*recent_data).app_exec as *mut _);
    glib::ffi::g_strfreev((*recent_data).groups as *mut _);
}
