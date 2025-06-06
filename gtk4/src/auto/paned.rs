// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(feature = "v4_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
use crate::AccessibleRange;
use crate::{
    ffi, Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, LayoutManager, Orientable,
    Orientation, Overflow, ScrollType, Widget,
};
use glib::{
    object::ObjectType as _,
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

#[cfg(feature = "v4_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
glib::wrapper! {
    #[doc(alias = "GtkPaned")]
    pub struct Paned(Object<ffi::GtkPaned>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, AccessibleRange, Orientable;

    match fn {
        type_ => || ffi::gtk_paned_get_type(),
    }
}

#[cfg(not(any(feature = "v4_10")))]
glib::wrapper! {
    #[doc(alias = "GtkPaned")]
    pub struct Paned(Object<ffi::GtkPaned>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, Orientable;

    match fn {
        type_ => || ffi::gtk_paned_get_type(),
    }
}

impl Paned {
    #[doc(alias = "gtk_paned_new")]
    pub fn new(orientation: Orientation) -> Paned {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_paned_new(orientation.into_glib())).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Paned`] objects.
    ///
    /// This method returns an instance of [`PanedBuilder`](crate::builders::PanedBuilder) which can be used to create [`Paned`] objects.
    pub fn builder() -> PanedBuilder {
        PanedBuilder::new()
    }

    #[doc(alias = "gtk_paned_get_end_child")]
    #[doc(alias = "get_end_child")]
    #[doc(alias = "end-child")]
    pub fn end_child(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_paned_get_end_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_paned_get_position")]
    #[doc(alias = "get_position")]
    pub fn position(&self) -> i32 {
        unsafe { ffi::gtk_paned_get_position(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_paned_get_resize_end_child")]
    #[doc(alias = "get_resize_end_child")]
    #[doc(alias = "resize-end-child")]
    pub fn resizes_end_child(&self) -> bool {
        unsafe { from_glib(ffi::gtk_paned_get_resize_end_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_paned_get_resize_start_child")]
    #[doc(alias = "get_resize_start_child")]
    #[doc(alias = "resize-start-child")]
    pub fn resizes_start_child(&self) -> bool {
        unsafe { from_glib(ffi::gtk_paned_get_resize_start_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_paned_get_shrink_end_child")]
    #[doc(alias = "get_shrink_end_child")]
    #[doc(alias = "shrink-end-child")]
    pub fn shrinks_end_child(&self) -> bool {
        unsafe { from_glib(ffi::gtk_paned_get_shrink_end_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_paned_get_shrink_start_child")]
    #[doc(alias = "get_shrink_start_child")]
    #[doc(alias = "shrink-start-child")]
    pub fn shrinks_start_child(&self) -> bool {
        unsafe { from_glib(ffi::gtk_paned_get_shrink_start_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_paned_get_start_child")]
    #[doc(alias = "get_start_child")]
    #[doc(alias = "start-child")]
    pub fn start_child(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_paned_get_start_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_paned_get_wide_handle")]
    #[doc(alias = "get_wide_handle")]
    #[doc(alias = "wide-handle")]
    pub fn is_wide_handle(&self) -> bool {
        unsafe { from_glib(ffi::gtk_paned_get_wide_handle(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_paned_set_end_child")]
    #[doc(alias = "end-child")]
    pub fn set_end_child(&self, child: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_paned_set_end_child(
                self.to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_paned_set_position")]
    #[doc(alias = "position")]
    pub fn set_position(&self, position: i32) {
        unsafe {
            ffi::gtk_paned_set_position(self.to_glib_none().0, position);
        }
    }

    #[doc(alias = "gtk_paned_set_resize_end_child")]
    #[doc(alias = "resize-end-child")]
    pub fn set_resize_end_child(&self, resize: bool) {
        unsafe {
            ffi::gtk_paned_set_resize_end_child(self.to_glib_none().0, resize.into_glib());
        }
    }

    #[doc(alias = "gtk_paned_set_resize_start_child")]
    #[doc(alias = "resize-start-child")]
    pub fn set_resize_start_child(&self, resize: bool) {
        unsafe {
            ffi::gtk_paned_set_resize_start_child(self.to_glib_none().0, resize.into_glib());
        }
    }

    #[doc(alias = "gtk_paned_set_shrink_end_child")]
    #[doc(alias = "shrink-end-child")]
    pub fn set_shrink_end_child(&self, resize: bool) {
        unsafe {
            ffi::gtk_paned_set_shrink_end_child(self.to_glib_none().0, resize.into_glib());
        }
    }

    #[doc(alias = "gtk_paned_set_shrink_start_child")]
    #[doc(alias = "shrink-start-child")]
    pub fn set_shrink_start_child(&self, resize: bool) {
        unsafe {
            ffi::gtk_paned_set_shrink_start_child(self.to_glib_none().0, resize.into_glib());
        }
    }

    #[doc(alias = "gtk_paned_set_start_child")]
    #[doc(alias = "start-child")]
    pub fn set_start_child(&self, child: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_paned_set_start_child(
                self.to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_paned_set_wide_handle")]
    #[doc(alias = "wide-handle")]
    pub fn set_wide_handle(&self, wide: bool) {
        unsafe {
            ffi::gtk_paned_set_wide_handle(self.to_glib_none().0, wide.into_glib());
        }
    }

    #[doc(alias = "max-position")]
    pub fn max_position(&self) -> i32 {
        ObjectExt::property(self, "max-position")
    }

    #[doc(alias = "min-position")]
    pub fn min_position(&self) -> i32 {
        ObjectExt::property(self, "min-position")
    }

    #[doc(alias = "position-set")]
    pub fn is_position_set(&self) -> bool {
        ObjectExt::property(self, "position-set")
    }

    #[doc(alias = "accept-position")]
    pub fn connect_accept_position<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn accept_position_trampoline<F: Fn(&Paned) -> bool + 'static>(
            this: *mut ffi::GtkPaned,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"accept-position".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    accept_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_accept_position(&self) -> bool {
        self.emit_by_name("accept-position", &[])
    }

    #[doc(alias = "cancel-position")]
    pub fn connect_cancel_position<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cancel_position_trampoline<F: Fn(&Paned) -> bool + 'static>(
            this: *mut ffi::GtkPaned,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"cancel-position".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    cancel_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_cancel_position(&self) -> bool {
        self.emit_by_name("cancel-position", &[])
    }

    #[doc(alias = "cycle-child-focus")]
    pub fn connect_cycle_child_focus<F: Fn(&Self, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn cycle_child_focus_trampoline<F: Fn(&Paned, bool) -> bool + 'static>(
            this: *mut ffi::GtkPaned,
            reversed: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(reversed)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"cycle-child-focus".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    cycle_child_focus_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_cycle_child_focus(&self, reversed: bool) -> bool {
        self.emit_by_name("cycle-child-focus", &[&reversed])
    }

    #[doc(alias = "cycle-handle-focus")]
    pub fn connect_cycle_handle_focus<F: Fn(&Self, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn cycle_handle_focus_trampoline<
            F: Fn(&Paned, bool) -> bool + 'static,
        >(
            this: *mut ffi::GtkPaned,
            reversed: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(reversed)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"cycle-handle-focus".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    cycle_handle_focus_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_cycle_handle_focus(&self, reversed: bool) -> bool {
        self.emit_by_name("cycle-handle-focus", &[&reversed])
    }

    #[doc(alias = "move-handle")]
    pub fn connect_move_handle<F: Fn(&Self, ScrollType) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn move_handle_trampoline<F: Fn(&Paned, ScrollType) -> bool + 'static>(
            this: *mut ffi::GtkPaned,
            scroll_type: ffi::GtkScrollType,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(scroll_type)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"move-handle".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    move_handle_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_move_handle(&self, scroll_type: ScrollType) -> bool {
        self.emit_by_name("move-handle", &[&scroll_type])
    }

    #[doc(alias = "toggle-handle-focus")]
    pub fn connect_toggle_handle_focus<F: Fn(&Self) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn toggle_handle_focus_trampoline<F: Fn(&Paned) -> bool + 'static>(
            this: *mut ffi::GtkPaned,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"toggle-handle-focus".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    toggle_handle_focus_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_toggle_handle_focus(&self) -> bool {
        self.emit_by_name("toggle-handle-focus", &[])
    }

    #[doc(alias = "end-child")]
    pub fn connect_end_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_end_child_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut ffi::GtkPaned,
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
                c"notify::end-child".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_end_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "max-position")]
    pub fn connect_max_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_position_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut ffi::GtkPaned,
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
                c"notify::max-position".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_max_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "min-position")]
    pub fn connect_min_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_position_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut ffi::GtkPaned,
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
                c"notify::min-position".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_min_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "position")]
    pub fn connect_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut ffi::GtkPaned,
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
                c"notify::position".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "position-set")]
    pub fn connect_position_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_set_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut ffi::GtkPaned,
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
                c"notify::position-set".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_position_set_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "resize-end-child")]
    pub fn connect_resize_end_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_resize_end_child_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut ffi::GtkPaned,
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
                c"notify::resize-end-child".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_resize_end_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "resize-start-child")]
    pub fn connect_resize_start_child_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_resize_start_child_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut ffi::GtkPaned,
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
                c"notify::resize-start-child".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_resize_start_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "shrink-end-child")]
    pub fn connect_shrink_end_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_shrink_end_child_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut ffi::GtkPaned,
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
                c"notify::shrink-end-child".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_shrink_end_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "shrink-start-child")]
    pub fn connect_shrink_start_child_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_shrink_start_child_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut ffi::GtkPaned,
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
                c"notify::shrink-start-child".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_shrink_start_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "start-child")]
    pub fn connect_start_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_start_child_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut ffi::GtkPaned,
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
                c"notify::start-child".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_start_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "wide-handle")]
    pub fn connect_wide_handle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_wide_handle_trampoline<F: Fn(&Paned) + 'static>(
            this: *mut ffi::GtkPaned,
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
                c"notify::wide-handle".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_wide_handle_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Paned {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Paned`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PanedBuilder {
    builder: glib::object::ObjectBuilder<'static, Paned>,
}

impl PanedBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn end_child(self, end_child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("end-child", end_child.clone().upcast()),
        }
    }

    pub fn position(self, position: i32) -> Self {
        Self {
            builder: self.builder.property("position", position),
        }
    }

    pub fn position_set(self, position_set: bool) -> Self {
        Self {
            builder: self.builder.property("position-set", position_set),
        }
    }

    pub fn resize_end_child(self, resize_end_child: bool) -> Self {
        Self {
            builder: self.builder.property("resize-end-child", resize_end_child),
        }
    }

    pub fn resize_start_child(self, resize_start_child: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("resize-start-child", resize_start_child),
        }
    }

    pub fn shrink_end_child(self, shrink_end_child: bool) -> Self {
        Self {
            builder: self.builder.property("shrink-end-child", shrink_end_child),
        }
    }

    pub fn shrink_start_child(self, shrink_start_child: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("shrink-start-child", shrink_start_child),
        }
    }

    pub fn start_child(self, start_child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("start-child", start_child.clone().upcast()),
        }
    }

    pub fn wide_handle(self, wide_handle: bool) -> Self {
        Self {
            builder: self.builder.property("wide-handle", wide_handle),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    #[cfg(feature = "v4_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_18")))]
    pub fn limit_events(self, limit_events: bool) -> Self {
        Self {
            builder: self.builder.property("limit-events", limit_events),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    pub fn orientation(self, orientation: Orientation) -> Self {
        Self {
            builder: self.builder.property("orientation", orientation),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Paned`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Paned {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
