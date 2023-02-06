// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, Expression};
use glib::translate::*;
use glib::{value::FromValue, Object, StaticType, Type, Value};

#[doc(hidden)]
impl AsRef<Expression> for Expression {
    #[inline]
    fn as_ref(&self) -> &Expression {
        self
    }
}

// rustdoc-stripper-ignore-next
/// A common trait implemented by the various [`Expression`](crate::Expression) types.
///
/// # Safety
///
/// The user is not supposed to implement this trait.
pub unsafe trait IsExpression:
    glib::StaticType + FromGlibPtrFull<*mut ffi::GtkExpression> + 'static
{
}

impl Expression {
    #[inline]
    pub fn upcast(self) -> Self {
        self
    }

    #[inline]
    pub fn upcast_ref(&self) -> &Self {
        self
    }

    #[inline]
    pub fn is<E: IsExpression>(&self) -> bool {
        self.type_().is_a(E::static_type())
    }

    #[inline]
    pub fn downcast<E: IsExpression>(self) -> Result<E, Expression> {
        unsafe {
            if self.is::<E>() {
                Ok(from_glib_full(self.into_glib_ptr()))
            } else {
                Err(self)
            }
        }
    }

    #[inline]
    pub fn downcast_ref<E: IsExpression>(&self) -> Option<&E> {
        unsafe {
            if self.is::<E>() {
                Some(&*(self as *const Expression as *const E))
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn type_(&self) -> Type {
        unsafe {
            let ptr = self.as_ptr();
            from_glib((*(*(ptr as *mut glib::gobject_ffi::GTypeInstance)).g_class).g_type)
        }
    }

    #[doc(alias = "gtk_expression_evaluate")]
    pub fn evaluate(&self, this: Option<&impl IsA<Object>>) -> Option<Value> {
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

    // rustdoc-stripper-ignore-next
    /// Similar to [`Self::evaluate`] but panics if the value is of a different type.
    #[doc(alias = "gtk_expression_evaluate")]
    pub fn evaluate_as<V: for<'b> FromValue<'b> + 'static, T: IsA<Object>>(
        &self,
        this: Option<&T>,
    ) -> Option<V> {
        self.evaluate(this).map(|v| {
            v.get_owned::<V>()
                .expect("Failed to evaluate to this value type")
        })
    }

    // rustdoc-stripper-ignore-next
    /// Create a [`PropertyExpression`](crate::PropertyExpression) that looks up for
    /// `property_name` with self as parameter. This is useful in long chains of
    /// [`Expression`](crate::Expression)s.
    pub fn chain_property<T: IsA<glib::Object>>(
        &self,
        property_name: &str,
    ) -> crate::PropertyExpression {
        crate::PropertyExpression::new(T::static_type(), Some(self), property_name)
    }

    // rustdoc-stripper-ignore-next
    /// Create a [`ClosureExpression`](crate::ClosureExpression) from a [`glib::Closure`] with self
    /// as the second parameter and `R` as the return type. The return type is checked at run-time
    /// and must always be specified. This is useful in long chains of
    /// [`Expression`](crate::Expression)s when using the [`glib::closure!`] macro.
    ///
    /// Note that the first parameter will always be the `this` object bound to the expression. If
    /// `None` is passed as `this` then the type of the first parameter must be
    /// `Option<glib::Object>` otherwise type checking will panic.
    ///
    /// ```no_run
    /// # use gtk4 as gtk;
    /// use gtk::prelude::*;
    /// use gtk::glib;
    /// use glib::{closure, Object};
    ///
    /// let button = gtk::Button::new();
    /// button.set_label("Hello");
    /// let label = button
    ///     .property_expression("label")
    ///     .chain_closure::<String>(closure!(|_: Option<Object>, label: &str| {
    ///         format!("{} World", label)
    ///     }))
    ///     .evaluate_as::<String, _>(gtk::Widget::NONE);
    /// assert_eq!(label.unwrap(), "Hello World");
    /// ```
    pub fn chain_closure<R>(&self, closure: glib::RustClosure) -> crate::ClosureExpression
    where
        R: glib::value::ValueType,
    {
        crate::ClosureExpression::new::<R>([self], closure)
    }

    // rustdoc-stripper-ignore-next
    /// Create a [`ClosureExpression`](crate::ClosureExpression) with self as the second parameter.
    /// This is useful in long chains of [`Expression`](crate::Expression)s.
    pub fn chain_closure_with_callback<F, R>(&self, f: F) -> crate::ClosureExpression
    where
        F: Fn(&[glib::Value]) -> R + 'static,
        R: glib::value::ValueType,
    {
        crate::ClosureExpression::with_callback([self], f)
    }
}

impl std::fmt::Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Expression")
            .field("value_type", &self.value_type())
            .field("is_static", &self.is_static())
            .finish()
    }
}

impl glib::value::ValueType for Expression {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for Expression {
    type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib_full(ffi::gtk_value_dup_expression(value.to_glib_none().0))
    }
}

impl glib::value::ToValue for Expression {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe { ffi::gtk_value_set_expression(value.to_glib_none_mut().0, self.to_glib_none().0) }
        value
    }

