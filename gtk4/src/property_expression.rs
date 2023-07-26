// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, Expression, PropertyExpression};
use glib::{translate::*, Object};

define_expression!(PropertyExpression, ffi::GtkPropertyExpression);

impl PropertyExpression {
    #[doc(alias = "gtk_property_expression_new")]
    pub fn new<T: IsA<Object>>(
        expression: Option<impl AsRef<Expression>>,
        property_name: &str,
    ) -> PropertyExpression {
        assert_initialized_main_thread!();
        Self::with_type(T::static_type(), expression, property_name)
    }

    #[doc(alias = "gtk_property_expression_new")]
    pub fn with_type(
        this_type: glib::Type,
        expression: Option<impl AsRef<Expression>>,
        property_name: &str,
    ) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_property_expression_new(
                this_type.into_glib(),
                expression
                    .map(|p| p.as_ref().clone().upcast())
                    .into_glib_ptr(),
                property_name.to_glib_none().0,
            ))
        }
    }
}

impl std::fmt::Debug for PropertyExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PropertyExpression")
            .field("value_type", &self.value_type())
            .field("is_static", &self.is_static())
            .field("pspec", &self.pspec())
            .field("expression", &self.expression())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as gtk4;

    #[test]
    fn test_property_expression() {
        let prop_expr =
            PropertyExpression::new::<crate::StringObject>(crate::Expression::NONE, "string");
        assert!(prop_expr.is::<PropertyExpression>());
    }
}
