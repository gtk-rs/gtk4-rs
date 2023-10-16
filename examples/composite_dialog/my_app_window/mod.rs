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

    /// Callback handler for `notify::counter` signal.
    ///
    /// When counter property reach 3, a dialog pops up and present the user
    /// with 2 choices: Set the counter to 6 or reset the counter to 0.
    #[template_callback]
    fn on_counter_notify(&self, _p: &glib::ParamSpec) {
        // Check counter property and create a Dialog.
        if self.counter() == 3 {
            self.imp().dialog.present();
        }
    }

    /// Handler for dialog's response.
    ///
    /// In the callback handler, response type is i32 instead of gtk::ResponseType.
    /// As the bindings replaces the original i32 to an enum that makes more sense in
    /// Rust. Although, the template callback can't be changed.
    ///
    /// In the callback handler if we try to use a gtk::ResponseType as response's type, a runtime error is thrown
    /// when a button's dialog is clicked (a.k.a dialog emits the signal response) with the following message:
    ///
    /// *Wrong type for argument 1 in template callback \`counter_choice\`:
    /// ValueTypeMismatchError { actual: gint, requested: GtkResponseType }*
    #[template_callback]
    fn counter_choice(&self, response: i32) {
        self.imp().dialog.set_visible(false);

        match gtk::ResponseType::from(response) {
            gtk::ResponseType::Ok => self.set_counter(0),
            gtk::ResponseType::Other(35) => self.set_counter(6),
            gtk::ResponseType::DeleteEvent => (),
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
