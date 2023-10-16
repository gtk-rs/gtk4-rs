mod custom_editable;
pub mod custom_tag;

use custom_editable::CustomEditable;
use custom_tag::CustomTag;
use gtk::{
    gdk,
    glib::{self, clone},
    prelude::*,
    subclass::prelude::*,
};

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.editable")
        .build();

    application.connect_startup(|_| {
        let provider = gtk::CssProvider::new();
        provider.load_from_string(include_str!("style.css"));
        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().unwrap(),
            &provider,
            800,
        );
    });

    application.connect_activate(build_ui);
    application.run()
}

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
        }));
        tag.connect_local_id(custom_tag::imp::CustomTag::signals()[1].signal_id(), None, false, move |_args| {
            println!("Tag clicked");
            None
        });
        editable.add_tag(&tag);
    }));
    horizontal_container.append(&add_tag_button);

    let show_spinner = gtk::CheckButton::builder()
        .halign(gtk::Align::End)
        .label("Show Spinner")
        .build();
    show_spinner
        .bind_property("active", &editable, "show-spinner")
        .build();
    horizontal_container.append(&show_spinner);

    container.append(&horizontal_container);
    window.set_child(Some(&container));
    window.present();
}
