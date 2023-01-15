// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{CssParserWarning, Ordering};
use glib::{error::ErrorDomain, translate::*, Quark};
use std::cmp;

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