    #[inline]
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

impl From<Expression> for glib::Value {
    fn from(e: Expression) -> Self {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<Expression>();
        unsafe { ffi::gtk_value_take_expression(value.to_glib_none_mut().0, e.into_glib_ptr()) }
        value
    }
}

// rustdoc-stripper-ignore-next
/// Trait containing convenience methods in creating
/// [`PropertyExpression`](crate::PropertyExpression) that
/// looks up a property of a [`glib::Object`].
///
/// # Example
///
/// `label_expression` is an [`Expression`](crate::Expression) that looks up at Button's `label`
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

    // rustdoc-stripper-ignore-next
    /// Create an expression looking up a property in the bound `this` object.
    fn this_expression(property_name: &str) -> crate::PropertyExpression;
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

    fn this_expression(property_name: &str) -> crate::PropertyExpression {
        skip_assert_initialized!();
        crate::PropertyExpression::new(T::static_type(), Expression::NONE, property_name)
    }
}

macro_rules! define_expression {
    ($rust_type:ident, $ffi_type:path) => {
        impl std::ops::Deref for $rust_type {
            type Target = crate::Expression;

            #[inline]
            fn deref(&self) -> &Self::Target {
                unsafe { &*(self as *const $rust_type as *const crate::Expression) }
            }
        }

        impl AsRef<crate::Expression> for $rust_type {
            #[inline]
            fn as_ref(&self) -> &crate::Expression {
                self.upcast_ref()
            }
        }

        impl $rust_type {
            #[inline]
            pub fn upcast(self) -> crate::Expression {
                unsafe { std::mem::transmute(self) }
            }

            #[inline]
            pub fn upcast_ref(&self) -> &crate::Expression {
                self
            }
        }

        #[doc(hidden)]
        impl<'a> ToGlibPtr<'a, *const ffi::GtkExpression> for $rust_type {
            type Storage =
                ::std::marker::PhantomData<&'a $crate::glib::shared::Shared<$ffi_type, $rust_type>>;

            #[inline]
            fn to_glib_none(
                &'a self,
            ) -> $crate::glib::translate::Stash<'a, *const ffi::GtkExpression, Self> {
                let stash =
                    $crate::glib::translate::ToGlibPtr::<*const $ffi_type>::to_glib_none(self);
                $crate::glib::translate::Stash(stash.0 as *const _, stash.1)
            }

            #[inline]
            fn to_glib_full(&self) -> *const ffi::GtkExpression {
                $crate::glib::translate::ToGlibPtr::<*const $ffi_type>::to_glib_full(self)
                    as *const _
            }
        }

        #[doc(hidden)]
        impl<'a> ToGlibPtr<'a, *mut ffi::GtkExpression> for $rust_type {
            type Storage =
                ::std::marker::PhantomData<&'a $crate::glib::shared::Shared<$ffi_type, $rust_type>>;

            #[inline]
            fn to_glib_none(
                &'a self,
            ) -> $crate::glib::translate::Stash<'a, *mut ffi::GtkExpression, Self> {
                let stash =
                    $crate::glib::translate::ToGlibPtr::<*mut $ffi_type>::to_glib_none(self);
                $crate::glib::translate::Stash(stash.0 as *mut _, stash.1)
            }

            #[inline]
            fn to_glib_full(&self) -> *mut ffi::GtkExpression {
                $crate::glib::translate::ToGlibPtr::<*mut $ffi_type>::to_glib_full(self) as *mut _
            }
        }

        #[doc(hidden)]
        impl IntoGlibPtr<*mut ffi::GtkExpression> for $rust_type {
            #[inline]
            unsafe fn into_glib_ptr(self) -> *mut ffi::GtkExpression {
                let s = std::mem::ManuallyDrop::new(self);
                s.to_glib_none().0
            }
        }

        #[doc(hidden)]
        impl IntoGlibPtr<*const ffi::GtkExpression> for $rust_type {
            #[inline]
            unsafe fn into_glib_ptr(self) -> *const ffi::GtkExpression {
                let s = std::mem::ManuallyDrop::new(self);
                s.to_glib_none().0
            }
        }

        #[doc(hidden)]
        impl FromGlibPtrFull<*mut ffi::GtkExpression> for $rust_type {
            #[inline]
            unsafe fn from_glib_full(ptr: *mut ffi::GtkExpression) -> Self {
                from_glib_full(ptr as *mut $ffi_type)
            }
        }

        unsafe impl crate::expression::IsExpression for $rust_type {}

        impl glib::value::ValueType for $rust_type {
            type Type = Self;
        }

        unsafe impl<'a> glib::value::FromValue<'a> for $rust_type {
            type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

            #[inline]
            unsafe fn from_value(value: &'a glib::Value) -> Self {
                skip_assert_initialized!();
                from_glib_full(ffi::gtk_value_dup_expression(value.to_glib_none().0))
            }
        }

        impl glib::value::ToValue for $rust_type {
            #[inline]
            fn to_value(&self) -> glib::Value {
                let mut value = glib::Value::for_value_type::<Self>();
                unsafe {
                    ffi::gtk_value_set_expression(
                        value.to_glib_none_mut().0,
                        self.as_ptr() as *mut _,
                    )
                }
                value
            }

            #[inline]
            fn value_type(&self) -> glib::Type {
                use glib::StaticType;
                Self::static_type()
            }
        }

        impl glib::value::ToValueOptional for $rust_type {
            #[inline]
            fn to_value_optional(s: Option<&Self>) -> glib::Value {
                skip_assert_initialized!();
                let mut value = glib::Value::for_value_type::<Self>();
                unsafe {
                    ffi::gtk_value_set_expression(
                        value.to_glib_none_mut().0,
                        s.map(|s| s.as_ptr()).unwrap_or(std::ptr::null_mut()) as *mut _,
                    )
                }
                value
            }
        }

        impl From<$rust_type> for glib::Value {
            fn from(e: $rust_type) -> Self {
                skip_assert_initialized!();
                let mut value = glib::Value::for_value_type::<$rust_type>();
                unsafe {
                    ffi::gtk_value_take_expression(value.to_glib_none_mut().0, e.into_glib_ptr())
                }
                value
            }
        }

        impl glib::HasParamSpec for $rust_type {
            type ParamSpec = crate::ParamSpecExpression;

            type SetValue = $rust_type;

            type BuilderFn = for<'a> fn(&'a str) -> crate::builders::ParamSpecExpressionBuilder<'a>;

            fn param_spec_builder() -> Self::BuilderFn {
                Self::ParamSpec::builder
            }
        }
    };
}
