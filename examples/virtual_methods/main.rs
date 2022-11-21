mod base_button;
mod derived_button;

use crate::base_button::BaseButtonExt;
use gtk::{
    glib::{self, clone},
    prelude::*,
};

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.virtual_methods"),
        Default::default(),
    );

    application.connect_activate(|app| {
        let win = gtk::ApplicationWindow::new(app);
        let boxed = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        let base_button = base_button::BaseButton::new();
        let derived_button = derived_button::DerivedButton::new();

        base_button.connect_clicked(|b| {
            let ctx = glib::MainContext::default();
            ctx.spawn_local(clone!(@weak b => async move {
                b.async_method().await.unwrap();
            }));
        });
        derived_button.connect_clicked(|b| {
            let ctx = glib::MainContext::default();
            ctx.spawn_local(clone!(@weak b => async move {
                b.async_method().await.unwrap();
            }));
        });

        boxed.append(&base_button);
        boxed.append(&derived_button);
        win.set_child(Some(&boxed));
        win.show();
    });

    application.run();
}
