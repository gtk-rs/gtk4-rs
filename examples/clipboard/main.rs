use glib::clone;
use gtk::prelude::*;
use gtk::{gdk, gio, glib};

fn main() -> glib::ExitCode {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.clipboard"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("Clipboard")
        .default_width(660)
        .default_height(420)
        .build();

    let display = gdk::Display::default().unwrap();
    let clipboard = display.clipboard();

    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .spacing(24)
        .build();

    // The text copy/paste part
    let title = gtk::Label::builder()
        .label("Text")
        .halign(gtk::Align::Start)
        .build();
    title.add_css_class("title-2");
    container.append(&title);

    let text_container = gtk::Box::builder()
        .halign(gtk::Align::Center)
        .orientation(gtk::Orientation::Horizontal)
        .spacing(24)
        .build();

    let from_entry = gtk::Entry::builder()
        .placeholder_text("Type text to copy")
        .build();
    text_container.append(&from_entry);

    let copy_btn = gtk::Button::with_label("Copy");
    copy_btn.connect_clicked(clone!(@weak clipboard, @weak from_entry => move |_btn| {
        let text = from_entry.text();
        clipboard.set_text(&text);
    }));
    text_container.append(&copy_btn);

    let into_entry = gtk::Entry::new();
    text_container.append(&into_entry);

    let paste_btn = gtk::Button::with_label("Paste");
    paste_btn.connect_clicked(clone!(@weak clipboard, @weak into_entry => move |_btn| {
        clipboard.read_text_async(gio::Cancellable::NONE, clone!(@weak into_entry => move|res| {
            if let Ok(Some(text)) = res {
                into_entry.set_text(&text);
            }
        }));
    }));
    text_container.append(&paste_btn);
    container.append(&text_container);

    // The texture copy/paste part
    let title = gtk::Label::builder()
        .label("Texture")
        .halign(gtk::Align::Start)
        .build();
    title.add_css_class("title-2");
    container.append(&title);

    let texture_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .halign(gtk::Align::Center)
        .spacing(24)
        .build();

    let file = gio::File::for_path("./examples/clipboard/asset.png");
    let asset_paintable = gdk::Texture::from_file(&file).unwrap();

    let image_from = gtk::Image::builder()
        .pixel_size(96)
        .paintable(&asset_paintable)
        .build();
    texture_container.append(&image_from);
    let copy_texture_btn = gtk::Button::builder()
        .label("Copy")
        .valign(gtk::Align::Center)
        .build();
    copy_texture_btn.connect_clicked(clone!(@weak clipboard, @weak image_from => move |_btn| {
        let texture = image_from.paintable().and_downcast::<gdk::Texture>().unwrap();
        clipboard.set_texture(&texture);
    }));
    texture_container.append(&copy_texture_btn);

    let image_into = gtk::Image::builder()
        .pixel_size(96)
        .icon_name("image-missing")
        .build();
    texture_container.append(&image_into);
    let paste_texture_btn = gtk::Button::builder()
        .label("Paste")
        .valign(gtk::Align::Center)
        .build();
    paste_texture_btn.connect_clicked(clone!(@weak clipboard => move |_btn| {
        clipboard.read_texture_async(gio::Cancellable::NONE, clone!(@weak image_into => move |res| {
            if let Ok(Some(texture)) = res {
                image_into.set_paintable(Some(&texture));
            }
        }));
    }));
    texture_container.append(&paste_texture_btn);
    container.append(&texture_container);

    window.set_child(Some(&container));
    window.show();
}
