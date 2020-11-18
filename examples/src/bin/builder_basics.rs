//! # Builder Basics Sample
//!
//! This sample demonstrates how to use the builder with an imported glade file

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{ApplicationWindow, Builder, Button, MessageDialog, ResponseType};

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("builder_basics.ui");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("window1").expect("Couldn't get window1");
    window.set_application(Some(application));
    let bigbutton: Button = builder.get_object("button1").expect("Couldn't get button1");
    let dialog: MessageDialog = builder
        .get_object("messagedialog1")
        .expect("Couldn't get messagedialog1");

    dialog.connect_response(move |d: &MessageDialog, _: ResponseType| {
        d.close();
    });

    bigbutton.connect_clicked(move |_| {
        dialog.show();
    });

    window.show();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.builder_basics"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
