mod imp;

use gtk::{glib, prelude::*, subclass::prelude::ObjectSubclassIsExt};

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
    #[allow(deprecated)]
    #[template_callback]
    fn popup_dialog(&self, _p: &glib::ParamSpec) {
        // Check counter property and create a Dialog.
        if self.counter() == 3 {
            let dial = gtk::Dialog::with_buttons(
                Some("Counter value is 3"),
                Some(self),
                gtk::DialogFlags::MODAL,
                &[
                    ("Set counter to 6", ResponseType::Other(35)),
                    ("Reset counter", ResponseType::Ok),
                ],
            );
            dial.set_transient_for(Some(self));

            let app = self.clone();

            // Closure handling response signal from gtk::Dialog.
            // The signature of the function differ from the documentation for response signal.
            // gtk-rs use an i32 instead of a [`gtk::ResponseType`] as a response signal.
            dial.connect_closure(
                "response",
                false,
                glib::closure_local!(move |d: &gtk::Dialog, response: i32| {
                    match ResponseType::from(response) {
                        ResponseType::Other(35) => {
                            app.set_counter(6);
                            d.close();
                        }
                        ResponseType::Ok => {
                            app.set_counter(0);
                            d.close();
                        }
                        _ => (),
                    }
                }),
            );

            dial.present();
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
