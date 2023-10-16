use gtk::{glib, prelude::*};

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.builder_basics")
        .build();
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk::Application) {
    let ui_src = include_str!("gtk_builder.ui");
    let builder = gtk::Builder::from_string(ui_src);

    let window = builder
        .object::<gtk::ApplicationWindow>("window")
        .expect("Couldn't get window");
    window.set_application(Some(application));
    let bigbutton = builder
        .object::<gtk::Button>("button")
        .expect("Couldn't get button");

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
