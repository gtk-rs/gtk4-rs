mod ex_button;

use ex_button::ExButton;
use gtk::{glib, prelude::*};

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.widget_subclass")
        .build();
    application.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);
        let button = ExButton::new();
        button.set_margin_top(18);
        button.set_margin_bottom(18);
        button.set_margin_start(18);
        button.set_margin_end(18);
        window.set_child(Some(&button));
        window.present();
    });

    application.run()
}
