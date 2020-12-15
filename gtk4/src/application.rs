// Take a look at the license at the top of the repository in the LICENSE file.

use crate::rt;
use crate::Application;
use glib::translate::*;

impl Application {
    pub fn new(
        application_id: Option<&str>,
        flags: gio::ApplicationFlags,
    ) -> Result<Application, glib::BoolError> {
        skip_assert_initialized!();
        rt::init()?;
        unsafe {
            Option::from_glib_full(ffi::gtk_application_new(
                application_id.to_glib_none().0,
                flags.to_glib(),
            ))
            .ok_or_else(|| glib::glib_bool_error!("Failed to create application"))
        }
    }
}
