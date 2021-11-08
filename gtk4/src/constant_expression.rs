// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use glib::{value::FromValue, ToValue, Value};

glib::wrapper! {
    #[derive(Debug)]
    #[doc(alias = "GtkConstantExpression")]
    pub struct ConstantExpression(Shared<ffi::GtkConstantExpression>);

    match fn {
        ref => |ptr| ffi::gtk_expression_ref(ptr as *mut ffi::GtkExpression),
        unref => |ptr| ffi::gtk_expression_unref(ptr as *mut ffi::GtkExpression),
    }
}

define_expression!(
    ConstantExpression,
    ffi::GtkConstantExpression,
    ffi::gtk_constant_expression_get_type
);

impl ConstantExpression {
    #[doc(alias = "gtk_constant_expression_new")]
    #[doc(alias = "gtk_constant_expression_new_for_value")]
    pub fn new<V: ToValue>(value: &V) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_constant_expression_new_for_value(
                value.to_value().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_constant_expression_new_for_value")]
    #[doc(alias = "new_for_value")]
    pub fn for_value(value: &Value) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_constant_expression_new_for_value(
                value.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_constant_expression_get_value")]
    #[doc(alias = "get_value")]
    pub fn value(&self) -> Value {
        unsafe {
            from_glib_none(ffi::gtk_constant_expression_get_value(
                self.to_glib_none().0,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_synced;

    #[test]
    fn test_expressions() {
        test_synced(move || {
            let expr1 = ConstantExpression::new(&23);
            assert_eq!(expr1.value().get::<i32>().unwrap(), 23);
            let expr2 = ConstantExpression::for_value(&"hello".to_value());
            assert_eq!(expr2.value().get::<String>().unwrap(), "hello");
            let expr1 = ConstantExpression::new(&23);
            assert_eq!(expr1.value().get::<i32>().unwrap(), 23);
            assert_eq!(expr1.value_as::<i32>(), 23);
            let expr2 = ConstantExpression::for_value(&"hello".to_value());
            assert_eq!(expr2.value().get::<String>().unwrap(), "hello");
            assert_eq!(expr2.value_as::<String>(), "hello");
        });
    }
}
