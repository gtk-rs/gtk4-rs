// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use glib::{ToValue, Value};

define_expression!(
    ConstantExpression,
    ffi::GtkConstantExpression,
    ffi::gtk_constant_expression_get_type
);

impl ConstantExpression {
    #[doc(alias = "gtk_constant_expression_new")]
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_THREAD_WORKER;

    #[test]
    fn test_expressions() {
        TEST_THREAD_WORKER
            .push(move || {
                let expr1 = ConstantExpression::new(&23);
                assert_eq!(expr1.value().get::<i32>().unwrap(), 23);
                let expr2 = ConstantExpression::for_value(&"hello".to_value());
                assert_eq!(expr2.value().get::<String>().unwrap(), "hello");
                let expr1 = ConstantExpression::new(&23);
                assert_eq!(expr1.value().get::<i32>().unwrap(), 23);
                let expr2 = ConstantExpression::for_value(&"hello".to_value());
                assert_eq!(expr2.value().get::<String>().unwrap(), "hello");
            })
            .expect("Failed to schedule a test call");
    }
}
