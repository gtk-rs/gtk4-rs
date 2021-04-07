use gtk::prelude::*;
use gtk::{Align, Application, ApplicationWindowBuilder, ButtonBuilder};

fn build_ui(application: &Application) {
    let window = ApplicationWindowBuilder::new()
        .application(application)
        .title("First GTK Program")
        .default_width(350)
        .default_height(70)
        .build();

    let button = ButtonBuilder::new()
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .halign(Align::Center)
        .valign(Align::Center)
        .label("Click Me!")
        .build();

    window.set_child(Some(&button));

    window.show();
}

fn main() {
    let application =
        Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(build_ui);
    application.run();
}
