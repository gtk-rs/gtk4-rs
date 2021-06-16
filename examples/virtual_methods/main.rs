mod base_button;
mod derived_button;
mod virtual_methods_window;

use crate::virtual_methods_window::VirtualMethodsAppWindow;
use gtk::prelude::*;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.virtual_methods"),
        Default::default(),
    );

    application.connect_activate(|app| {
        let win = VirtualMethodsAppWindow::new(app);
        win.show();
    });

    application.run();
}
