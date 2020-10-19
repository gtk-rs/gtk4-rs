// Copyright 2013-2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use glib;
use glib::translate::*;
use libc::c_uint;
use std::cell::Cell;
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

thread_local! {
    static IS_MAIN_THREAD: Cell<bool> = Cell::new(false)
}

static INITIALIZED: AtomicBool = ATOMIC_BOOL_INIT;

/// Asserts that this is the main thread and `gtk::init` has been called.
macro_rules! assert_initialized_main_thread {
    () => {
        if !::rt::is_initialized_main_thread() {
            if ::rt::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
    };
}

/// No-op.
macro_rules! skip_assert_initialized {
    () => {};
}

/// Asserts that `gtk::init` has not been called.
#[allow(unused_macros)]
macro_rules! assert_not_initialized {
    () => {
        if ::rt::is_initialized() {
            panic!("This function has to be called before `gtk::init`.");
        }
    };
}

/// Returns `true` if GTK has been initialized.
#[inline]
pub fn is_initialized() -> bool {
    skip_assert_initialized!();
    INITIALIZED.load(Ordering::Acquire)
}

/// Returns `true` if GTK has been initialized and this is the main thread.
#[inline]
pub fn is_initialized_main_thread() -> bool {
    skip_assert_initialized!();
    IS_MAIN_THREAD.with(|c| c.get())
}

/// Informs this crate that GTK has been initialized and the current thread is the main one.
pub unsafe fn set_initialized() {
    skip_assert_initialized!();
    if is_initialized_main_thread() {
        return;
    } else if is_initialized() {
        panic!("Attempted to initialize GTK from two different threads.");
    } else if !{ from_glib(gtk_sys::gtk_is_initialized()) } {
        panic!("GTK was not actually initialized");
    }
    INITIALIZED.store(true, Ordering::Release);
    IS_MAIN_THREAD.with(|c| c.set(true));
}

/// Tries to initialize GTK+.
///
/// Call either this function or [`Application::new`][new] before using any
/// other GTK+ functions.
///
/// [new]: struct.Application.html#method.new
///
/// Note that this function calls `gtk_init_check()` rather than `gtk_init()`,
/// so will not cause the program to terminate if GTK could not be initialized.
/// Instead, an Ok is returned if the windowing system was successfully
/// initialized otherwise an Err is returned.
pub fn init() -> Result<(), glib::BoolError> {
    skip_assert_initialized!();
    if is_initialized_main_thread() {
        return Ok(());
    } else if is_initialized() {
        panic!("Attempted to initialize GTK from two different threads.");
    }
    unsafe {
        if from_glib(gtk_sys::gtk_init_check()) {
            if !glib::MainContext::default().acquire() {
                return Err(glib_bool_error!("Failed to acquire default main context"));
            }

            if !{ from_glib(gtk_sys::gtk_is_initialized()) } {
                return Err(glib_bool_error!("GTK was not actually initialized"));
            }

            set_initialized();
            Ok(())
        } else {
            Err(glib_bool_error!("Failed to initialize GTK"))
        }
    }
}

pub fn get_major_version() -> u32 {
    skip_assert_initialized!();
    unsafe { gtk_sys::gtk_get_major_version() as u32 }
}

pub fn get_minor_version() -> u32 {
    skip_assert_initialized!();
    unsafe { gtk_sys::gtk_get_minor_version() as u32 }
}

pub fn get_micro_version() -> u32 {
    skip_assert_initialized!();
    unsafe { gtk_sys::gtk_get_micro_version() as u32 }
}

pub fn get_binary_age() -> u32 {
    skip_assert_initialized!();
    unsafe { gtk_sys::gtk_get_binary_age() as u32 }
}

pub fn get_interface_age() -> u32 {
    skip_assert_initialized!();
    unsafe { gtk_sys::gtk_get_interface_age() as u32 }
}

pub fn check_version(
    required_major: u32,
    required_minor: u32,
    required_micro: u32,
) -> Option<String> {
    skip_assert_initialized!();
    unsafe {
        from_glib_none(gtk_sys::gtk_check_version(
            required_major as c_uint,
            required_minor as c_uint,
            required_micro as c_uint,
        ))
    }
}
