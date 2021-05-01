// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Expression;
use glib::translate::*;
use glib::{value::ValueType, StaticType, Value};

define_expression!(
    ClosureExpression,
    ffi::GtkClosureExpression,
    ffi::gtk_closure_expression_get_type
);

impl ClosureExpression {
    #[doc(alias = "gtk_closure_expression_new")]
    pub fn new<F, R>(callback: F, params: &[Expression]) -> Self
    where
        F: Fn(&[Value]) -> R + 'static,
        R: ValueType,
    {
        assert_initialized_main_thread!();
        let closure = glib::Closure::new_local(move |values| {
            let ret = callback(values);
            Some(ret.to_value())
        });
        unsafe {
            from_glib_full(ffi::gtk_closure_expression_new(
                R::Type::static_type().into_glib(),
                closure.to_glib_none().0,
                params.len() as u32,
                params.to_glib_full(),
            ))
        }
    }

    #[doc(alias = "gtk_closure_expression_new")]
    pub fn with_closure<R>(closure: glib::Closure, params: &[Expression]) -> Self
    where
        R: ValueType,
    {
        assert_initialized_main_thread!();
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
