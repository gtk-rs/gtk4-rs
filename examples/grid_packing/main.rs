use gtk::glib::clone;
use gtk::prelude::*;

fn build_ui(application: &gtk::Application) {
    // Create a new window, and set its title
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("Grid Packing"));

    // Here we construct the grid that is going contain our buttons
    let grid = gtk::Grid::new();

    // Pack the grid in the window
    window.set_child(Some(&grid));

    // Create the first button and put it into the grid at (0, 0)
    let button_1 = gtk::Button::with_label("Button 1");
    button_1.connect_clicked(move |_| println!("Hello World"));
    grid.attach(&button_1, 0, 0, 1, 1);

    // Create the second button and put it into the grid at (1, 0)
    let button_2 = gtk::Button::with_label("Button 2");
    button_2.connect_clicked(move |_| println!("Hello World"));
    grid.attach(&button_2, 1, 0, 1, 1);

    // Create the exit button and put it into the grid at (0, 1)
    let exit_button = gtk::Button::with_label("Quit");
    exit_button.connect_clicked(clone!(@weak window => move |_| window.destroy()));
    grid.attach(&exit_button, 0, 1, 2, 1);

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
