use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder, Button, MessageDialog, ResponseType};

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.builder_basics"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &Application) {
    let ui_src = include_str!("gtk_builder.ui");
    let builder = Builder::from_string(ui_src);

    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));
    let bigbutton: Button = builder.object("button").expect("Couldn't get button");
    let dialog: MessageDialog = builder
        .object("messagedialog")
        .expect("Couldn't get messagedialog");

    dialog.connect_response(move |d: &MessageDialog, _: ResponseType| {
        d.hide();
    });

    bigbutton.connect_clicked(move |_| {
        dialog.show();
    });

    window.show();
}
