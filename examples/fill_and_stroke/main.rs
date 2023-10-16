mod paintable;

use gtk::{glib, prelude::*};

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.fill_and_stroke")
        .build();

    application.connect_activate(|app| {
        let win = gtk::ApplicationWindow::builder()
            .application(app)
            .title("Fill and Stroke")
            .build();

        let picture = gtk::Picture::builder()
            .content_fit(gtk::ContentFit::ScaleDown)
            .build();
        let paintable = paintable::CustomPaintable::new();
        picture.set_paintable(Some(&paintable));
        win.set_child(Some(&picture));

        win.present();
    });

    application.run()
}
