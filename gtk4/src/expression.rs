// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{translate::*, ToValue};
use glib::{IsA, Object, StaticType, Type, Value};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Expression(Shared<ffi::GtkExpression>);

    match fn {
        ref => |ptr| ffi::gtk_expression_ref(ptr),
        unref => |ptr| ffi::gtk_expression_unref(ptr),
    }
}

impl glib::StaticType for Expression {
    #[doc(alias = "gtk_expression_get_type")]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_expression_get_type()) }
    }
}

#[doc(hidden)]
impl AsRef<Expression> for Expression {
    fn as_ref(&self) -> &Expression {
        &self
    }
}

pub const NONE_EXPRESSION: Option<&Expression> = None;

pub unsafe trait IsExpression:
    glib::StaticType + FromGlibPtrFull<*mut ffi::GtkExpression> + 'static
{
}

impl Expression {
    pub fn downcast<E: IsExpression>(self) -> Result<E, Expression> {
        unsafe {
            if self.type_() == E::static_type() {
                Ok(from_glib_full(self.to_glib_full()))
            } else {
                Err(self)
            }
        }
    }

    pub fn downcast_ref<E: IsExpression>(&self) -> Option<&E> {
        unsafe {
            if self.type_() == E::static_type() {
                Some(&*(self as *const Expression as *const E))
            } else {
                None
            }
        }
    }
    pub fn type_(&self) -> Type {
        unsafe {
            let ptr = self.to_glib_none().0;

            from_glib((*(*(ptr as *mut glib::gobject_ffi::GTypeInstance)).g_class).g_type)
        }
    }

