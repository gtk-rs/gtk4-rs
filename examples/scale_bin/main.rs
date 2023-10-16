mod scale_bin;

use gtk::{glib, prelude::*};
use scale_bin::ScaleBin;

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.scale_bin")
        .build();

    application.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);
        let scale_bin = ScaleBin::new();
        window.set_child(Some(&scale_bin));
        window.present();
    });

    application.run()
}
