// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, Notebook, Widget};
use glib::translate::*;
use libc::c_int;

impl Notebook {
    #[doc(alias = "gtk_notebook_append_page")]
    pub fn append_page(
        &self,
        child: &impl IsA<Widget>,
        tab_label: Option<&impl IsA<Widget>>,
    ) -> u32 {
        unsafe {
            let ret = ffi::gtk_notebook_append_page(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                tab_label.map(|p| p.as_ref()).to_glib_none().0,
            );
            debug_assert!(ret >= 0);
            ret as u32
        }
    }

    #[doc(alias = "gtk_notebook_append_page_menu")]
    pub fn append_page_menu(
        &self,
        child: &impl IsA<Widget>,
        tab_label: Option<&impl IsA<Widget>>,
        menu_label: Option<&impl IsA<Widget>>,
    ) -> u32 {
        unsafe {
            let ret = ffi::gtk_notebook_append_page_menu(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                tab_label.map(|p| p.as_ref()).to_glib_none().0,
                menu_label.map(|p| p.as_ref()).to_glib_none().0,
            );
            debug_assert!(ret >= 0);
            ret as u32
        }
    }

    #[doc(alias = "gtk_notebook_get_current_page")]
    #[doc(alias = "get_current_page")]
    pub fn current_page(&self) -> Option<u32> {
        unsafe {
            let ret = ffi::gtk_notebook_get_current_page(self.to_glib_none().0);
            if ret >= 0 {
                Some(ret as u32)
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_notebook_get_n_pages")]
    #[doc(alias = "get_n_pages")]
    pub fn n_pages(&self) -> u32 {
        unsafe {
            let ret = ffi::gtk_notebook_get_n_pages(self.to_glib_none().0);
            debug_assert!(ret >= 0);
            ret as u32
        }
    }

    #[doc(alias = "gtk_notebook_get_nth_page")]
    #[doc(alias = "get_nth_page")]
    pub fn nth_page(&self, page_num: Option<u32>) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_nth_page(
                self.to_glib_none().0,
                page_num.map_or(-1, |n| n as c_int),
            ))
        }
    }

    #[doc(alias = "gtk_notebook_insert_page")]
    pub fn insert_page(
        &self,
        child: &impl IsA<Widget>,
        tab_label: Option<&impl IsA<Widget>>,
        position: Option<u32>,
    ) -> u32 {
        unsafe {
            let ret = ffi::gtk_notebook_insert_page(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                tab_label.map(|p| p.as_ref()).to_glib_none().0,
                position.map_or(-1, |n| n as c_int),
            );
            debug_assert!(ret >= 0);
            ret as u32
        }
    }

    #[doc(alias = "gtk_notebook_insert_page_menu")]
    pub fn insert_page_menu(
        &self,
        child: &impl IsA<Widget>,
        tab_label: Option<&impl IsA<Widget>>,
        menu_label: Option<&impl IsA<Widget>>,
        position: Option<u32>,
    ) -> u32 {
        unsafe {
            let ret = ffi::gtk_notebook_insert_page_menu(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                tab_label.map(|p| p.as_ref()).to_glib_none().0,
                menu_label.map(|p| p.as_ref()).to_glib_none().0,
                position.map_or(-1, |n| n as c_int),
            );
            debug_assert!(ret >= 0);
            ret as u32
        }
    }
    #[doc(alias = "gtk_notebook_page_num")]
    pub fn page_num(&self, child: &impl IsA<Widget>) -> Option<u32> {
        unsafe {
            let ret =
                ffi::gtk_notebook_page_num(self.to_glib_none().0, child.as_ref().to_glib_none().0);
            if ret >= 0 {
                Some(ret as u32)
            } else {
                None
            }
        }
    }
    #[doc(alias = "gtk_notebook_prepend_page")]
    pub fn prepend_page(
        &self,
        child: &impl IsA<Widget>,
        tab_label: Option<&impl IsA<Widget>>,
    ) -> u32 {
        unsafe {
            let ret = ffi::gtk_notebook_prepend_page(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                tab_label.map(|p| p.as_ref()).to_glib_none().0,
            );
            debug_assert!(ret >= 0);
            ret as u32
        }
    }

    #[doc(alias = "gtk_notebook_prepend_page_menu")]
    pub fn prepend_page_menu(
        &self,
        child: &impl IsA<Widget>,
        tab_label: Option<&impl IsA<Widget>>,
        menu_label: Option<&impl IsA<Widget>>,
    ) -> u32 {
        unsafe {
            let ret = ffi::gtk_notebook_prepend_page_menu(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                tab_label.map(|p| p.as_ref()).to_glib_none().0,
                menu_label.map(|p| p.as_ref()).to_glib_none().0,
            );
            debug_assert!(ret >= 0);
            ret as u32
        }
    }

    #[doc(alias = "gtk_notebook_remove_page")]
    pub fn remove_page(&self, page_num: Option<u32>) {
        unsafe {
            ffi::gtk_notebook_remove_page(
                self.to_glib_none().0,
                page_num.map_or(-1, |n| n as c_int),
            );
        }
    }

    #[doc(alias = "gtk_notebook_reorder_child")]
    pub fn reorder_child(&self, child: &impl IsA<Widget>, position: Option<u32>) {
        unsafe {
            ffi::gtk_notebook_reorder_child(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                position.map_or(-1, |n| n as c_int),
            );
        }
    }

    #[doc(alias = "gtk_notebook_set_current_page")]
    pub fn set_current_page(&self, page_num: Option<u32>) {
        unsafe {
            ffi::gtk_notebook_set_current_page(
                self.to_glib_none().0,
                page_num.map_or(-1, |n| n as c_int),
            );
        }
    }
}
