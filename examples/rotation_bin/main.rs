mod rotation_bin;

use gtk::{glib, glib::clone, prelude::*};
use rotation_bin::{Rotation, RotationBin};

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.rotation_bin")
        .build();

    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .margin_bottom(6)
        .margin_top(6)
        .margin_start(6)
        .margin_end(6)
        .build();

    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(6)
        .margin_bottom(6)
        .margin_top(6)
        .margin_start(6)
        .margin_end(6)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();
    let rotation_bin = RotationBin::new();
    let img = gtk::Image::builder()
        .pixel_size(128)
        .icon_name("audio-x-generic")
        .build();

    let interactive_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(6)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();
    interactive_box.append(&img);
    rotation_bin.set_child(Some(&interactive_box));

    let grid = gtk::Grid::builder()
        .margin_start(6)
        .margin_end(6)
        .margin_top(6)
        .margin_bottom(6)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(6)
        .column_spacing(6)
        .build();

    let clockwise = gtk::Button::with_label("Rotate clockwise");
    let counter_clockwise = gtk::Button::with_label("Rotate counter clockwise");

    clockwise.connect_clicked(clone!(@weak rotation_bin => move |_| {
        rotation_bin.rotate_clockwise()
    }));
    counter_clockwise.connect_clicked(clone!(@weak rotation_bin => move |_| {
        rotation_bin.rotate_counter_clockwise()
    }));

    interactive_box.append(&clockwise);
    interactive_box.append(&counter_clockwise);

    let normal = gtk::Button::with_label("Reset rotation to 0 degrees");
    let deg90 = gtk::Button::with_label("Reset rotation to 90 degrees");
    let deg180 = gtk::Button::with_label("Reset rotation to 180 degrees");
    let deg270 = gtk::Button::with_label("Reset rotation to 270 degrees");

    normal.connect_clicked(clone!(@weak rotation_bin => move |_| {
        rotation_bin.set_rotation(Rotation::Normal)
    }));
    deg90.connect_clicked(clone!(@weak rotation_bin => move |_| {
        rotation_bin.set_rotation(Rotation::Deg90)
    }));
    deg180.connect_clicked(clone!(@weak rotation_bin => move |_| {
        rotation_bin.set_rotation(Rotation::Deg180)
    }));
    deg270.connect_clicked(clone!(@weak rotation_bin => move |_| {
        rotation_bin.set_rotation(Rotation::Deg270)
    }));

    grid.attach(&normal, 0, 1, 1, 1);
    grid.attach(&deg90, 1, 1, 1, 1);
    grid.attach(&deg180, 0, 2, 1, 1);
    grid.attach(&deg270, 1, 2, 1, 1);

    vbox.append(&rotation_bin);
    vbox.append(&grid);

    window.set_child(Some(&vbox));

    window.present();
}
