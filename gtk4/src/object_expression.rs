// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use glib::{IsA, Object, Value};

define_expression!(
    ObjectExpression,
    ffi::GtkObjectExpression,
    ffi::gtk_object_expression_get_type
);

impl ObjectExpression {
    #[doc(alias = "gtk_property_expression_new")]
    pub fn new<T: IsA<Object>>(object: &T) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_object_expression_new(
                object.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_object_expression_get_object")]
    #[doc(alias = "get_object")]
    pub fn object(&self) -> Option<Object> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_object_expression_get_object(self.to_glib_none().0)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_THREAD_WORKER;

    #[test]
    fn test_object_expression() {
        TEST_THREAD_WORKER
            .push(move || {
                let obj = crate::IconTheme::new();
                let expr = ObjectExpression::new(&obj);
                assert_eq!(expr.object().unwrap(), obj);
            })
            .expect("Failed to schedule a test call");
    }
}
