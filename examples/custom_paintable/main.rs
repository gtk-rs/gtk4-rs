mod custom_paintable;

use custom_paintable::CustomPaintable;
use gtk::glib;
use gtk::prelude::*;

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.paintable")
        .build();
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("Custom Paintable"));
    window.set_default_size(500, 500);

    let paintable = CustomPaintable::new();

    let picture = gtk::Picture::new();
    picture.set_halign(gtk::Align::Center);
    picture.set_size_request(200, 200);
    picture.set_paintable(Some(&paintable));

    window.set_child(Some(&picture));
    window.present();
}
