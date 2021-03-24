//! # Custom GtkEditable
//!
//! This example shows how to create a custom `gtk::Editable` which is the equivalent
//! of creating a custom text entry that can have tags shown on it.
//! It's a copy of the tagged entry demo from gtk4-demo

mod custom_editable;
pub mod custom_tag;

use custom_editable::CustomEditable;
use custom_tag::CustomTag;
use glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, glib};
use std::env::args;

static CSS: &str = "
tag {
    margin: 4px 0px;
    padding: 4px;
    border-radius: 4px;
    background: lightskyblue;
  }
  tag box {
    border-spacing: 4px;
  }
  tag label,
  tag image {
    color: white;
  }
  tag button {
    min-height: 0;
    min-width: 0;
    padding: 0;
    border: 1px solid white;
  }
  
  entry.tagged {
    border-spacing: 4px;
  }
";

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("Custom Editable"));
    window.set_default_size(500, 500);

    let container = gtk::Box::new(gtk::Orientation::Vertical, 12);
    container.set_valign(gtk::Align::Center);
    container.set_halign(gtk::Align::Center);

    let editable = CustomEditable::new();
    editable.set_halign(gtk::Align::Fill);

    container.append(&editable);

    let horizontal_container = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    horizontal_container.set_halign(gtk::Align::Center);

    let add_tag_button = gtk::Button::with_label("Add Tag");
    add_tag_button.set_halign(gtk::Align::Center);
    add_tag_button.connect_clicked(clone!(@weak editable => move |_btn| {
        let tag = CustomTag::new("Blue");
        tag.connect_local_id(custom_tag::imp::CustomTag::signals()[0].signal_id(), None, false, clone!(@weak editable, @weak tag => @default-return None, move |_args| {
            editable.remove_tag(&tag);
            None
        })).unwrap();
        tag.connect_local_id(custom_tag::imp::CustomTag::signals()[1].signal_id(), None, false, move |_args| {
            println!("Tag clicked");
            None
        }).unwrap();
        editable.add_tag(&tag);
    }));
    horizontal_container.append(&add_tag_button);

    let show_spinner = gtk::CheckButtonBuilder::new()
        .halign(gtk::Align::End)
        .label("Show Spinner")
        .build();
    show_spinner
        .bind_property("active", &editable, "show-spinner")
        .build();
    horizontal_container.append(&show_spinner);

    container.append(&horizontal_container);
    window.set_child(Some(&container));
    window.show();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.editable"),
        Default::default(),
    )
    .expect("Initialization failed...");

    let provider = gtk::CssProvider::new();
    provider.load_from_data(CSS.as_bytes());
    gtk::StyleContext::add_provider_for_display(
        &gdk::Display::get_default().unwrap(),
        &provider,
        800,
    );

    application.connect_activate(build_ui);

    application.run(&args().collect::<Vec<_>>());
}
