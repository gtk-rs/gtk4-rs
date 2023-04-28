use gtk::prelude::*;
use gtk::{glib, Align, Application, ApplicationWindow, Button};

fn main() -> glib::ExitCode {
    let application = Application::new(
        Some("com.github.gtk-rs.examples.builder_pattern"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &Application) {
    let window = ApplicationWindow::builder()
        .application(application)
        .title("First GTK Program")
        .default_width(350)
        .default_height(70)
        .build();

    let button = Button::builder()
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .halign(Align::Center)
        .valign(Align::Center)
        .label("Click Me!")
        .build();

    window.set_child(Some(&button));

    window.present();
}
