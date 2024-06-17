use std::fs::read_to_string;

use gtk::{gio, glib, prelude::*};

fn main() -> glib::ExitCode {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.text_viewer"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run()
}

pub fn build_ui(application: &gtk::Application) {
    let ui_src = include_str!("text_viewer.ui");
    let builder = gtk::Builder::from_string(ui_src);

    let window = builder
        .object::<gtk::ApplicationWindow>("window")
        .expect("Couldn't get window");
    window.set_application(Some(application));
    let open_button = builder
        .object::<gtk::Button>("open_button")
        .expect("Couldn't get builder");
    let text_view = builder
        .object::<gtk::TextView>("text_view")
        .expect("Couldn't get text_view");

    open_button.connect_clicked(glib::clone!(
        #[weak]
        window,
        #[weak]
        text_view,
        move |_| {
            let dialog = gtk::FileDialog::builder()
                .title("Open File")
                .accept_label("Open")
                .build();

            dialog.open(Some(&window), gio::Cancellable::NONE, move |file| {
                if let Ok(file) = file {
                    let filename = file.path().expect("Couldn't get file path");
                    let contents = read_to_string(filename).expect("Couldn't open file");

                    text_view.buffer().set_text(&contents);
                }
            });
        }
    ));

    window.present();
}
