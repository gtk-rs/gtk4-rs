use gtk::glib;
use gtk::prelude::*;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use gtk::{
    Application, ApplicationWindow, Builder, Button, FileChooserAction, FileChooserDialog,
    ResponseType, TextView,
};

fn main() {
    let application = Application::new(
        Some("com.github.gtk-rs.examples.text_viewer"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}

pub fn build_ui(application: &Application) {
    let ui_src = include_str!("text_viewer.ui");
    let builder = Builder::new();
    builder
        .add_from_string(ui_src)
        .expect("Couldn't add from string");

    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));
    let open_button: Button = builder.object("open_button").expect("Couldn't get builder");
    let text_view: TextView = builder.object("text_view").expect("Couldn't get text_view");

    open_button.connect_clicked(glib::clone!(@weak window, @weak text_view => move |_| {

        let file_chooser = FileChooserDialog::new(
            Some("Open File"),
            Some(&window),
            FileChooserAction::Open,
            &[("Open", ResponseType::Ok), ("Cancel", ResponseType::Cancel)],
        );

        file_chooser.connect_response(move |d: &FileChooserDialog, response: ResponseType| {
            if response == ResponseType::Ok {
                let file = d.file().expect("Couldn't get file");

                let filename = file.path().expect("Couldn't get file path");
                let file = File::open(filename).expect("Couldn't open file");

                let mut reader = BufReader::new(file);
                let mut contents = String::new();
                let _ = reader.read_to_string(&mut contents);

                text_view.buffer().set_text(&contents);
            }

            d.close();
        });

        file_chooser.show();
    }));

    window.show();
}
