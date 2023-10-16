use chrono::Local;
use gtk::{glib, prelude::*};

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.clock")
        .build();
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title(Some("Clock Example"));
    window.set_default_size(260, 40);

    let time = current_time();
    let label = gtk::Label::default();
    label.set_text(&time);

    window.set_child(Some(&label));

    window.present();

    // we are using a closure to capture the label (else we could also use a normal
    // function)
    let tick = move || {
        let time = current_time();
        label.set_text(&time);
        // we could return glib::ControlFlow::Break to stop our clock after this tick
        glib::ControlFlow::Continue
    };

    // executes the closure once every second
    glib::timeout_add_seconds_local(1, tick);
}

fn current_time() -> String {
    format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
}
