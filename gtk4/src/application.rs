// Take a look at the license at the top of the repository in the LICENSE file.

use crate::prelude::*;
use crate::rt;
use crate::Application;
use gio::ApplicationFlags;
use glib::signal::SignalHandlerId;
use glib::translate::*;

use std::cell::RefCell;
use std::rc::Rc;

impl Application {
    #[doc(alias = "gtk_application_new")]
    pub fn new(application_id: Option<&str>, flags: ApplicationFlags) -> Self {
        skip_assert_initialized!();
        let app: Application = unsafe {
            from_glib_full(ffi::gtk_application_new(
                application_id.to_glib_none().0,
                flags.into_glib(),
            ))
        };
        Self::register_startup_hook(&app);
        app
    }

    pub(crate) fn register_startup_hook(app: &Application) {
        skip_assert_initialized!();
        let signalid: Rc<RefCell<Option<SignalHandlerId>>> = Rc::new(RefCell::new(None));
        {
            let signalid_ = signalid.clone();

            let id = app.connect_startup(move |app| {
                app.disconnect(
                    signalid_
                        .borrow_mut()
                        .take()
                        .expect("Signal ID went missing"),
                );
                unsafe { rt::set_initialized() }
            });
            *signalid.borrow_mut() = Some(id);
        }
    }
}

impl Default for Application {
    fn default() -> Self {
        let app = glib::object::Object::new::<Self>(&[]);
        Self::register_startup_hook(&app);
        app
    }
}
