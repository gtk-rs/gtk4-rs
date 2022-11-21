pub mod imp;

use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct CustomTag(ObjectSubclass<imp::CustomTag>) @extends gtk::Widget;
}

impl CustomTag {
    pub fn new(label: &str) -> Self {
        glib::Object::new(&[("label", &label), ("has-close-button", &true)])
    }

    pub fn set_has_close_button(&self, has_close_button: bool) {
        let imp = self.imp();
        if imp.has_close_button.get() == has_close_button {
            return;
        }

        if has_close_button {
            let button = gtk::Button::builder()
                .halign(gtk::Align::Center)
                .valign(gtk::Align::Center)
                .has_frame(false)
                .build();
            button.connect_clicked(clone!(@weak self as tag => move |_btn| {
                tag.emit_by_name::<()>("closed", &[]);
            }));
            let icon = gtk::Image::from_icon_name("window-close-symbolic");
            button.set_child(Some(&icon));

            imp.container.append(&button);
            imp.button.replace(Some(button));
        } else if let Some(button) = imp.button.borrow_mut().take() {
            imp.container.remove(&button);
        }
        imp.has_close_button.set(has_close_button);
    }
}
