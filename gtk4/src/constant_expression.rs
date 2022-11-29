// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ConstantExpression;
use glib::translate::*;
use glib::{value::FromValue, ToValue};

define_expression!(ConstantExpression, ffi::GtkConstantExpression);

impl ConstantExpression {
    #[doc(alias = "gtk_constant_expression_new")]
    #[doc(alias = "gtk_constant_expression_new_for_value")]
    pub fn new(value: impl Into<glib::Value>) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_constant_expression_new_for_value(
                value.into().to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Similar to [`Self::value`] but panics if the value is of a different type.
    #[doc(alias = "gtk_constant_expression_get_value")]
    #[doc(alias = "get_value")]
    pub fn value_as<V: for<'b> FromValue<'b> + 'static>(&self) -> V {
        let value = self.value();
        value
            .get_owned::<V>()
            .expect("Failed to get ConstantExpression value")
    }
}

impl std::fmt::Debug for ConstantExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConstantExpression")
            .field("value_type", &self.value_type())
            .field("is_static", &self.is_static())
            .field("value", &self.value())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as gtk4;

    #[test]
    fn test_expressions() {
        let expr1 = ConstantExpression::new(23);
        assert_eq!(expr1.value().get::<i32>().unwrap(), 23);
        let expr2 = ConstantExpression::for_value(&"hello".to_value());
        assert_eq!(expr2.value().get::<String>().unwrap(), "hello");
        let expr1 = ConstantExpression::new(23);
        assert_eq!(expr1.value().get::<i32>().unwrap(), 23);
        assert_eq!(expr1.value_as::<i32>(), 23);
        let expr2 = ConstantExpression::for_value(&"hello".to_value());
        assert_eq!(expr2.value().get::<String>().unwrap(), "hello");
        assert_eq!(expr2.value_as::<String>(), "hello");

        assert!(expr1.is::<ConstantExpression>());
    }
}
