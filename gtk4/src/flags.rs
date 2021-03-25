// Take a look at the license at the top of the repository in the LICENSE file.

use bitflags::bitflags;
use glib::translate::*;
use glib::value::{FromValue, FromValueOptional, SetValue};
use glib::{StaticType, Type};
use std::fmt;

#[cfg(any(target_os = "linux", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(target_os = "linux")))]
bitflags! {
    pub struct PrintCapabilities: u32 {
        const PAGE_SET = 1;
        const COPIES = 2;
        const COLLATE = 4;
        const REVERSE = 8;
        const SCALE = 16;
        const GENERATE_PDF = 32;
        const GENERATE_PS = 64;
        const PREVIEW = 128;
        const NUMBER_UP = 256;
        const NUMBER_UP_LAYOUT = 512;
    }
}

#[cfg(any(target_os = "linux", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(target_os = "linux")))]
impl fmt::Display for PrintCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
#[cfg(any(target_os = "linux", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(target_os = "linux")))]
impl ToGlib for PrintCapabilities {
    type GlibType = ffi::GtkPrintCapabilities;

    fn to_glib(&self) -> ffi::GtkPrintCapabilities {
        self.bits()
    }
}

#[doc(hidden)]
#[cfg(any(target_os = "linux", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(target_os = "linux")))]
impl FromGlib<ffi::GtkPrintCapabilities> for PrintCapabilities {
    unsafe fn from_glib(value: ffi::GtkPrintCapabilities) -> PrintCapabilities {
        skip_assert_initialized!();
        PrintCapabilities::from_bits_truncate(value)
    }
}

#[cfg(any(target_os = "linux", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(target_os = "linux")))]
impl StaticType for PrintCapabilities {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_print_capabilities_get_type()) }
    }
}

#[cfg(any(target_os = "linux", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(target_os = "linux")))]
impl<'a> FromValueOptional<'a> for PrintCapabilities {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(target_os = "linux", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(target_os = "linux")))]
impl<'a> FromValue<'a> for PrintCapabilities {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(target_os = "linux", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(target_os = "linux")))]
impl SetValue for PrintCapabilities {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}
