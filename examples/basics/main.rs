use gtk4::prelude::*;

fn main() {
    let application = gtk4::Application::new(
        Some("com.github.gtk4-rs.examples.basic"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(application);

    window.set_title(Some("First GTK4 Program"));
    window.set_default_size(350, 70);

    let button = gtk4::Button::with_label("Click me!");

    window.set_child(Some(&button));

    window.show();
}
