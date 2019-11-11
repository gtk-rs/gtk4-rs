// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::IsA;
use gtk_sys;
use libc::c_int;
use Notebook;
use Widget;

pub trait NotebookExtManual: 'static {
    fn append_page<T: IsA<Widget>, U: IsA<Widget>>(&self, child: &T, tab_label: Option<&U>) -> u32;

    fn append_page_menu<T, U, V>(
        &self,
        child: &T,
        tab_label: Option<&U>,
        menu_label: Option<&V>,
    ) -> u32
    where
        T: IsA<Widget>,
        U: IsA<Widget>,
        V: IsA<Widget>;

    fn get_current_page(&self) -> Option<u32>;

    fn get_n_pages(&self) -> u32;

    fn get_nth_page(&self, page_num: Option<u32>) -> Option<Widget>;

    fn insert_page<T, U>(&self, child: &T, tab_label: Option<&U>, position: Option<u32>) -> u32
    where
        T: IsA<Widget>,
        U: IsA<Widget>;

    fn insert_page_menu<T, U, V>(
        &self,
        child: &T,
        tab_label: Option<&U>,
        menu_label: Option<&V>,
        position: Option<u32>,
    ) -> u32
    where
        T: IsA<Widget>,
        U: IsA<Widget>,
        V: IsA<Widget>;

    fn page_num<T: IsA<Widget>>(&self, child: &T) -> Option<u32>;

    fn prepend_page<T, U>(&self, child: &T, tab_label: Option<&U>) -> u32
    where
        T: IsA<Widget>,
        U: IsA<Widget>;

    fn prepend_page_menu<T, U, V>(
        &self,
        child: &T,
        tab_label: Option<&U>,
        menu_label: Option<&V>,
    ) -> u32
    where
        T: IsA<Widget>,
        U: IsA<Widget>,
        V: IsA<Widget>;

    fn remove_page(&self, page_num: Option<u32>);

    fn reorder_child<T: IsA<Widget>>(&self, child: &T, position: Option<u32>);

    fn set_current_page(&self, page_num: Option<u32>);
}

impl<O: IsA<Notebook>> NotebookExtManual for O {
    fn append_page<T: IsA<Widget>, U: IsA<Widget>>(&self, child: &T, tab_label: Option<&U>) -> u32 {
        unsafe {
            let ret = gtk_sys::gtk_notebook_append_page(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                tab_label.map(|p| p.as_ref()).to_glib_none().0,
            );
            assert!(ret >= 0);
            ret as u32
        }
    }

    fn append_page_menu<T, U, V>(
        &self,
        child: &T,
        tab_label: Option<&U>,
        menu_label: Option<&V>,
    ) -> u32
    where
        T: IsA<Widget>,
        U: IsA<Widget>,
        V: IsA<Widget>,
    {
        unsafe {
            let ret = gtk_sys::gtk_notebook_append_page_menu(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                tab_label.map(|p| p.as_ref()).to_glib_none().0,
                menu_label.map(|p| p.as_ref()).to_glib_none().0,
            );
            assert!(ret >= 0);
            ret as u32
        }
    }

    fn get_current_page(&self) -> Option<u32> {
        unsafe {
            let ret = gtk_sys::gtk_notebook_get_current_page(self.as_ref().to_glib_none().0);
            if ret >= 0 {
                Some(ret as u32)
            } else {
                None
            }
        }
    }

    fn get_n_pages(&self) -> u32 {
        unsafe {
            let ret = gtk_sys::gtk_notebook_get_n_pages(self.as_ref().to_glib_none().0);
            assert!(ret >= 0);
            ret as u32
        }
    }

    fn get_nth_page(&self, page_num: Option<u32>) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_notebook_get_nth_page(
                self.as_ref().to_glib_none().0,
                page_num.map_or(-1, |n| n as c_int),
            ))
        }
    }

    fn insert_page<T, U>(&self, child: &T, tab_label: Option<&U>, position: Option<u32>) -> u32
    where
        T: IsA<Widget>,
        U: IsA<Widget>,
    {
        unsafe {
            let ret = gtk_sys::gtk_notebook_insert_page(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                tab_label.map(|p| p.as_ref()).to_glib_none().0,
                position.map_or(-1, |n| n as c_int),
            );
            assert!(ret >= 0);
            ret as u32
        }
    }

    fn insert_page_menu<T, U, V>(
        &self,
        child: &T,
        tab_label: Option<&U>,
        menu_label: Option<&V>,
        position: Option<u32>,
    ) -> u32
    where
        T: IsA<Widget>,
        U: IsA<Widget>,
        V: IsA<Widget>,
    {
        unsafe {
            let ret = gtk_sys::gtk_notebook_insert_page_menu(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                tab_label.map(|p| p.as_ref()).to_glib_none().0,
                menu_label.map(|p| p.as_ref()).to_glib_none().0,
                position.map_or(-1, |n| n as c_int),
            );
            assert!(ret >= 0);
            ret as u32
        }
    }

    fn page_num<T: IsA<Widget>>(&self, child: &T) -> Option<u32> {
        unsafe {
            let ret = gtk_sys::gtk_notebook_page_num(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
            if ret >= 0 {
                Some(ret as u32)
            } else {
                None
            }
        }
    }

    fn prepend_page<T, U>(&self, child: &T, tab_label: Option<&U>) -> u32
    where
        T: IsA<Widget>,
        U: IsA<Widget>,
    {
        unsafe {
            let ret = gtk_sys::gtk_notebook_prepend_page(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                tab_label.map(|p| p.as_ref()).to_glib_none().0,
            );
            assert!(ret >= 0);
            ret as u32
        }
    }

    fn prepend_page_menu<T, U, V>(
        &self,
        child: &T,
        tab_label: Option<&U>,
        menu_label: Option<&V>,
    ) -> u32
    where
        T: IsA<Widget>,
        U: IsA<Widget>,
        V: IsA<Widget>,
    {
        unsafe {
            let ret = gtk_sys::gtk_notebook_prepend_page_menu(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                tab_label.map(|p| p.as_ref()).to_glib_none().0,
                menu_label.map(|p| p.as_ref()).to_glib_none().0,
            );
            assert!(ret >= 0);
            ret as u32
        }
    }

    fn remove_page(&self, page_num: Option<u32>) {
        unsafe {
            gtk_sys::gtk_notebook_remove_page(
                self.as_ref().to_glib_none().0,
                page_num.map_or(-1, |n| n as c_int),
            );
        }
    }

    fn reorder_child<T: IsA<Widget>>(&self, child: &T, position: Option<u32>) {
        unsafe {
            gtk_sys::gtk_notebook_reorder_child(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                position.map_or(-1, |n| n as c_int),
            );
        }
    }

    fn set_current_page(&self, page_num: Option<u32>) {
        unsafe {
            gtk_sys::gtk_notebook_set_current_page(
                self.as_ref().to_glib_none().0,
                page_num.map_or(-1, |n| n as c_int),
            );
        }
    }
}
