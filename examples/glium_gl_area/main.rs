use std::ptr;

use gtk::prelude::*;

mod glium_gl_area;
use glium_gl_area::GliumGLArea;

fn main() {
    // Load GL pointers from epoxy (GL context management library used by GTK).
    {
        #[cfg(target_os = "macos")]
        let library = unsafe { libloading::os::unix::Library::new("libepoxy.0.dylib") }.unwrap();
        #[cfg(all(unix, not(target_os = "macos")))]
        let library = unsafe { libloading::os::unix::Library::new("libepoxy.so.0") }.unwrap();
        #[cfg(windows)]
        let library = libloading::os::windows::Library::open_already_loaded("libepoxy-0.dll")
            .or_else(|_| libloading::os::windows::Library::open_already_loaded("epoxy-0.dll"))
            .unwrap();

        epoxy::load_with(|name| {
            unsafe { library.get::<_>(name.as_bytes()) }
                .map(|symbol| *symbol)
                .unwrap_or(ptr::null())
        });
    }

    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.glium-gl-area"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("Glium in GLArea"));

    let widget = GliumGLArea::new();
    window.set_child(Some(&widget));

    window.show();
}
