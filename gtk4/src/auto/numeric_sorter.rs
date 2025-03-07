// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Expression, SortType, Sorter};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkNumericSorter")]
    pub struct NumericSorter(Object<ffi::GtkNumericSorter, ffi::GtkNumericSorterClass>) @extends Sorter;

    match fn {
        type_ => || ffi::gtk_numeric_sorter_get_type(),
    }
}

impl NumericSorter {
    #[doc(alias = "gtk_numeric_sorter_new")]
    pub fn new(expression: Option<impl AsRef<Expression>>) -> NumericSorter {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_numeric_sorter_new(
                expression
                    .map(|p| p.as_ref().clone().upcast())
                    .into_glib_ptr(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`NumericSorter`] objects.
    ///
    /// This method returns an instance of [`NumericSorterBuilder`](crate::builders::NumericSorterBuilder) which can be used to create [`NumericSorter`] objects.
    pub fn builder() -> NumericSorterBuilder {
        NumericSorterBuilder::new()
    }

    #[doc(alias = "gtk_numeric_sorter_get_expression")]
    #[doc(alias = "get_expression")]
    pub fn expression(&self) -> Option<Expression> {
        unsafe {
            from_glib_none(ffi::gtk_numeric_sorter_get_expression(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_numeric_sorter_get_sort_order")]
    #[doc(alias = "get_sort_order")]
    #[doc(alias = "sort-order")]
    pub fn sort_order(&self) -> SortType {
        unsafe {
            from_glib(ffi::gtk_numeric_sorter_get_sort_order(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_numeric_sorter_set_expression")]
    #[doc(alias = "expression")]
    pub fn set_expression(&self, expression: Option<impl AsRef<Expression>>) {
        unsafe {
            ffi::gtk_numeric_sorter_set_expression(
                self.to_glib_none().0,
                expression.as_ref().map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_numeric_sorter_set_sort_order")]
    #[doc(alias = "sort-order")]
    pub fn set_sort_order(&self, sort_order: SortType) {
        unsafe {
            ffi::gtk_numeric_sorter_set_sort_order(self.to_glib_none().0, sort_order.into_glib());
        }
    }

    #[doc(alias = "expression")]
    pub fn connect_expression_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_expression_trampoline<F: Fn(&NumericSorter) + 'static>(
            this: *mut ffi::GtkNumericSorter,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::expression".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_expression_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "sort-order")]
    pub fn connect_sort_order_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sort_order_trampoline<F: Fn(&NumericSorter) + 'static>(
            this: *mut ffi::GtkNumericSorter,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::sort-order".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_sort_order_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for NumericSorter {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`NumericSorter`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct NumericSorterBuilder {
    builder: glib::object::ObjectBuilder<'static, NumericSorter>,
}

impl NumericSorterBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn expression(self, expression: impl AsRef<Expression>) -> Self {
        Self {
            builder: self
                .builder
                .property("expression", expression.as_ref().clone()),
        }
    }

    pub fn sort_order(self, sort_order: SortType) -> Self {
        Self {
            builder: self.builder.property("sort-order", sort_order),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`NumericSorter`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> NumericSorter {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
