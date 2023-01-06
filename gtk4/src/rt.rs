// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use std::{
    cell::Cell,
    sync::atomic::{AtomicBool, Ordering},
};

#[cfg(target_os = "macos")]
extern "C" {
    fn pthread_main_np() -> i32;
}

thread_local! {
    static IS_MAIN_THREAD: Cell<bool> = Cell::new(false)
}

static INITIALIZED: AtomicBool = AtomicBool::new(false);

// rustdoc-stripper-ignore-next
/// Asserts that this is the main thread and `gtk::init` has been called.
macro_rules! assert_initialized_main_thread {
    () => {
        #[allow(unknown_lints)]
        #[allow(clippy::if_then_panic)]
        if !crate::rt::is_initialized_main_thread() {
            if crate::rt::is_initialized() {
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
        assert!(
            !crate::rt::is_initialized(),
            "This function has to be called before `gtk::init`."
        );
    };
}

/// Returns `true` if GTK has been initialized.
#[inline]
pub fn is_initialized() -> bool {
    skip_assert_initialized!();
    if cfg!(not(feature = "unsafe-assume-initialized")) {
        INITIALIZED.load(Ordering::Acquire)
    } else {
        true
    }
}

/// Returns `true` if GTK has been initialized and this is the main thread.
#[inline]
pub fn is_initialized_main_thread() -> bool {
    skip_assert_initialized!();
    if cfg!(not(feature = "unsafe-assume-initialized")) {
        IS_MAIN_THREAD.with(|c| c.get())
    } else {
        true
    }
}

/// Informs this crate that GTK has been initialized and the current thread is the main one.
///
/// # Panics
///
/// This function will panic if you attempt to initialize GTK from more than
/// one thread.
///
/// # Safety
///
/// You must only call this if:
///
/// 1. You have initialized the underlying GTK library yourself.
/// 2. You did 1 on the thread with which you are calling this function
/// 3. You ensure that this thread is the main thread for the process.
#[allow(unknown_lints)]
#[allow(clippy::if_then_panic)]
pub unsafe fn set_initialized() {
    skip_assert_initialized!();
    if is_initialized_main_thread() {
        return;
    } else if is_initialized() {
        panic!("Attempted to initialize GTK from two different threads.");
    } else if !{ from_glib(ffi::gtk_is_initialized()) } {
        panic!("GTK was not actually initialized");
    }
    //  OS X has its own notion of the main thread and init must be called on that thread.
    #[cfg(target_os = "macos")]
    {
        assert_ne!(
            pthread_main_np(),
            0,
            "Attempted to initialize GTK on OSX from non-main thread"
        );
    }
    INITIALIZED.store(true, Ordering::Release);
    IS_MAIN_THREAD.with(|c| c.set(true));
}

/// Tries to initialize GTK.
///
/// Call either this function or [`Application::new`][new] before using any
/// other GTK functions.
///
/// [new]: struct.Application.html#method.new
///
/// Note that this function calls `gtk_init_check()` rather than `gtk_init()`,
/// so will not cause the program to terminate if GTK could not be initialized.
/// Instead, an Ok is returned if the windowing system was successfully
/// initialized otherwise an Err is returned.
#[doc(alias = "gtk_init")]
#[allow(unknown_lints)]
#[allow(clippy::if_then_panic)]
pub fn init() -> Result<(), glib::BoolError> {
    skip_assert_initialized!();
    if is_initialized_main_thread() {
        return Ok(());
    } else if is_initialized() {
        #[cfg(not(test))]
        panic!("Attempted to initialize GTK from two different threads.");
        #[cfg(test)]
        panic!("Use #[gtk::test] instead of #[test]");
    }

    unsafe {
        if from_glib(ffi::gtk_init_check()) {
            // See https://github.com/gtk-rs/gtk-rs-core/issues/186 for reasoning behind
            // acquiring and leaking the main context here.
            let result: bool = from_glib(glib::ffi::g_main_context_acquire(
                glib::ffi::g_main_context_default(),
            ));
            if !result {
                return Err(glib::bool_error!("Failed to acquire default main context"));
            }

            if !from_glib::<glib::ffi::gboolean, bool>(ffi::gtk_is_initialized()) {
                return Err(glib::bool_error!("GTK was not actually initialized"));
            }

            set_initialized();
            Ok(())
        } else {
            Err(glib::bool_error!("Failed to initialize GTK"))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate as gtk4;

    #[test]
    fn init_should_acquire_default_main_context() {
        let context = glib::MainContext::ref_thread_default();
        assert!(context.is_owner());
    }
}
