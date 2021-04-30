use gtk::glib::clone;
use gtk::prelude::*;

fn build_ui(application: &gtk::Application) {
    // Create a new window, set its title and default size
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("Grid Packing"));
    window.set_default_size(100, 100);

    // Here we construct the grid that is going contain our buttons and add margins
    let grid = gtk::Grid::new();
    grid.set_margin_start(6);
    grid.set_margin_end(6);
    grid.set_margin_top(6);
    grid.set_margin_bottom(6);

    // Add the grid in the window
    window.set_child(Some(&grid));

    // Create the first button, make it expand and put it into the grid at (0, 0)
    let button_1 = gtk::Button::with_label("Button 1");
    button_1.connect_clicked(move |_| println!("Hello World"));

    grid.attach(&button_1, 0, 0, 1, 1);

    // Create the second button, make it expand and put it into the grid at (1, 0)
    let button_2 = gtk::Button::with_label("Button 2");
    button_2.connect_clicked(move |_| println!("Hello World"));

    grid.attach(&button_2, 1, 0, 1, 1);

    // Create the quit button, make it expand and put it into the grid at (0, 1)
    let quit_button = gtk::Button::with_label("Quit");
    quit_button.connect_clicked(clone!(@weak window => move |_| window.destroy()));

    grid.attach(&quit_button, 0, 1, 2, 1);

    window.show();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.grid-packing"),
        Default::default(),
    );

    application.connect_activate(build_ui);
    application.run();
}
