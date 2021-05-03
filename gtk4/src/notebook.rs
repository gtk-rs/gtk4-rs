// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Notebook, Widget};
use glib::translate::*;
use glib::IsA;
use libc::c_int;

pub trait NotebookExtManual: 'static {
    #[doc(alias = "gtk_notebook_append_page")]
    fn append_page<T: IsA<Widget>, U: IsA<Widget>>(&self, child: &T, tab_label: Option<&U>) -> u32;

    #[doc(alias = "gtk_notebook_append_page_menu")]
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

    #[doc(alias = "gtk_notebook_get_current_page")]
    #[doc(alias = "get_current_page")]
    fn current_page(&self) -> Option<u32>;

    #[doc(alias = "gtk_notebook_get_n_pages")]
    #[doc(alias = "get_n_pages")]
    fn n_pages(&self) -> u32;

    #[doc(alias = "gtk_notebook_get_nth_page")]
    #[doc(alias = "get_nth_page")]
    fn nth_page(&self, page_num: Option<u32>) -> Option<Widget>;

    #[doc(alias = "gtk_notebook_insert_page")]
    fn insert_page<T, U>(&self, child: &T, tab_label: Option<&U>, position: Option<u32>) -> u32
    where
        T: IsA<Widget>,
        U: IsA<Widget>;

    #[doc(alias = "gtk_notebook_insert_page_menu")]
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

    #[doc(alias = "gtk_notebook_page_num")]
    fn page_num<T: IsA<Widget>>(&self, child: &T) -> Option<u32>;

    #[doc(alias = "gtk_notebook_prepend_page")]
    fn prepend_page<T, U>(&self, child: &T, tab_label: Option<&U>) -> u32
    where
        T: IsA<Widget>,
        U: IsA<Widget>;

    #[doc(alias = "gtk_notebook_prepend_page_menu")]
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

    #[doc(alias = "gtk_notebook_remove_page")]
    fn remove_page(&self, page_num: Option<u32>);

    #[doc(alias = "gtk_notebook_reorder_child")]
    fn reorder_child<T: IsA<Widget>>(&self, child: &T, position: Option<u32>);

    #[doc(alias = "gtk_notebook_set_current_page")]
    fn set_current_page(&self, page_num: Option<u32>);
}

impl<O: IsA<Notebook>> NotebookExtManual for O {
    fn append_page<T: IsA<Widget>, U: IsA<Widget>>(&self, child: &T, tab_label: Option<&U>) -> u32 {
        unsafe {
            let ret = ffi::gtk_notebook_append_page(
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
            let ret = ffi::gtk_notebook_append_page_menu(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                tab_label.map(|p| p.as_ref()).to_glib_none().0,
                menu_label.map(|p| p.as_ref()).to_glib_none().0,
            );
            assert!(ret >= 0);
            ret as u32
        }
    }

    fn current_page(&self) -> Option<u32> {
        unsafe {
            let ret = ffi::gtk_notebook_get_current_page(self.as_ref().to_glib_none().0);
            if ret >= 0 {
                Some(ret as u32)
            } else {
                None
            }
        }
    }

    fn n_pages(&self) -> u32 {
        unsafe {
            let ret = ffi::gtk_notebook_get_n_pages(self.as_ref().to_glib_none().0);
            assert!(ret >= 0);
            ret as u32
        }
    }

    fn nth_page(&self, page_num: Option<u32>) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_nth_page(
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
            let ret = ffi::gtk_notebook_insert_page(
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
            let ret = ffi::gtk_notebook_insert_page_menu(
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
            let ret = ffi::gtk_notebook_page_num(
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
            let ret = ffi::gtk_notebook_prepend_page(
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
            let ret = ffi::gtk_notebook_prepend_page_menu(
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
            ffi::gtk_notebook_remove_page(
                self.as_ref().to_glib_none().0,
                page_num.map_or(-1, |n| n as c_int),
            );
        }
    }

    fn reorder_child<T: IsA<Widget>>(&self, child: &T, position: Option<u32>) {
        unsafe {
            ffi::gtk_notebook_reorder_child(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                position.map_or(-1, |n| n as c_int),
            );
        }
    }

    fn set_current_page(&self, page_num: Option<u32>) {
        unsafe {
            ffi::gtk_notebook_set_current_page(
                self.as_ref().to_glib_none().0,
                page_num.map_or(-1, |n| n as c_int),
            );
        }
    }
}
