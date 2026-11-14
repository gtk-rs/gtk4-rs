mod tracked_button;

use glib::closure_local;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, glib};
use tracked_button::TrackedButton;

const APP_ID: &str = "org.gtk_rs.GObjectSignals3";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

// ANCHOR: handler
fn build_ui(app: &Application) {
    let button = TrackedButton::new();
    button.set_text("press me");
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);

    // The handler reacts to "text-changed" by setting a *new* text value.
    // `set_text` borrows the same `RefCell` mutably. This only works because
    // `clicked()` dropped its borrow before emitting the signal - if it had
    // kept the borrow alive we would panic here at runtime with
    // `BorrowMutError: already borrowed`.
    button.connect_closure(
        "text-changed",
        false,
        closure_local!(move |button: TrackedButton, current: &str| {
            println!("saw text-changed: {current}");
            button.set_text(&format!("{current}!"));
        }),
    );
    // ANCHOR_END: handler

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    window.present();
}
