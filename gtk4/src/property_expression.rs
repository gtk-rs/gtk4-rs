// Take a look at the license at the top of the repository in the LICENSE file.

use crate::expression::Expression;
use glib::translate::*;
use glib::{Type, Value};

define_expression!(
    PropertyExpression,
    ffi::GtkPropertyExpression,
    ffi::gtk_property_expression_get_type
);

impl PropertyExpression {
    #[doc(alias = "gtk_property_expression_new")]
    pub fn new<E: AsRef<Expression>>(
        this_type: Type,
        expression: Option<&E>,
        property_name: &str,
    ) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_property_expression_new(
                this_type.into_glib(),
                expression.map(|e| e.as_ref()).to_glib_full(),
                property_name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_property_expression_new_for_pspec")]
    #[doc(alias = "new_for_pspec")]
    pub fn for_pspec<E: AsRef<Expression>>(expression: Option<&E>, pspec: glib::ParamSpec) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_property_expression_new_for_pspec(
                expression.map(|e| e.as_ref()).to_glib_full(),
                pspec.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_property_expression_get_expression")]
    #[doc(alias = "get_expression")]
    pub fn expression(&self) -> Option<Expression> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_property_expression_get_expression(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_property_expression_get_pspec")]
    #[doc(alias = "get_pspec")]
    pub fn pspec(&self) -> glib::ParamSpec {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_property_expression_get_pspec(
                self.to_glib_none().0,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NONE_EXPRESSION, TEST_THREAD_WORKER};
    use glib::StaticType;

    #[test]
    fn test_property_expression() {
        TEST_THREAD_WORKER
            .push(move || {
                let _prop_expr = PropertyExpression::new(
                    crate::StringObject::static_type(),
                    NONE_EXPRESSION,
                    "string",
                );
            })
            .expect("Failed to schedule a test call");
    }
}
