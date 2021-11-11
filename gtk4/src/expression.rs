// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ExpressionWatch;
use glib::translate::*;
use glib::{IsA, Object, StaticType, Type, Value};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GtkExpression")]
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
        self
    }
}

// rustdoc-stripper-ignore-next
/// A common trait implemented by the various [`gtk::Expression`] types.
///
/// # Safety
///
/// The user is not supposed to implement this trait.
pub unsafe trait IsExpression:
    glib::StaticType + FromGlibPtrFull<*mut ffi::GtkExpression> + 'static
{
}

impl Expression {
    pub const NONE: Option<&'static Expression> = None;

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
    #[doc(alias = "get_type")]
    pub fn type_(&self) -> Type {
        unsafe {
            let ptr = self.to_glib_none().0;

            from_glib((*(*(ptr as *mut glib::gobject_ffi::GTypeInstance)).g_class).g_type)
        }
    }

    #[doc(alias = "gtk_expression_get_value_type")]
    #[doc(alias = "get_value_type")]
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
    pub fn bind<T: IsA<Object>, U: IsA<Object>>(
        &self,
        target: &T,
        property_name: &str,
        this: Option<&U>,
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

    // rustdoc-stripper-ignore-next
    /// Create a [`gtk::PropertyExpression`] that looks up for `property_name`
    /// with self as parameter. This is useful in long chains of [`gtk::Expression`]s.
    pub fn chain_property<T: IsA<glib::Object>>(
        &self,
        property_name: &str,
    ) -> crate::PropertyExpression {
        crate::PropertyExpression::new(T::static_type(), Some(self), property_name)
    }

    // rustdoc-stripper-ignore-next
    /// Create a [`gtk::ClosureExpression`] with self as a parameter. This is useful in long
    /// chains of [`gtk::Expression`]s.
    pub fn chain_closure<F, R>(&self, f: F) -> crate::ClosureExpression
    where
        F: Fn(&[glib::Value]) -> R + 'static,
        R: glib::value::ValueType,
    {
        // FIXME Pass an owned self to avoid cloning
        crate::ClosureExpression::new(&[self.clone()], f)
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
        let mut value = glib::Value::for_value_type::<Self>();
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
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe { ffi::gtk_value_set_expression(value.to_glib_none_mut().0, s.to_glib_none().0) }
        value
    }
}

// rustdoc-stripper-ignore-next
/// Trait containing convenience methods in creating [`gtk::PropertyExpression`] that
/// looks up a property of a [`glib::Object`].
///
/// # Example
///
/// `label_expression` is a [`gtk::Expression`] that looks up at Button's `label`
/// property.
///
/// ```no_run
/// # use gtk4 as gtk;
/// use gtk::prelude::*;
///
/// let button = gtk::Button::new();
/// button.set_label("Label property");
///
/// let label_expression = button.property_expression("label");
/// ```
pub trait GObjectPropertyExpressionExt {
    // rustdoc-stripper-ignore-next
    /// Create an expression looking up an object's property.
    fn property_expression(&self, property_name: &str) -> crate::PropertyExpression;

    // rustdoc-stripper-ignore-next
    /// Create an expression looking up an object's property with a weak reference.
    fn property_expression_weak(&self, property_name: &str) -> crate::PropertyExpression;
}

impl<T: IsA<glib::Object>> GObjectPropertyExpressionExt for T {
    fn property_expression(&self, property_name: &str) -> crate::PropertyExpression {
        let obj_expr = crate::ConstantExpression::new(self);
        crate::PropertyExpression::new(T::static_type(), Some(&obj_expr), property_name)
    }

    fn property_expression_weak(&self, property_name: &str) -> crate::PropertyExpression {
        let obj_expr = crate::ObjectExpression::new(self);
        crate::PropertyExpression::new(T::static_type(), Some(&obj_expr), property_name)
    }
}

macro_rules! define_expression {
    ($rust_type:ident, $ffi_type:path, $get_type:path) => {
        impl std::ops::Deref for $rust_type {
            type Target = crate::Expression;

            fn deref(&self) -> &Self::Target {
                unsafe { &*(self as *const $rust_type as *const crate::Expression) }
            }
        }

        impl AsRef<crate::Expression> for $rust_type {
            fn as_ref(&self) -> &crate::Expression {
                self.upcast_ref()
            }
        }

        impl $rust_type {
            pub fn upcast(self) -> crate::Expression {
                unsafe { std::mem::transmute(self) }
            }

            pub fn upcast_ref(&self) -> &crate::Expression {
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
            fn static_type() -> glib::Type {
                unsafe { glib::translate::FromGlib::from_glib($get_type()) }
            }
        }

        unsafe impl crate::expression::IsExpression for $rust_type {}

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
                let mut value = glib::Value::for_value_type::<Self>();
                unsafe {
                    ffi::gtk_value_set_expression(
                        value.to_glib_none_mut().0,
                        self.to_glib_none().0 as *mut _,
                    )
                }
                value
            }

            fn value_type(&self) -> glib::Type {
                use glib::StaticType;
                Self::static_type()
            }
        }

        impl glib::value::ToValueOptional for $rust_type {
            fn to_value_optional(s: Option<&Self>) -> glib::Value {
                skip_assert_initialized!();
                let mut value = glib::Value::for_value_type::<Self>();
                unsafe {
                    ffi::gtk_value_set_expression(
                        value.to_glib_none_mut().0,
                        s.to_glib_none().0 as *mut _,
                    )
                }
                value
            }
        }
    };
}
