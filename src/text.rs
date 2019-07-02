// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use DeleteType;
use MovementStep;
use Text;
use Widget;
use glib;
use glib::GString;
use glib::object::ObjectExt;
use glib::object::ObjectType;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use gobject_sys;
use gtk_sys;
use std::mem::transmute;

impl Text {
    pub fn connect_activate<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(self.as_ptr() as *mut _, b"activate\0".as_ptr() as *const _,
                Some(transmute(activate_trampoline::<F> as usize)), Box::into_raw(f))
        }
    }

    pub fn connect_backspace<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn backspace_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(self.as_ptr() as *mut _, b"backspace\0".as_ptr() as *const _,
                Some(transmute(backspace_trampoline::<F> as usize)), Box::into_raw(f))
        }
    }

    pub fn connect_copy_clipboard<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn copy_clipboard_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(self.as_ptr() as *mut _, b"copy-clipboard\0".as_ptr() as *const _,
                Some(transmute(copy_clipboard_trampoline::<F> as usize)), Box::into_raw(f))
        }
    }

    pub fn connect_cut_clipboard<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cut_clipboard_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(self.as_ptr() as *mut _, b"cut-clipboard\0".as_ptr() as *const _,
                Some(transmute(cut_clipboard_trampoline::<F> as usize)), Box::into_raw(f))
        }
    }

    pub fn connect_delete_from_cursor<F: Fn(&Text, DeleteType, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn delete_from_cursor_trampoline<F: Fn(&Text, DeleteType, i32) + 'static>(this: *mut gtk_sys::GtkText, type_: gtk_sys::GtkDeleteType, count: libc::c_int, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(type_), count)
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(self.as_ptr() as *mut _, b"delete-from-cursor\0".as_ptr() as *const _,
                Some(transmute(delete_from_cursor_trampoline::<F> as usize)), Box::into_raw(f))
        }
    }

    pub fn connect_insert_at_cursor<F: Fn(&Text, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn insert_at_cursor_trampoline<F: Fn(&Text, &str) + 'static>(this: *mut gtk_sys::GtkText, string: *mut libc::c_char, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &GString::from_glib_borrow(string))
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(self.as_ptr() as *mut _, b"insert-at-cursor\0".as_ptr() as *const _,
                Some(transmute(insert_at_cursor_trampoline::<F> as usize)), Box::into_raw(f))
        }
    }

    pub fn connect_insert_emoji<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn insert_emoji_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(self.as_ptr() as *mut _, b"insert-emoji\0".as_ptr() as *const _,
                Some(transmute(insert_emoji_trampoline::<F> as usize)), Box::into_raw(f))
        }
    }

    pub fn connect_move_cursor<F: Fn(&Text, MovementStep, i32, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn move_cursor_trampoline<F: Fn(&Text, MovementStep, i32, bool) + 'static>(this: *mut gtk_sys::GtkText, step: gtk_sys::GtkMovementStep, count: libc::c_int, extend: glib_sys::gboolean, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(step), count, from_glib(extend))
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(self.as_ptr() as *mut _, b"move-cursor\0".as_ptr() as *const _,
                Some(transmute(move_cursor_trampoline::<F> as usize)), Box::into_raw(f))
        }
    }

    pub fn connect_paste_clipboard<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn paste_clipboard_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(self.as_ptr() as *mut _, b"paste-clipboard\0".as_ptr() as *const _,
                Some(transmute(paste_clipboard_trampoline::<F> as usize)), Box::into_raw(f))
        }
    }

    pub fn connect_populate_popup<F: Fn(&Text, &Widget) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn populate_popup_trampoline<F: Fn(&Text, &Widget) + 'static>(this: *mut gtk_sys::GtkText, widget: *mut gtk_sys::GtkWidget, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(widget))
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(self.as_ptr() as *mut _, b"populate-popup\0".as_ptr() as *const _,
                Some(transmute(populate_popup_trampoline::<F> as usize)), Box::into_raw(f))
        }
    }

    pub fn connect_preedit_changed<F: Fn(&Text, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn preedit_changed_trampoline<F: Fn(&Text, &str) + 'static>(this: *mut gtk_sys::GtkText, preedit: *mut libc::c_char, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &GString::from_glib_borrow(preedit))
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(self.as_ptr() as *mut _, b"preedit-changed\0".as_ptr() as *const _,
                Some(transmute(preedit_changed_trampoline::<F> as usize)), Box::into_raw(f))
        }
    }

    pub fn connect_toggle_overwrite<F: Fn(&Text) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn toggle_overwrite_trampoline<F: Fn(&Text) + 'static>(this: *mut gtk_sys::GtkText, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(self.as_ptr() as *mut _, b"toggle-overwrite\0".as_ptr() as *const _,
                Some(transmute(toggle_overwrite_trampoline::<F> as usize)), Box::into_raw(f))
        }
    }

    pub fn emit_activate(&self) {
        let stash: Stash<*mut gtk_sys::GtkText, _> = self.to_glib_none();
        let _ = unsafe { glib::Object::from_glib_borrow(stash.0 as *mut gobject_sys::GObject).emit("activate", &[]).unwrap() };
    }

    pub fn emit_backspace(&self) {
        let stash: Stash<*mut gtk_sys::GtkText, _> = self.to_glib_none();
        let _ = unsafe { glib::Object::from_glib_borrow(stash.0 as *mut gobject_sys::GObject).emit("backspace", &[]).unwrap() };
    }

    pub fn emit_copy_clipboard(&self) {
        let stash: Stash<*mut gtk_sys::GtkText, _> = self.to_glib_none();
        let _ = unsafe { glib::Object::from_glib_borrow(stash.0 as *mut gobject_sys::GObject).emit("copy-clipboard", &[]).unwrap() };
    }

    pub fn emit_cut_clipboard(&self) {
        let stash: Stash<*mut gtk_sys::GtkText, _> = self.to_glib_none();
        let _ = unsafe { glib::Object::from_glib_borrow(stash.0 as *mut gobject_sys::GObject).emit("cut-clipboard", &[]).unwrap() };
    }

    pub fn emit_delete_from_cursor(&self, type_: DeleteType, count: i32) {
        let stash: Stash<*mut gtk_sys::GtkText, _> = self.to_glib_none();
        let _ = unsafe { glib::Object::from_glib_borrow(stash.0 as *mut gobject_sys::GObject).emit("delete-from-cursor", &[&type_, &count]).unwrap() };
    }

    pub fn emit_insert_at_cursor(&self, string: &str) {
        let stash: Stash<*mut gtk_sys::GtkText, _> = self.to_glib_none();
        let _ = unsafe { glib::Object::from_glib_borrow(stash.0 as *mut gobject_sys::GObject).emit("insert-at-cursor", &[&string]).unwrap() };
    }

    pub fn emit_insert_emoji(&self) {
        let stash: Stash<*mut gtk_sys::GtkText, _> = self.to_glib_none();
        let _ = unsafe { glib::Object::from_glib_borrow(stash.0 as *mut gobject_sys::GObject).emit("insert-emoji", &[]).unwrap() };
    }

    pub fn emit_move_cursor(&self, step: MovementStep, count: i32, extend: bool) {
        let stash: Stash<*mut gtk_sys::GtkText, _> = self.to_glib_none();
        let _ = unsafe { glib::Object::from_glib_borrow(stash.0 as *mut gobject_sys::GObject).emit("move-cursor", &[&step, &count, &extend]).unwrap() };
    }

    pub fn emit_paste_clipboard(&self) {
        let stash: Stash<*mut gtk_sys::GtkText, _> = self.to_glib_none();
        let _ = unsafe { glib::Object::from_glib_borrow(stash.0 as *mut gobject_sys::GObject).emit("paste-clipboard", &[]).unwrap() };
    }

    pub fn emit_preedit_changed(&self, preedit: &str) {
        let stash: Stash<*mut gtk_sys::GtkText, _> = self.to_glib_none();
        let _ = unsafe { glib::Object::from_glib_borrow(stash.0 as *mut gobject_sys::GObject).emit("preedit-changed", &[&preedit]).unwrap() };
    }

    pub fn emit_toggle_overwrite(&self) {
        let stash: Stash<*mut gtk_sys::GtkText, _> = self.to_glib_none();
        let _ = unsafe { glib::Object::from_glib_borrow(stash.0 as *mut gobject_sys::GObject).emit("toggle-overwrite", &[]).unwrap() };
    }
}
