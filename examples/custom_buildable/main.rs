mod custom_buildable;

use custom_buildable::CustomBuildable;
use gtk::glib;
use gtk::prelude::*;

fn main() -> glib::ExitCode {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.buildable"),
        Default::default(),
    );

    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk::Application) {
    CustomBuildable::static_type();

    let ui_src = include_str!("window.ui");
    let builder = gtk::Builder::from_string(ui_src);

    let window: gtk::ApplicationWindow = builder.object("window").expect("Couldn't get window");
    application.add_window(&window);
    window.show();
}
