mod my_app_window;

use gtk::glib;
use gtk::prelude::*;
use my_app_window::MyAppWindow;

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.composite_dialog")
        .build();

    application.connect_activate(|app| {
        let win = MyAppWindow::new(app);
        win.present();
    });
    application.run()
}
