mod squares_widget;

use gtk::glib;
use gtk::prelude::*;

use squares_widget::SquaresWidget;

fn main() -> glib::ExitCode {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.squares"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk::Application) {
    let win = gtk::ApplicationWindow::new(application);
    win.set_title(Some("Squares"));

    let widget = SquaresWidget::new();
    win.set_child(Some(&widget));
    win.present();
}
