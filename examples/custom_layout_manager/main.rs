mod custom_layout;
mod custom_layout_child;
mod simple_widget;
use std::str::FromStr;

use gtk::gdk;
use gtk::prelude::*;

const COLORS: [&str; 16] = [
    "red",
    "orange",
    "yellow",
    "green",
    "blue",
    "grey",
    "magenta",
    "lime",
    "yellow",
    "firebrick",
    "aqua",
    "purple",
    "tomato",
    "pink",
    "thistle",
    "maroon",
];
const TOTAL_COLORS: i32 = COLORS.len() as i32;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.custom_layout"),
        Default::default(),
    );

    application.connect_activate(|app| {
        let window = gtk::ApplicationWindow::builder()
            .default_width(600)
            .default_height(600)
            .application(app)
            .title("Custom Layout Manager")
            .build();

        let widget = simple_widget::SimpleWidget::new();
        for color in &COLORS {
            let rgba = gdk::RGBA::from_str(color).unwrap();
            let child = custom_layout_child::CustomLayoutChild::new(rgba);
            widget.add_child(&child);
        }

        window.set_child(Some(&widget));
        window.show();
    });

    application.run();
}
