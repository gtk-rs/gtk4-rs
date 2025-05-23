// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Widget};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkNotebookPage")]
    pub struct NotebookPage(Object<ffi::GtkNotebookPage>);

    match fn {
        type_ => || ffi::gtk_notebook_page_get_type(),
    }
}

impl NotebookPage {
    #[doc(alias = "gtk_notebook_page_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Widget {
        unsafe { from_glib_none(ffi::gtk_notebook_page_get_child(self.to_glib_none().0)) }
    }

    pub fn is_detachable(&self) -> bool {
        ObjectExt::property(self, "detachable")
    }

    pub fn set_detachable(&self, detachable: bool) {
        ObjectExt::set_property(self, "detachable", detachable)
    }

    pub fn menu(&self) -> Option<Widget> {
        ObjectExt::property(self, "menu")
    }

    #[doc(alias = "menu-label")]
    pub fn menu_label(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "menu-label")
    }

    #[doc(alias = "menu-label")]
    pub fn set_menu_label(&self, menu_label: Option<&str>) {
        ObjectExt::set_property(self, "menu-label", menu_label)
    }

    pub fn position(&self) -> i32 {
        ObjectExt::property(self, "position")
    }

    pub fn set_position(&self, position: i32) {
        ObjectExt::set_property(self, "position", position)
    }

    pub fn is_reorderable(&self) -> bool {
        ObjectExt::property(self, "reorderable")
    }

    pub fn set_reorderable(&self, reorderable: bool) {
        ObjectExt::set_property(self, "reorderable", reorderable)
    }

    pub fn tab(&self) -> Option<Widget> {
        ObjectExt::property(self, "tab")
    }

    #[doc(alias = "tab-expand")]
    pub fn is_tab_expand(&self) -> bool {
        ObjectExt::property(self, "tab-expand")
    }

    #[doc(alias = "tab-expand")]
    pub fn set_tab_expand(&self, tab_expand: bool) {
        ObjectExt::set_property(self, "tab-expand", tab_expand)
    }

    #[doc(alias = "tab-fill")]
    pub fn is_tab_fill(&self) -> bool {
        ObjectExt::property(self, "tab-fill")
    }

    #[doc(alias = "tab-fill")]
    pub fn set_tab_fill(&self, tab_fill: bool) {
        ObjectExt::set_property(self, "tab-fill", tab_fill)
    }

    #[doc(alias = "tab-label")]
    pub fn tab_label(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "tab-label")
    }

    #[doc(alias = "tab-label")]
    pub fn set_tab_label(&self, tab_label: Option<&str>) {
        ObjectExt::set_property(self, "tab-label", tab_label)
    }

    #[doc(alias = "detachable")]
    pub fn connect_detachable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_detachable_trampoline<F: Fn(&NotebookPage) + 'static>(
            this: *mut ffi::GtkNotebookPage,
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
                c"notify::detachable".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_detachable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "menu-label")]
    pub fn connect_menu_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_menu_label_trampoline<F: Fn(&NotebookPage) + 'static>(
            this: *mut ffi::GtkNotebookPage,
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
                c"notify::menu-label".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_menu_label_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "position")]
    pub fn connect_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_trampoline<F: Fn(&NotebookPage) + 'static>(
            this: *mut ffi::GtkNotebookPage,
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

    #[doc(alias = "reorderable")]
    pub fn connect_reorderable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_reorderable_trampoline<F: Fn(&NotebookPage) + 'static>(
            this: *mut ffi::GtkNotebookPage,
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
                c"notify::reorderable".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_reorderable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "tab-expand")]
    pub fn connect_tab_expand_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tab_expand_trampoline<F: Fn(&NotebookPage) + 'static>(
            this: *mut ffi::GtkNotebookPage,
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
                c"notify::tab-expand".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_tab_expand_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "tab-fill")]
    pub fn connect_tab_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tab_fill_trampoline<F: Fn(&NotebookPage) + 'static>(
            this: *mut ffi::GtkNotebookPage,
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
                c"notify::tab-fill".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_tab_fill_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "tab-label")]
    pub fn connect_tab_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tab_label_trampoline<F: Fn(&NotebookPage) + 'static>(
            this: *mut ffi::GtkNotebookPage,
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
                c"notify::tab-label".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_tab_label_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
