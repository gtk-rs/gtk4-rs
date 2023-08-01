use gtk::glib::{self, clone};
use gtk::prelude::*;

use std::rc::Rc;

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.dialog")
        .build();

    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk::Application) {
    let button = gtk::Button::builder()
        .label("Open Dialog")
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();

    let window = Rc::new(
        gtk::ApplicationWindow::builder()
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
        if let Some(application) = window.application() {
            application.remove_window(window);
        }
        glib::Propagation::Proceed
    });
}

async fn dialog<W: IsA<gtk::Window>>(window: Rc<W>) {
    let question_dialog = gtk::AlertDialog::builder()
        .modal(true)
        .buttons(["Cancel", "Ok"])
        .message("What is your answer?")
        .build();

    let answer = question_dialog.choose_future(Some(&*window)).await;

    let info_dialog = gtk::AlertDialog::builder()
        .modal(true)
        .message("You answered")
        .detail(format!("Your answer: {answer:?}"))
        .build();

    info_dialog.show(Some(&*window));
}
