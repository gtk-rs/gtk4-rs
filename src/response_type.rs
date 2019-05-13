use ffi;
use glib::translate::{FromGlib, ToGlib, ToGlibPtr, ToGlibPtrMut, from_glib};
use glib::value::{FromValue, FromValueOptional, SetValue};
use glib::{StaticType, Type, Value};
use gobject_ffi;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ResponseType {
    None,
    Reject,
    Accept,
    DeleteEvent,
    Ok,
    Cancel,
    Close,
    Yes,
    No,
    Apply,
    Help,
    Other(u16),
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for ResponseType {
    type GlibType = ffi::GtkResponseType;

    fn to_glib(&self) -> ffi::GtkResponseType {
        match *self {
            ResponseType::None => ffi::GTK_RESPONSE_NONE,
            ResponseType::Reject => ffi::GTK_RESPONSE_REJECT,
            ResponseType::Accept => ffi::GTK_RESPONSE_ACCEPT,
            ResponseType::DeleteEvent => ffi::GTK_RESPONSE_DELETE_EVENT,
            ResponseType::Ok => ffi::GTK_RESPONSE_OK,
            ResponseType::Cancel => ffi::GTK_RESPONSE_CANCEL,
            ResponseType::Close => ffi::GTK_RESPONSE_CLOSE,
            ResponseType::Yes => ffi::GTK_RESPONSE_YES,
            ResponseType::No => ffi::GTK_RESPONSE_NO,
            ResponseType::Apply => ffi::GTK_RESPONSE_APPLY,
            ResponseType::Help => ffi::GTK_RESPONSE_HELP,
            ResponseType::Other(value) => value as ffi::GtkResponseType,
            ResponseType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkResponseType> for ResponseType {
    fn from_glib(value: ffi::GtkResponseType) -> Self {
        skip_assert_initialized!();
        match value {
            -1 => ResponseType::None,
            -2 => ResponseType::Reject,
            -3 => ResponseType::Accept,
            -4 => ResponseType::DeleteEvent,
            -5 => ResponseType::Ok,
            -6 => ResponseType::Cancel,
            -7 => ResponseType::Close,
            -8 => ResponseType::Yes,
            -9 => ResponseType::No,
            -10 => ResponseType::Apply,
            -11 => ResponseType::Help,
            value if value >= 0 && value <= ::std::u16::MAX as i32 => ResponseType::Other(value as u16),
            value => ResponseType::__Unknown(value),
        }
    }
}

impl fmt::Display for ResponseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ResponseType::{}", match *self {
            ResponseType::None => "None",
            ResponseType::Reject => "Reject",
            ResponseType::Accept => "Accept",
            ResponseType::DeleteEvent => "DeleteEvent",
            ResponseType::Ok => "Ok",
            ResponseType::Cancel => "Cancel",
            ResponseType::Close => "Close",
            ResponseType::Yes => "Yes",
            ResponseType::No => "No",
            ResponseType::Apply => "Apply",
            ResponseType::Help => "Help",
            ResponseType::Other(_) => "Other",
            ResponseType::__Unknown(_) => "Unknown",
        })
    }
}

impl StaticType for ResponseType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_response_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ResponseType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ResponseType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for ResponseType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}
