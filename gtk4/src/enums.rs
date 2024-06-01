// Take a look at the license at the top of the repository in the LICENSE file.

use std::cmp;

use glib::{translate::*, Quark};

use crate::{ffi, prelude::*, CssParserWarning, Ordering};

impl From<cmp::Ordering> for Ordering {
    #[inline]
    fn from(o: cmp::Ordering) -> Self {
        skip_assert_initialized!();
        match o {
            cmp::Ordering::Equal => Self::Equal,
            cmp::Ordering::Greater => Self::Larger,
            cmp::Ordering::Less => Self::Smaller,
        }
    }
}

impl From<Ordering> for cmp::Ordering {
    #[inline]
    fn from(o: Ordering) -> Self {
        skip_assert_initialized!();
        match o {
            Ordering::Equal => Self::Equal,
            Ordering::Larger => Self::Greater,
            Ordering::Smaller => Self::Less,
            Ordering::__Unknown(_) => unreachable!(),
        }
    }
}

impl ErrorDomain for CssParserWarning {
    #[inline]
    fn domain() -> Quark {
        skip_assert_initialized!();
        unsafe { from_glib(ffi::gtk_css_parser_warning_quark()) }
    }

    #[inline]
    fn code(self) -> i32 {
        self.into_glib()
    }

    #[inline]
    fn from(code: i32) -> Option<Self> {
        skip_assert_initialized!();
        match code {
            ffi::GTK_CSS_PARSER_WARNING_DEPRECATED => Some(Self::Deprecated),
            ffi::GTK_CSS_PARSER_WARNING_SYNTAX => Some(Self::Syntax),
            ffi::GTK_CSS_PARSER_WARNING_UNIMPLEMENTED => Some(Self::Unimplemented),
            value => Some(Self::__Unknown(value)),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GtkAlign")]
pub enum Align {
    #[doc(alias = "GTK_ALIGN_FILL")]
    Fill,
    #[doc(alias = "GTK_ALIGN_START")]
    Start,
    #[doc(alias = "GTK_ALIGN_END")]
    End,
    #[doc(alias = "GTK_ALIGN_CENTER")]
    Center,
    #[doc(alias = "GTK_ALIGN_BASELINE")]
    Baseline,
    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "GTK_ALIGN_BASELINE_FILL")]
    BaselineFill,
    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "GTK_ALIGN_BASELINE_CENTER")]
    BaselineCenter,
    #[doc(hidden)]
    __Unknown(i32),
}

impl std::fmt::Display for Align {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Align::{}",
            match *self {
                Self::Fill => "Fill",
                Self::Start => "Start",
                Self::End => "End",
                Self::Center => "Center",
                Self::Baseline => "Baseline",
                #[cfg(feature = "v4_12")]
                Self::BaselineFill => "BaselineFill",
                #[cfg(feature = "v4_12")]
                Self::BaselineCenter => "BaselineCenter",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for Align {
    type GlibType = ffi::GtkAlign;

    #[inline]
    fn into_glib(self) -> ffi::GtkAlign {
        match self {
            Self::Fill => ffi::GTK_ALIGN_FILL,
            Self::Start => ffi::GTK_ALIGN_START,
            Self::End => ffi::GTK_ALIGN_END,
            Self::Center => ffi::GTK_ALIGN_CENTER,
            #[cfg(not(feature = "v4_12"))]
            Self::Baseline => ffi::GTK_ALIGN_BASELINE,
            #[cfg(feature = "v4_12")]
            Self::BaselineFill | Self::Baseline => ffi::GTK_ALIGN_BASELINE_FILL,
            #[cfg(feature = "v4_12")]
            Self::BaselineCenter => ffi::GTK_ALIGN_BASELINE_CENTER,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkAlign> for Align {
    #[inline]
    unsafe fn from_glib(value: ffi::GtkAlign) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::GTK_ALIGN_FILL => Self::Fill,
            ffi::GTK_ALIGN_START => Self::Start,
            ffi::GTK_ALIGN_END => Self::End,
            ffi::GTK_ALIGN_CENTER => Self::Center,
            #[cfg(not(feature = "v4_12"))]
            ffi::GTK_ALIGN_BASELINE => Self::Baseline,
            #[cfg(feature = "v4_12")]
            ffi::GTK_ALIGN_BASELINE_FILL => Self::BaselineFill,
            #[cfg(feature = "v4_12")]
            ffi::GTK_ALIGN_BASELINE_CENTER => Self::BaselineCenter,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for Align {
    #[inline]
    #[doc(alias = "gtk_align_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gtk_align_get_type()) }
    }
}

impl glib::HasParamSpec for Align {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder_with_default
    }
}

impl glib::value::ValueType for Align {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for Align {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl glib::value::ToValue for Align {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<Align> for glib::Value {
    #[inline]
    fn from(v: Align) -> Self {
        skip_assert_initialized!();
        glib::value::ToValue::to_value(&v)
    }
}
