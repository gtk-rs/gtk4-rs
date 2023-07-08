use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Builder, Button};

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.builder_basics")
        .build();
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &Application) {
    let ui_src = include_str!("gtk_builder.ui");
    let builder = Builder::from_string(ui_src);

    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));
    let bigbutton: Button = builder.object("button").expect("Couldn't get button");

    bigbutton.connect_clicked(glib::clone!(@weak window => move |_| {
        gtk::AlertDialog::builder()
            .modal(true)
            .message("Thank you for trying this example")
            .detail("You have pressed the button")
            .buttons(["Ok"])
            .build()
            .show(Some(&window));
    }));

    window.present();
}
