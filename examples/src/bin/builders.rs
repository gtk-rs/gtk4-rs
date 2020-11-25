//! # Builders Sample
//!
//! This sample demonstrates how to create a widget using the builders.
//! These allow to set construct-only properties and other construct
//! properties when creating the widget.

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Align, Application, ApplicationWindowBuilder, ButtonBuilder};

use std::env::args;

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

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
