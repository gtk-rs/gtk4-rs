use gtk::gio;
use gtk::{glib::signal::Inhibit, prelude::*};
use gtk::{Align, Application, ApplicationWindowBuilder, SwitchBuilder};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default())
        .expect("Initialization failed...");
    app.connect_activate(|app| on_activate(app));

    // Get command-line arguments
    let args: Vec<String> = std::env::args().collect();
    // Run the application
    app.run(&args);
}

// When the application is launched…
fn on_activate(application: &Application) {
    // … create a new window …
    let window = ApplicationWindowBuilder::new()
        .application(application)
        .title("My GTK App")
        .build();

    // ANCHOR: switch
    // Create a switch
    let switch = SwitchBuilder::new()
        .margin_top(48)
        .margin_bottom(48)
        .margin_start(48)
        .margin_end(48)
        .valign(Align::Center)
        .halign(Align::Center)
        .build();
    // ANCHOR_END: switch
    let settings = gio::Settings::new("org.gtk.example");
    switch.connect_state_set(|_, is_enabled| {
        // We don't want to inhibit the signal from being propagated to the default handler
        Inhibit(false)
    });

    // Add button
    window.set_child(Some(&switch));
    window.present();
}
