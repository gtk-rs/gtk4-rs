// Take a look at the license at the top of the repository in the LICENSE file.

use std::cell::Cell;
use std::sync::atomic::{AtomicBool, Ordering};

thread_local! {
    static IS_MAIN_THREAD: Cell<bool> = Cell::new(false)
}

static INITIALIZED: AtomicBool = AtomicBool::new(false);

macro_rules! assert_initialized_main_thread {
    () => {
        if !crate::rt::is_initialized_main_thread() {
            if crate::rt::is_initialized() {
                panic!("GDK may only be used from the main thread.");
            } else {
                panic!("GDK has not been initialized. Call `gdk::init` or `gtk::init` first.");
            }
        }
    };
}

/// No-op.
macro_rules! skip_assert_initialized {
    () => {};
}

/// Returns `true` if GDK has been initialized.
#[inline]
pub fn is_initialized() -> bool {
    skip_assert_initialized!();
    INITIALIZED.load(Ordering::Acquire)
}

/// Returns `true` if GDK has been initialized and this is the main thread.
#[inline]
pub fn is_initialized_main_thread() -> bool {
    skip_assert_initialized!();
    IS_MAIN_THREAD.with(|c| c.get())
}
