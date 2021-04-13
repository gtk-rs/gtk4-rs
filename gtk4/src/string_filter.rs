// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Expression, StringFilter};
use glib::translate::*;

impl StringFilter {
    #[doc(alias = "gtk_string_filter_new")]
    pub fn new<E: AsRef<Expression>>(expression: Option<&E>) -> StringFilter {
        assert_initialized_main_thread!();
        if let Some(e) = expression {
            if !e.as_ref().value_type().is_a(glib::Type::STRING) {
                panic!("StringFilter::new must take either None or an expression that evaluates to a string.");
            }
        }
        unsafe {
            from_glib_full(ffi::gtk_string_filter_new(
                expression.map(|e| e.as_ref()).to_glib_full(),
            ))
        }
    }

    #[doc(alias = "gtk_string_filter_set_expression")]
    pub fn set_expression<E: AsRef<Expression>>(&self, expression: Option<&E>) {
        if let Some(e) = expression {
            if !e.as_ref().value_type().is_a(glib::Type::STRING) {
                panic!("StringFilter::set_expression must take either None or an expression that evaluates to a string.");
            }
        }
        unsafe {
            ffi::gtk_string_filter_set_expression(
                self.to_glib_none().0,
                expression.map(|e| e.as_ref()).to_glib_none().0,
            );
        }
    }
}
