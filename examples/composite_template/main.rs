mod ex_application_window;
pub mod ex_menu_button;

use ex_application_window::ExApplicationWindow;
use gtk::glib;
use gtk::prelude::*;

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.composite_template")
        .build();

    application.connect_activate(|app| {
        let win = ExApplicationWindow::new(app);
        win.present();
    });
    application.run()
}
