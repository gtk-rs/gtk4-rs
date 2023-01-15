// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, DeleteType, MovementStep, Text, Widget};
use glib::{
    signal::{connect_raw, SignalHandlerId},
    translate::*,
    GString, IntoGStr,
};
use std::mem::transmute;

impl Text {
    pub fn connect_activate<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<F: Fn(&Text) + 'static>(
            this: *mut ffi::GtkText,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate\0".as_ptr() as *const _,
                Some(transmute(activate_trampoline::<F> as usize)),
                Box::into_raw(f),
            )
        }
    }

    pub fn connect_backspace<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn backspace_trampoline<F: Fn(&Text) + 'static>(
            this: *mut ffi::GtkText,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"backspace\0".as_ptr() as *const _,
                Some(transmute(backspace_trampoline::<F> as usize)),
                Box::into_raw(f),
            )
        }
    }

    pub fn connect_copy_clipboard<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn copy_clipboard_trampoline<F: Fn(&Text) + 'static>(
            this: *mut ffi::GtkText,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"copy-clipboard\0".as_ptr() as *const _,
                Some(transmute(copy_clipboard_trampoline::<F> as usize)),
                Box::into_raw(f),
            )
        }
    }

    pub fn connect_cut_clipboard<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cut_clipboard_trampoline<F: Fn(&Text) + 'static>(
            this: *mut ffi::GtkText,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cut-clipboard\0".as_ptr() as *const _,
                Some(transmute(cut_clipboard_trampoline::<F> as usize)),
                Box::into_raw(f),
            )
        }
    }

    pub fn connect_delete_from_cursor<F: Fn(&Text, DeleteType, i32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn delete_from_cursor_trampoline<
            F: Fn(&Text, DeleteType, i32) + 'static,
        >(
            this: *mut ffi::GtkText,
            type_: ffi::GtkDeleteType,
            count: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(type_), count)
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"delete-from-cursor\0".as_ptr() as *const _,
                Some(transmute(delete_from_cursor_trampoline::<F> as usize)),
                Box::into_raw(f),
            )
        }
    }

    pub fn connect_insert_at_cursor<F: Fn(&Text, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn insert_at_cursor_trampoline<F: Fn(&Text, &str) + 'static>(
            this: *mut ffi::GtkText,
            string: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &GString::from_glib_borrow(string))
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"insert-at-cursor\0".as_ptr() as *const _,
                Some(transmute(insert_at_cursor_trampoline::<F> as usize)),
                Box::into_raw(f),
            )
        }
    }

    pub fn connect_insert_emoji<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn insert_emoji_trampoline<F: Fn(&Text) + 'static>(
            this: *mut ffi::GtkText,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"insert-emoji\0".as_ptr() as *const _,
                Some(transmute(insert_emoji_trampoline::<F> as usize)),
                Box::into_raw(f),
            )
        }
    }

    pub fn connect_move_cursor<F: Fn(&Text, MovementStep, i32, bool) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn move_cursor_trampoline<
            F: Fn(&Text, MovementStep, i32, bool) + 'static,
        >(
            this: *mut ffi::GtkText,
            step: ffi::GtkMovementStep,
            count: libc::c_int,
            extend: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                from_glib(step),
                count,
                from_glib(extend),
            )
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-cursor\0".as_ptr() as *const _,
                Some(transmute(move_cursor_trampoline::<F> as usize)),
                Box::into_raw(f),
            )
        }
    }

    pub fn connect_paste_clipboard<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn paste_clipboard_trampoline<F: Fn(&Text) + 'static>(
            this: *mut ffi::GtkText,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"paste-clipboard\0".as_ptr() as *const _,
                Some(transmute(paste_clipboard_trampoline::<F> as usize)),
                Box::into_raw(f),
            )
        }
    }

    pub fn connect_populate_popup<F: Fn(&Text, &Widget) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn populate_popup_trampoline<F: Fn(&Text, &Widget) + 'static>(
            this: *mut ffi::GtkText,
            widget: *mut ffi::GtkWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(widget))
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"populate-popup\0".as_ptr() as *const _,
                Some(transmute(populate_popup_trampoline::<F> as usize)),
                Box::into_raw(f),
            )
        }
    }

    pub fn connect_preedit_changed<F: Fn(&Text, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn preedit_changed_trampoline<F: Fn(&Text, &str) + 'static>(
            this: *mut ffi::GtkText,
            preedit: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &GString::from_glib_borrow(preedit))
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"preedit-changed\0".as_ptr() as *const _,
                Some(transmute(preedit_changed_trampoline::<F> as usize)),
                Box::into_raw(f),
            )
        }
    }

    pub fn connect_toggle_overwrite<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn toggle_overwrite_trampoline<F: Fn(&Text) + 'static>(
            this: *mut ffi::GtkText,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"toggle-overwrite\0".as_ptr() as *const _,
                Some(transmute(toggle_overwrite_trampoline::<F> as usize)),
                Box::into_raw(f),
            )
        }
    }

    pub fn emit_activate(&self) {
        self.emit_by_name::<()>("activate", &[]);
    }

    pub fn emit_backspace(&self) {
        self.emit_by_name::<()>("backspace", &[]);
    }

    pub fn emit_copy_clipboard(&self) {
        self.emit_by_name::<()>("copy-clipboard", &[]);
    }

    pub fn emit_cut_clipboard(&self) {
        self.emit_by_name::<()>("cut-clipboard", &[]);
    }

    pub fn emit_delete_from_cursor(&self, type_: DeleteType, count: i32) {
        self.emit_by_name::<()>("delete-from-cursor", &[&type_, &count]);
    }

    pub fn emit_insert_at_cursor(&self, string: impl IntoGStr) {
        string.run_with_gstr(|string| {
            self.emit_by_name::<()>("insert-at-cursor", &[&string]);
        });
    }

    pub fn emit_insert_emoji(&self) {
        self.emit_by_name::<()>("insert-emoji", &[]);
    }

    pub fn emit_move_cursor(&self, step: MovementStep, count: i32, extend: bool) {
        self.emit_by_name::<()>("move-cursor", &[&step, &count, &extend]);
    }

    pub fn emit_paste_clipboard(&self) {
        self.emit_by_name::<()>("paste-clipboard", &[]);
    }

    pub fn emit_preedit_changed(&self, preedit: impl IntoGStr) {
        preedit.run_with_gstr(|preedit| {
            self.emit_by_name::<()>("preedit-changed", &[&preedit]);
        });
    }

    pub fn emit_toggle_overwrite(&self) {
        self.emit_by_name::<()>("toggle-overwrite", &[]);
    }
}
