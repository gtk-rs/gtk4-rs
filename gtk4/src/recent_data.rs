// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use libc::c_char;

#[doc(alias = "GtkRecentData")]
pub struct RecentData {
    display_name: Option<String>,
    description: Option<String>,
    mime_type: String,
    app_name: String,
    app_exec: String,
    groups: Vec<String>,
    is_private: bool,
}

impl RecentData {
    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn mime_type(&self) -> &str {
        &self.mime_type
    }

    pub fn app_name(&self) -> &str {
        &self.app_name
    }

    pub fn app_exec(&self) -> &str {
        &self.app_exec
    }

    pub fn groups(&self) -> &Vec<String> {
        &self.groups
    }

    pub fn is_private(&self) -> bool {
        self.is_private
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *mut ffi::GtkRecentData> for RecentData {
    #[allow(clippy::type_complexity)]
    type Storage = (
        Box<ffi::GtkRecentData>,
        [Stash<'a, *mut c_char, Option<String>>; 2],
        [Stash<'a, *mut c_char, String>; 3],
        Stash<'a, *mut *mut c_char, [String]>,
    );

    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::GtkRecentData, Self> {
        let display_name = self.display_name.to_glib_none();
        let description = self.description.to_glib_none();
        let mime_type = self.mime_type.to_glib_none();
        let app_name = self.app_name.to_glib_none();
        let app_exec = self.app_exec.to_glib_none();
        let groups = self.groups.to_glib_none();

        let mut data = Box::new(ffi::GtkRecentData {
            display_name: display_name.0,
            description: description.0,
            mime_type: mime_type.0,
            app_name: app_name.0,
            app_exec: app_exec.0,
            groups: groups.0,
            is_private: self.is_private.into_glib(),
        });

        Stash(
            &mut *data,
            (
                data,
                [display_name, description],
                [mime_type, app_name, app_exec],
                groups,
            ),
        )
    }
}
