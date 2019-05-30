// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use libc::c_char;
use gtk_sys;

pub struct RecentData {
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub mime_type: String,
    pub app_name: String,
    pub app_exec: String,
    pub groups: Vec<String>,
    pub is_private: bool,
}

#[doc(hidden)]
impl <'a> ToGlibPtr<'a, *mut gtk_sys::GtkRecentData> for RecentData {
    type Storage = (Box<gtk_sys::GtkRecentData>,
                    [Stash<'a, *mut c_char, Option<String>>; 2],
                    [Stash<'a, *mut c_char, String>; 3],
                    Stash<'a, *mut *mut c_char, [String]>);

    fn to_glib_none(&'a self) -> Stash<'a, *mut gtk_sys::GtkRecentData, Self> {
        let display_name = self.display_name.to_glib_none();
        let description = self.description.to_glib_none();
        let mime_type = self.mime_type.to_glib_none();
        let app_name = self.app_name.to_glib_none();
        let app_exec = self.app_exec.to_glib_none();
        let groups = self.groups.to_glib_none();

        let mut data = Box::new(gtk_sys::GtkRecentData {
            display_name: display_name.0,
            description: description.0,
            mime_type: mime_type.0,
            app_name: app_name.0,
            app_exec: app_exec.0,
            groups: groups.0,
            is_private: self.is_private.to_glib(),
        });

        Stash(&mut *data, (data, [display_name, description],
                                 [mime_type, app_name, app_exec],
                                 groups))
    }
}
