use gtk::{
    glib::{self, clone},
    prelude::*,
};

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

    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("Dialog Example")
        .default_width(350)
        .default_height(70)
        .child(&button)
        .visible(true)
        .build();

    button.connect_clicked(clone!(
        #[weak]
        window,
        move |_| {
            gtk::glib::MainContext::default().spawn_local(dialog(window.clone()));
        }
    ));

    window.connect_close_request(move |window| {
        if let Some(application) = window.application() {
            application.remove_window(window);
        }
        glib::Propagation::Proceed
    });
}

async fn dialog<W: IsA<gtk::Window>>(window: W) {
    let question_dialog = gtk::AlertDialog::builder()
        .modal(true)
        .buttons(["Cancel", "Ok"])
        .message("What is your answer?")
        .build();

    let answer = question_dialog.choose_future(Some(&window)).await;

    let info_dialog = gtk::AlertDialog::builder()
        .modal(true)
        .message("You answered")
        .detail(format!("Your answer: {answer:?}").as_str())
        .build();

    info_dialog.show(Some(&window));
}
