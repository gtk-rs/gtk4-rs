// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{translate::*, value::FromValue, Value};

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GtkExpressionWatch")]
    pub struct ExpressionWatch(Shared<ffi::GtkExpressionWatch>);

    match fn {
        ref => |ptr| ffi::gtk_expression_watch_ref(ptr),
        unref => |ptr| ffi::gtk_expression_watch_unref(ptr),
    }
}

impl ExpressionWatch {
    #[doc(alias = "gtk_expression_watch_evaluate")]
    pub fn evaluate(&self) -> Option<Value> {
        assert_initialized_main_thread!();
        unsafe {
            let mut value = Value::uninitialized();
            let ret = ffi::gtk_expression_watch_evaluate(
                self.to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            if from_glib(ret) {
                Some(value)
            } else {
                None
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Similar to [`Self::evaluate`] but panics if the value is of a different type.
    #[doc(alias = "gtk_expression_evaluate")]
    pub fn evaluate_as<V: for<'b> FromValue<'b> + 'static>(&self) -> Option<V> {
        self.evaluate().map(|v| {
            v.get_owned::<V>()
                .expect("Failed to evaluate to this value type")
        })
    }

    #[doc(alias = "gtk_expression_watch_unwatch")]
    pub fn unwatch(&self) {
        unsafe { ffi::gtk_expression_watch_unwatch(self.to_glib_none().0) }
    }
}

#[cfg(any(feature = "v4_2", feature = "dox"))]
impl glib::StaticType for ExpressionWatch {
    #[doc(alias = "gtk_expression_watch_get_type")]
    #[inline]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gtk_expression_watch_get_type()) }
    }
}
