mod base_button;
mod derived_button;
mod virtual_methods_window;

use crate::base_button::BaseButton;
use crate::derived_button::DerivedButton;
use crate::virtual_methods_window::VirtualMethodsAppWindow;
use gtk::prelude::*;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.virtual_methods"),
        Default::default(),
    );

    application.connect_activate(|app| {
        BaseButton::static_type();
        DerivedButton::static_type();
        let win = VirtualMethodsAppWindow::new(app);
        win.show();
    });

    application.run();
}
