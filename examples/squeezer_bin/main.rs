mod squeezer_bin;

use gtk::prelude::*;
use squeezer_bin::SqueezerBin;

fn main() {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.squeezer_bin")
        .build();

    application.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);
        let headerbar = gtk::HeaderBar::new();
        let mode_switch = gtk::Switch::new();
        let switch_label = gtk::Label::new(Some("keep aspect ratio"));
        let squeezer = SqueezerBin::new();
        squeezer.set_child(Some(&gtk::Label::new(Some("Hello World!"))));

        headerbar.pack_start(&mode_switch);
        headerbar.pack_start(&switch_label);

        mode_switch
            .bind_property("state", &squeezer, "keep-aspect-ratio")
            .build();

        window.set_titlebar(Some(&headerbar));
        window.set_child(Some(&squeezer));
        window.show();
    });

    application.run();
}
