mod squeezer_bin;

use gtk::glib;
use gtk::prelude::*;
use squeezer_bin::SqueezerBin;

fn main() {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk_rs.examples.squeezer_bin")
        .build();

    application.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);
        let headerbar = gtk::HeaderBar::new();
        let mode_switch = gtk::Switch::new();
        let squeezer = SqueezerBin::new();

        headerbar.pack_start(&mode_switch);

        mode_switch.connect_state_notify(glib::clone!(@weak squeezer => move |switch| {
            squeezer.imp().keep_aspect_ratio.set(switch.state());
            squeezer.queue_resize();
        }));

        window.set_titlebar(Some(&headerbar));
        window.set_child(Some(&squeezer));
        window.show();
    });

    application.run();
}
