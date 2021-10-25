// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Expression;
use glib::translate::*;
use glib::{value::ValueType, StaticType, Value};

glib::wrapper! {
    #[derive(Debug)]
    #[doc(alias = "GtkClosureExpression")]
    pub struct ClosureExpression(Shared<ffi::GtkClosureExpression>);

    match fn {
        ref => |ptr| ffi::gtk_expression_ref(ptr as *mut ffi::GtkExpression),
        unref => |ptr| ffi::gtk_expression_unref(ptr as *mut ffi::GtkExpression),
    }
}

define_expression!(
    ClosureExpression,
    ffi::GtkClosureExpression,
    ffi::gtk_closure_expression_get_type
);

impl ClosureExpression {
    #[doc(alias = "gtk_closure_expression_new")]
    pub fn new<F, R>(params: &[Expression], callback: F) -> Self
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
    pub fn with_closure<R>(params: &[Expression], closure: glib::Closure) -> Self
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
