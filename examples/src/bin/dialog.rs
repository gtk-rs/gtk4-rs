//! # Dialog Example
//!
//! Example of how to obtain the response to a dialog as a future

use gtk::glib::clone;
use gtk::glib::signal::Inhibit;
use gtk::prelude::*;

use std::env::args;
use std::rc::Rc;

async fn dialog<W: IsA<gtk::Window>>(window: Rc<W>) {
    let question_dialog = gtk::MessageDialogBuilder::new()
        .transient_for(&*window)
        .modal(true)
        .buttons(gtk::ButtonsType::OkCancel)
        .text("What is your answer?")
        .build();

    let answer = question_dialog.run_future().await;
    question_dialog.close();

    let info_dialog = gtk::MessageDialogBuilder::new()
        .transient_for(&*window)
        .modal(true)
        .buttons(gtk::ButtonsType::Close)
        .text("You answered")
        .secondary_text(&format!("Your answer: {:?}", answer))
        .build();

    info_dialog.run_future().await;
    info_dialog.close();
}

fn build_ui(application: &gtk::Application) {
    let button = gtk::ButtonBuilder::new()
        .label("Open Dialog")
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();

    let window = Rc::new(
        gtk::ApplicationWindowBuilder::new()
            .application(application)
            .title("Dialog Example")
            .default_width(350)
            .default_height(70)
            .child(&button)
            .visible(true)
            .build(),
    );

    button.connect_clicked(clone!(@strong window =>
        move |_| {
            gtk::glib::MainContext::default().spawn_local(dialog(Rc::clone(&window)));
        }
    ));

    window.connect_close_request(move |window| {
        if let Some(application) = window.get_application() {
            application.remove_window(window);
        }
        Inhibit(false)
    });
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let application = gtk::ApplicationBuilder::new()
        .application_id("com.github.gtk-rs.examples.dialog")
        .build()?;

    application.connect_activate(build_ui);

    application.run(&args().collect::<Vec<_>>());

    Ok(())
}
