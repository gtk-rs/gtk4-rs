// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ClosureExpression, Expression};
use glib::{translate::*, value::ValueType, StaticType, Value};

define_expression!(ClosureExpression, ffi::GtkClosureExpression);

impl ClosureExpression {
    #[doc(alias = "gtk_closure_expression_new")]
    pub fn new<R>(
        params: impl IntoIterator<Item = impl AsRef<Expression>>,
        closure: glib::RustClosure,
    ) -> Self
    where
        R: ValueType,
    {
        assert_initialized_main_thread!();

        let params = params
            .into_iter()
            .map(|e| e.as_ref().clone())
            .collect::<Vec<_>>();

        unsafe {
            from_glib_full(ffi::gtk_closure_expression_new(
                R::Type::static_type().into_glib(),
                closure.as_ref().to_glib_none().0,
                params.len() as u32,
                params.to_glib_full(),
            ))
        }
    }

    #[doc(alias = "gtk_closure_expression_new")]
    pub fn with_callback<R, F>(
        params: impl IntoIterator<Item = impl AsRef<Expression>>,
        callback: F,
    ) -> Self
    where
        F: Fn(&[Value]) -> R + 'static,
        R: ValueType,
    {
        assert_initialized_main_thread!();
        let closure = glib::Closure::new_local(move |values| {
            let ret = callback(values);
            Some(ret.to_value())
        });

        let params = params
            .into_iter()
            .map(|e| e.as_ref().clone())
            .collect::<Vec<_>>();

        unsafe {
            from_glib_full(ffi::gtk_closure_expression_new(
                R::Type::static_type().into_glib(),
                closure.to_glib_none().0,
                params.len() as u32,
                params.to_glib_full(),
            ))
        }
    }
}