    #[doc(alias = "gtk_expression_get_value_type")]
    pub fn value_type(&self) -> Type {
        assert_initialized_main_thread!();
        unsafe { from_glib(ffi::gtk_expression_get_value_type(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_expression_is_static")]
    pub fn is_static(&self) -> bool {
        assert_initialized_main_thread!();
        unsafe { from_glib(ffi::gtk_expression_is_static(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_expression_bind")]
    pub fn bind<T: IsA<Object>>(
        &self,
        target: &T,
        property_name: &str,
        this: Option<&T>,
    ) -> ExpressionWatch {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_expression_bind(
                self.to_glib_full(),
                target.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                this.map(|t| t.as_ref()).to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_expression_evaluate")]
    pub fn evaluate<T: IsA<Object>>(&self, this: Option<&T>) -> Option<Value> {
        assert_initialized_main_thread!();
        unsafe {
            let mut value = Value::uninitialized();
            let ret = ffi::gtk_expression_evaluate(
                self.to_glib_none().0,
                this.map(|t| t.as_ref()).to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            if from_glib(ret) {
                Some(value)
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_expression_watch")]
    pub fn watch<T: IsA<Object>, F: Fn() + 'static>(
        &self,
        this: Option<&T>,
        notify: F,
    ) -> ExpressionWatch {
        assert_initialized_main_thread!();
        unsafe extern "C" fn notify_trampoline<F: Fn() + 'static>(user_data: glib::ffi::gpointer) {
            let f: &F = &*(user_data as *const F);
            f()
        }
        unsafe extern "C" fn destroy_func<F: Fn() + 'static>(user_data: glib::ffi::gpointer) {
            let _callback: Box_<Option<Box_<F>>> = Box_::from_raw(user_data as *mut _);
        }
        let callback_data: Box_<F> = Box_::new(notify);
        unsafe {
            from_glib_none(ffi::gtk_expression_watch(
                self.to_glib_none().0,
                this.map(|t| t.as_ref()).to_glib_none().0,
                Some(notify_trampoline::<F> as _),
                Box_::into_raw(callback_data) as *mut _,
                Some(destroy_func::<F> as _),
            ))
        }
    }
}

impl glib::value::ValueType for Expression {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for Expression {
    type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a Value) -> Self {
        skip_assert_initialized!();
        from_glib_full(ffi::gtk_value_dup_expression(value.to_glib_none().0))
    }
}

impl glib::value::ToValue for Expression {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Expression>();
        unsafe { ffi::gtk_value_set_expression(value.to_glib_none_mut().0, self.to_glib_none().0) }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl glib::value::ToValueOptional for Expression {
    fn to_value_optional(s: Option<&Self>) -> glib::Value {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<Expression>();
        unsafe { ffi::gtk_value_set_expression(value.to_glib_none_mut().0, s.to_glib_none().0) }
        value
    }
}

/// Register Expression's ParamSpec support
trait GtkParamSpecExt {
    fn expression(name: &str, nick: &str, blurb: &str, flags: glib::ParamFlags) -> Self;
}

impl GtkParamSpecExt for glib::ParamSpec {
    #[doc(alias = "gtk_param_spec_expression")]
    fn expression(name: &str, nick: &str, blurb: &str, flags: glib::ParamFlags) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_param_spec_expression(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                flags.to_glib(),
            ))
        }
    }
}

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    #[doc(alias = "gtk_expression_watch_unwatch")]
    pub fn unwatch(&self) {
        unsafe { ffi::gtk_expression_watch_unwatch(self.to_glib_none().0) }
    }
}

#[cfg(any(feature = "v4_2", feature = "dox"))]
impl glib::StaticType for ExpressionWatch {
    #[doc(alias = "gtk_expression_watch_get_type")]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_expression_watch_get_type()) }
    }
}

macro_rules! define_expression {
    ($rust_type:ident, $ffi_type:path, $get_type:path) => {
        glib::wrapper! {
            #[derive(Debug)]
            pub struct $rust_type(Shared<$ffi_type>);

            match fn {
                ref => |ptr| ffi::gtk_expression_ref(ptr as *mut ffi::GtkExpression) as *mut $ffi_type,
                unref => |ptr| ffi::gtk_expression_unref(ptr as *mut ffi::GtkExpression),
            }
        }

        impl std::ops::Deref for $rust_type {
            type Target = Expression;

            fn deref(&self) -> &Self::Target {
                unsafe { &*(self as *const $rust_type as *const Expression) }
            }
        }

        impl AsRef<Expression> for $rust_type {
            fn as_ref(&self) -> &Expression {
                self.upcast_ref()
            }
        }

        impl $rust_type {
            pub fn upcast(self) -> Expression {
                unsafe { std::mem::transmute(self) }
            }

            pub fn upcast_ref(&self) -> &Expression {
                &*self
            }
        }

        #[doc(hidden)]
        impl FromGlibPtrFull<*mut ffi::GtkExpression> for $rust_type {
            unsafe fn from_glib_full(ptr: *mut ffi::GtkExpression) -> Self {
                from_glib_full(ptr as *mut $ffi_type)
            }
        }

        impl glib::StaticType for $rust_type {
            fn static_type() -> Type {
                unsafe {
                    from_glib($get_type())
                }
            }
        }

        unsafe impl IsExpression for $rust_type {}

        impl glib::value::ValueType for $rust_type {
            type Type = Self;
        }

        unsafe impl<'a> glib::value::FromValue<'a> for $rust_type {
            type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

            unsafe fn from_value(value: &'a Value) -> Self {
                skip_assert_initialized!();
                from_glib_full(ffi::gtk_value_dup_expression(value.to_glib_none().0))
            }
        }

        impl glib::value::ToValue for $rust_type {
            fn to_value(&self) -> glib::Value {
                let mut value = glib::Value::for_value_type::<$rust_type>();
                unsafe {
                    ffi::gtk_value_set_expression(value.to_glib_none_mut().0, self.to_glib_none().0 as *mut _)
                }
                value
            }

            fn value_type(&self) -> glib::Type {
                Self::static_type()
            }
        }

        impl glib::value::ToValueOptional for $rust_type {
            fn to_value_optional(s: Option<&Self>) -> glib::Value {
                skip_assert_initialized!();
                let mut value = glib::Value::for_value_type::<$rust_type>();
                unsafe {
                    ffi::gtk_value_set_expression(value.to_glib_none_mut().0, s.to_glib_none().0 as *mut _)
                }
                value
            }
        }
    };
}

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
                this_type.to_glib(),
                expression.map(|e| e.as_ref()).to_glib_none().0,
                property_name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_property_expression_new_for_pspec")]
    pub fn for_pspec<E: AsRef<Expression>>(expression: Option<&E>, pspec: glib::ParamSpec) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_property_expression_new_for_pspec(
                expression.map(|e| e.as_ref()).to_glib_none().0,
                pspec.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_property_expression_get_expression")]
    pub fn expression(&self) -> Option<Expression> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_property_expression_get_expression(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_property_expression_get_pspec")]
    pub fn pspec(&self) -> glib::ParamSpec {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_property_expression_get_pspec(
                self.to_glib_none().0,
            ))
        }
    }
}

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
    pub fn object(&self) -> Option<Object> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_object_expression_get_object(self.to_glib_none().0)) }
    }
}

define_expression!(
    ClosureExpression,
    ffi::GtkClosureExpression,
    ffi::gtk_closure_expression_get_type
);

impl ClosureExpression {
    #[doc(alias = "gtk_closure_expression_new")]
    pub fn new<F>(value_type: Type, callback: F, params: &[Expression]) -> Self
    where
        F: Fn(&[Value]) -> Option<Value> + 'static,
    {
        assert_initialized_main_thread!();
        let closure = glib::Closure::new_local(move |values| callback(values));
        unsafe {
            from_glib_full(ffi::gtk_closure_expression_new(
                value_type.to_glib(),
                closure.to_glib_none().0,
                params.len() as u32,
                params.to_glib_full(),
            ))
        }
    }

    #[doc(alias = "gtk_closure_expression_new")]
    pub fn with_closure(value_type: Type, closure: glib::Closure, params: &[Expression]) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_closure_expression_new(
                value_type.to_glib(),
                closure.to_glib_none().0,
                params.len() as u32,
                params.to_glib_full(),
            ))
        }
    }
}

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
    pub fn for_value(value: &Value) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_constant_expression_new_for_value(
                value.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_constant_expression_get_value")]
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
    use glib::StaticType;

    use super::*;

    #[test]
    fn test_expressions() {
        crate::init().unwrap();

        let _pspec = glib::ParamSpec::expression(
            "expression",
            "Expression",
            "Some Expression",
            glib::ParamFlags::CONSTRUCT_ONLY | glib::ParamFlags::READABLE,
        );

        let _prop_expr = PropertyExpression::new(
            crate::StringObject::static_type(),
            NONE_EXPRESSION,
            "string",
        );

        let obj = crate::IconTheme::new();
        let expr = ObjectExpression::new(&obj);
        assert_eq!(expr.object().unwrap(), obj);

        let expr1 = ConstantExpression::new(&23);
        assert_eq!(expr1.value().get::<i32>().unwrap(), 23);
        let expr2 = ConstantExpression::for_value(&"hello".to_value());
        assert_eq!(expr2.value().get::<String>().unwrap(), "hello");
        let expr1 = ConstantExpression::new(&23);
        assert_eq!(expr1.value().get::<i32>().unwrap(), 23);
        let expr2 = ConstantExpression::for_value(&"hello".to_value());
        assert_eq!(expr2.value().get::<String>().unwrap(), "hello");
    }
}
