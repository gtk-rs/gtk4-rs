use gtk::prelude::*;
use gtk::{  Adjustment, 
            Application, 
            ApplicationWindow, 
            Grid, 
            Label, 
            Orientation, 
            Scale
        };

fn main() {
    let application = Application::new(
        Some("com.github.gtk-rs.examples.builder_pattern"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}

// In this function we create the application window and the scales
fn build_ui(application: &Application) {
    // Let's first create the window.
    let window = ApplicationWindow::builder()
        .application(application)
        .title("Scale Example")
        .default_width(400)
        .default_height(300)
        .build();

    // We need 2 adjustments to describe the scales
    let horizontal_adjustment = Adjustment::new(
        0.0, // The value where the handle will be at the initial state
        0.0, // Lower bound
        100.0, // Upper bound
        0.0, // Step increment, keep it 0 if you don't want it to be operated by arrow keys
        0.0, // Page increment
        0.0 // Page size
    );

    let vertical_adjustment = Adjustment::new(
        75.0,// The value where the handle will be at the initial state
        0.0, // Lower bound
        100.0, // Upper bound
        5.0, // Step increment, use arrow keys to see the effect
        0.0, // Page increment
        0.0 // Page size
    );

    // Initiate the horizontal scale with horizontal orientation and the horizontal adjustment
    let horizontal_scale = Scale::new(
        Orientation::Horizontal, 
        Some(&horizontal_adjustment)
    );

    // Now if we want to take actions with the changed values of the scale, we'll have to implement a signal
    horizontal_scale.connect_local(
            "value-changed", 
            false, 
            move |args| {
            let slider = args[0]
                .get::<Scale>()
                .expect("The value needs to be of type `Scale`.");

            println!("Horizontal scale value: {:?}", slider.value());
            None
        })
        .expect("Could not connect to signal.");

    // For the vertical scale let's use the builder.
    let vertical_scale = Scale::builder()
        .orientation(Orientation::Vertical)
        .adjustment(&vertical_adjustment)
        .vexpand(true)
        .build();

    // We create a similar signal for the vertical scale too
    vertical_scale.connect_local(
            "value-changed", 
            false, 
            move |args| {
            let slider = args[0]
                .get::<Scale>()
                .expect("The value needs to be of type `Scale`.");

            println!("Vertical scale value: {:?}", slider.value());
            None
        })
        .expect("Could not connect to signal.");

    // To arrange everything in a presentable way we can use the grids.
    let grid = Grid::new();
    grid.set_column_spacing(10);
    grid.set_column_homogeneous(true);

    // Now let's put our scales in their places 
    grid.attach(&horizontal_scale, 0, 0, 1, 1);
    grid.attach(&vertical_scale, 1, 0, 1, 1);

    // We put 2 labels for our scales and put them beneath their respective scales.
    let horizontal_label = Label::new(Some("Horizontal scale"));
    let vertical_label = Label::new(Some("Vertical scale"));
    grid.attach(&horizontal_label, 0, 1, 1, 1);
    grid.attach(&vertical_label, 1, 1, 1, 1);

    // Finally attach the grid to the window and show it.
    window.set_child(Some(&grid));
    window.show();
}
