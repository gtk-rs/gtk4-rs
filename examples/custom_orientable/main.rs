mod custom_orientable;

use custom_orientable::CustomOrientable;
use gtk::glib;
use gtk::prelude::*;

fn main() -> glib::ExitCode {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.orientable_subclass"),
        Default::default(),
    );
    application.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);
        let bx = gtk::Box::new(gtk::Orientation::Vertical, 6);
        let orientable = CustomOrientable::new();
        let button = gtk::Button::with_label("Switch orientation");

        button.connect_clicked(glib::clone!(@weak orientable => move |_| {
            match orientable.orientation() {
                gtk::Orientation::Horizontal => orientable.set_orientation(gtk::Orientation::Vertical),
                gtk::Orientation::Vertical => orientable.set_orientation(gtk::Orientation::Horizontal),
                _ => unreachable!(),
            };
        }));

        orientable.set_halign(gtk::Align::Center);
        bx.append(&orientable);
        bx.append(&button);
        bx.set_margin_top(18);
        bx.set_margin_bottom(18);
        bx.set_margin_start(18);
        bx.set_margin_end(18);

        window.set_child(Some(&bx));
        window.present();
    });

    application.run()
}
