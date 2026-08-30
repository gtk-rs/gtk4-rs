use std::ptr;

use gtk::{glib, prelude::*};

mod femtovg_area;
use femtovg_area::FemtoVGArea;

fn epoxy_library() -> &'static libloading::Library {
    use std::sync::OnceLock;

    static EPOXY: OnceLock<libloading::Library> = OnceLock::new();

    EPOXY.get_or_init(|| {
        #[cfg(target_os = "macos")]
        let filename = "libepoxy.0.dylib";
        #[cfg(all(unix, not(target_os = "macos")))]
        let filename = "libepoxy.so.0";
        #[cfg(windows)]
        let filename = "libepoxy-0.dll";

        // SAFETY: the filename is a compile-time constant.
        let library = unsafe { libloading::Library::new(filename) };

        #[cfg(windows)]
        let library = library.or_else(|_| unsafe { libloading::Library::new("epoxy-0.dll") });

        library.unwrap()
    })
}

/// `epoxy_<name>` symbols are entries in libepoxy's dispatch table, not
/// functions, so they must be dereferenced once to obtain the actual
/// dispatcher for the currently bound GL context.
pub(crate) fn get_proc_address(name: &str) -> *const std::ffi::c_void {
    use std::ffi::c_void;

    let library = epoxy_library();
    let symbol = format!("epoxy_{name}");

    unsafe {
        library
            .get::<*const c_void>(&symbol)
            .map(|sym| {
                let entry = sym.try_as_raw_ptr().unwrap() as *const *const c_void;
                *entry
            })
            .unwrap_or(ptr::null())
    }
}

fn main() -> glib::ExitCode {
    epoxy_library();

    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.femtovg-area")
        .build();
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("FemtoVG in GLArea"));

    let widget = FemtoVGArea::default();
    window.set_child(Some(&widget));

    window.present();
}
