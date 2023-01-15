// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use glib::{
    value::{FromValue, ToValue, ValueType},
    StaticType, Type, Value,
};
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[doc(alias = "GtkResponseType")]
pub enum ResponseType {
    #[doc(alias = "GTK_RESPONSE_NONE")]
    None,
    #[doc(alias = "GTK_RESPONSE_REJECT")]
    Reject,
    #[doc(alias = "GTK_RESPONSE_ACCEPT")]
    Accept,
    #[doc(alias = "GTK_RESPONSE_DELETE_EVENT")]
    DeleteEvent,
    #[doc(alias = "GTK_RESPONSE_OK")]
    Ok,
    #[doc(alias = "GTK_RESPONSE_CANCEL")]
    Cancel,
    #[doc(alias = "GTK_RESPONSE_CLOSE")]
    Close,
    #[doc(alias = "GTK_RESPONSE_YES")]
    Yes,
    #[doc(alias = "GTK_RESPONSE_NO")]
    No,
    #[doc(alias = "GTK_RESPONSE_APPLY")]
    Apply,
    #[doc(alias = "GTK_RESPONSE_HELP")]
    Help,
    Other(u16),
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for ResponseType {
    type GlibType = ffi::GtkResponseType;

    #[inline]
    fn into_glib(self) -> ffi::GtkResponseType {
        match self {
            Self::None => ffi::GTK_RESPONSE_NONE,
            Self::Reject => ffi::GTK_RESPONSE_REJECT,
            Self::Accept => ffi::GTK_RESPONSE_ACCEPT,
            Self::DeleteEvent => ffi::GTK_RESPONSE_DELETE_EVENT,
            Self::Ok => ffi::GTK_RESPONSE_OK,
            Self::Cancel => ffi::GTK_RESPONSE_CANCEL,
            Self::Close => ffi::GTK_RESPONSE_CLOSE,
            Self::Yes => ffi::GTK_RESPONSE_YES,
            Self::No => ffi::GTK_RESPONSE_NO,
            Self::Apply => ffi::GTK_RESPONSE_APPLY,
            Self::Help => ffi::GTK_RESPONSE_HELP,
            Self::Other(value) => value as ffi::GtkResponseType,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkResponseType> for ResponseType {
    #[inline]
    unsafe fn from_glib(value: ffi::GtkResponseType) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GTK_RESPONSE_NONE => Self::None,
            ffi::GTK_RESPONSE_REJECT => Self::Reject,
            ffi::GTK_RESPONSE_ACCEPT => Self::Accept,
            ffi::GTK_RESPONSE_DELETE_EVENT => Self::DeleteEvent,
            ffi::GTK_RESPONSE_OK => Self::Ok,
            ffi::GTK_RESPONSE_CANCEL => Self::Cancel,
            ffi::GTK_RESPONSE_CLOSE => Self::Close,
            ffi::GTK_RESPONSE_YES => Self::Yes,
            ffi::GTK_RESPONSE_NO => Self::No,
            ffi::GTK_RESPONSE_APPLY => Self::Apply,
            ffi::GTK_RESPONSE_HELP => Self::Help,
            value if value >= 0 && value <= ::std::u16::MAX as i32 => Self::Other(value as u16),
            value => Self::__Unknown(value),
        }
    }
}

impl fmt::Display for ResponseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ResponseType::{}",
            match *self {
                Self::None => "None",
                Self::Reject => "Reject",
                Self::Accept => "Accept",
                Self::DeleteEvent => "DeleteEvent",
                Self::Ok => "Ok",
                Self::Cancel => "Cancel",
                Self::Close => "Close",
                Self::Yes => "Yes",
                Self::No => "No",
                Self::Apply => "Apply",
                Self::Help => "Help",
                Self::Other(_) => "Other",
                Self::__Unknown(_) => "Unknown",
            }
        )
    }
}

impl StaticType for ResponseType {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_response_type_get_type()) }
    }
}

impl ValueType for ResponseType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for ResponseType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for ResponseType {
    #[inline]
    fn to_value(&self) -> Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe { glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib()) }
        value
    }

    #[inline]
    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl From<ResponseType> for Value {
    #[inline]
    fn from(t: ResponseType) -> Self {
        skip_assert_initialized!();
        t.to_value()
    }
}

impl PartialEq<i32> for ResponseType {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        self.into_glib().eq(other)
    }
}

impl PartialEq<ResponseType> for i32 {
    #[inline]
    fn eq(&self, other: &ResponseType) -> bool {
        other.into_glib().eq(self)
    }
}

impl From<i32> for ResponseType {
    #[inline]
    fn from(response: i32) -> Self {
        skip_assert_initialized!();
        unsafe { Self::from_glib(response) }
    }
}

impl From<ResponseType> for i32 {
    #[inline]
    fn from(r: ResponseType) -> Self {
        skip_assert_initialized!();
        r.into_glib()
    }
}
