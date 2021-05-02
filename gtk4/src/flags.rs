// Take a look at the license at the top of the repository in the LICENSE file.

use bitflags::bitflags;
use glib::translate::*;
use glib::value::{FromValue, ToValue, ValueType};
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
impl IntoGlib for PrintCapabilities {
    type GlibType = ffi::GtkPrintCapabilities;

    fn into_glib(self) -> ffi::GtkPrintCapabilities {
        self.bits()
    }
}

#[doc(hidden)]
#[cfg(any(target_os = "linux", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(target_os = "linux")))]
impl FromGlib<ffi::GtkPrintCapabilities> for PrintCapabilities {
    unsafe fn from_glib(value: ffi::GtkPrintCapabilities) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
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
impl ValueType for PrintCapabilities {
    type Type = Self;
}

#[cfg(any(target_os = "linux", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(target_os = "linux")))]
unsafe impl<'a> FromValue<'a> for PrintCapabilities {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(target_os = "linux", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(target_os = "linux")))]
impl ToValue for PrintCapabilities {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib())
        }
        value
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}
