// Since Gtk4 v4.10 gtk4::Dialog is deprecated and gtk4-rs's examples minimum version is v4.10
#[allow(deprecated)]
mod imp;

use gtk::{glib, prelude::*, subclass::prelude::*};

glib::wrapper! {
    pub struct MyAppWindow(ObjectSubclass<imp::MyAppWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow;
}

#[gtk::template_callbacks]
impl MyAppWindow {
    pub fn new<P: IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    /// Callback handler for notify::label signal.
    ///
    /// When counter property reach 3, a dialog pops up and present the user
    /// with 2 choices: Set the counter to 6 or reset the counter to 0.
    #[template_callback]
    fn popup_dialog(&self, _p: &glib::ParamSpec) {
        // Check counter property and create a Dialog.
        if self.counter() == 3 {
            self.imp().popup.present();
        }
    }

    /// Handler for popup dialog's response.
    ///
    /// In the callback handler, response type is i32 instead of gtk::ResponseType.
    #[template_callback]
    fn counter_choice(&self, response: i32) {
        match gtk::ResponseType::from(response) {
            gtk::ResponseType::Ok => {
                self.set_counter(0);
                self.imp().popup.set_visible(false);
            }
            gtk::ResponseType::Other(35) => {
                self.set_counter(6);
                self.imp().popup.set_visible(false);
            }
            gtk::ResponseType::DeleteEvent => {
                self.imp().popup.set_visible(false);
            }
            _ => unimplemented!(),
        }
    }

    /// Callback handler for gtk::Button plus.
    #[template_callback]
    fn add_to_counter(&self, _button: &gtk::Button) {
        let n = self.counter() + 1;
        self.set_counter(n);
    }

    /// Callback handler for gtk::Button minus.
    #[template_callback]
    fn sub_to_counter(&self, _button: &gtk::Button) {
        let n = self.counter() - 1;
        self.set_counter(n);
    }
}
