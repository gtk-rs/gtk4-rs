mod text_view;

use gtk::{glib, prelude::*};
use text_view::AccessibleTextView;

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.accessible_text")
        .build();
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title(Some("Accessible Text Example"));
    window.set_default_size(260, 140);

    let text_view = glib::Object::new::<AccessibleTextView>();
    window.set_child(Some(&text_view));

    window.present();
}
