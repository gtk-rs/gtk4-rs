use gtk::{
    glib::{self, clone},
    prelude::*,
};

fn main() {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.scale")
        .build();
    application.connect_activate(build_ui);
    application.run();
}

// In this function we create the application window and the scales
fn build_ui(application: &gtk::Application) {
    // Let's first create the window.
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("Scale")
        .default_width(400)
        .default_height(300)
        .build();

    // A label to update the current value of the last adjusted scale
    let update_label = gtk::Label::default();

    // We need 2 adjustments to describe the scales
    let horizontal_adjustment = gtk::Adjustment::new(
        0.0,   // The value where the handle will be at the initial state
        0.0,   // Lower bound
        100.0, // Upper bound
        0.0,   // Step increment, keep it 0 if you don't want it to be operated by arrow keys
        0.0,   // Page increment
        0.0,   // Page size
    );

    let vertical_adjustment = gtk::Adjustment::new(
        75.0,  // The value where the handle will be at the initial state
        0.0,   // Lower bound
        100.0, // Upper bound
        5.0,   // Step increment, use arrow keys to see the effect
        0.0,   // Page increment
        0.0,   // Page size
    );

    // Initiate the horizontal scale with horizontal orientation and the horizontal adjustment
    let horizontal_scale =
        gtk::Scale::new(gtk::Orientation::Horizontal, Some(&horizontal_adjustment));

    // Now if we want to take actions with the changed values of the scale, we'll have to implement a signal
    horizontal_scale.connect_value_changed(clone!(
        #[weak]
        update_label,
        move |slider| {
            update_label.set_text(&format!("Horizontal scale value: {:.2}", slider.value()));
        }
    ));

    // Now for the vertical scale let's use the builder
    let vertical_scale = gtk::Scale::builder()
        .orientation(gtk::Orientation::Vertical)
        .adjustment(&vertical_adjustment)
        .vexpand(true)
        .build();

    // To create a similar signal for the vertical scale
    vertical_scale.connect_value_changed(clone!(
        #[weak]
        update_label,
        move |slider| {
            update_label.set_text(&format!("Vertical scale value: {:.2}", slider.value()));
        }
    ));

    // To arrange everything in a presentable way we can use the grids
    let grid = gtk::Grid::builder()
        .column_spacing(10)
        .column_homogeneous(true)
        .build();

    // Now let's put our scales in their places
    grid.attach(&horizontal_scale, 0, 1, 1, 1);
    grid.attach(&vertical_scale, 1, 1, 1, 1);

    // We put 2 labels for our scales and put them beneath their respective scales
    let horizontal_label = gtk::Label::new(Some("Horizontal scale"));
    let vertical_label = gtk::Label::new(Some("Vertical scale"));
    grid.attach(&horizontal_label, 0, 0, 1, 1);
    grid.attach(&vertical_label, 1, 0, 1, 1);
    // Attach the label where we update the latest scale values
    grid.attach(&update_label, 0, 2, 2, 1);

    // Finally attach the grid to the window and show it
    window.set_child(Some(&grid));
    window.present();
}
