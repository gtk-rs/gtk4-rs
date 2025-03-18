use glib::clone;
use gtk::{Application, ApplicationWindow, Button, glib};
use gtk::{gio, prelude::*};

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

const APP_ID: &str = "org.gtk_rs.MainEventLoop10";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

struct DemoStruct {
    text: String,
}

impl DemoStruct {
    fn mutate(&mut self, string: String) {
        self.text = string.to_string();
    }
}

fn build_ui(app: &Application) {
    // Create a button
    let button = Button::builder()
        .label("START")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let (sender, receiver) = async_channel::bounded(1);
    // ANCHOR: callback
    //
    // Wrap the new structure in Arc/Mutext
    let demo_struct = Arc::new(Mutex::new(DemoStruct {
        text: "Start".to_string(),
    }));

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |moved_button| {
        let sender = sender.clone();
        // Get mutable reference to the structure
        let arc_struct = Arc::clone(&demo_struct);

        moved_button.set_label("Working");

        gio::spawn_blocking(move || {
            let three_seconds = Duration::from_secs(3);
            thread::sleep(three_seconds);
            // Mutate the string on another thread
            arc_struct
                .lock()
                .unwrap()
                .mutate("Mutated on another thread".to_string());
            // Send the structe back
            sender.send_blocking(arc_struct).unwrap();
        });
    });

    glib::spawn_future_local(clone!(
        #[weak]
        button,
        async move {
            while let Ok(mutated_struct) = receiver.recv().await {
                // Set the label using the string from the received structure
                button.set_label(&mutated_struct.lock().unwrap().text);
            }
        }
    ));
    // ANCHOR_END: callback
    //
    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    // Present window
    window.present();
}
