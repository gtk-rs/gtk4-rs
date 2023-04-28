mod data_set;
use std::str::FromStr;

use gtk::prelude::*;
use gtk::{gdk, glib};

fn main() -> glib::ExitCode {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.flowbox"),
        Default::default(),
    );

    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .default_width(600)
        .default_height(600)
        .application(app)
        .title("FlowBox")
        .build();

    let flow_box = gtk::FlowBox::builder()
        .valign(gtk::Align::Start)
        .max_children_per_line(30)
        .min_children_per_line(4)
        .selection_mode(gtk::SelectionMode::None)
        .build();

    data_set::COLORS.iter().for_each(|color| {
        let color_widget = create_color_button(color);
        flow_box.insert(&color_widget, -1);
    });

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&flow_box)
        .build();

    window.set_child(Some(&scrolled_window));
    window.present();
}

fn create_color_button(color: &'static str) -> gtk::Button {
    let button = gtk::Button::new();
    let drawing_area = gtk::DrawingArea::builder()
        .content_height(24)
        .content_width(24)
        .build();

    let rgba = gdk::RGBA::from_str(color).unwrap();
    drawing_area.set_draw_func(move |_, cr, _width, _height| {
        GdkCairoContextExt::set_source_rgba(cr, &rgba);
        cr.paint().expect("Invalid cairo surface state");
    });
    button.set_child(Some(&drawing_area));

    button
}
