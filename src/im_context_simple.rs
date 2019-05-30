// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use IMContextSimple;
use IsA;
use gtk_sys;
use glib::translate::*;
use std::path::Path;

pub trait IMContextSimpleExtManual: 'static {
    fn add_compose_file<P: AsRef<Path>>(&self, compose_file: P);
    //fn add_table(&self, data: &[u16], max_seq_len: u32, n_seqs: u32);
}

impl<O: IsA<IMContextSimple>> IMContextSimpleExtManual for O {
    fn add_compose_file<P: AsRef<Path>>(&self, compose_file: P) {
        unsafe {
            let compose_file = compose_file.as_ref();
            gtk_sys::gtk_im_context_simple_add_compose_file(self.as_ref().to_glib_none().0, compose_file.to_glib_none().0);
        }
    }

    /*fn add_table(&self, data: &[u16], max_seq_len: u32, n_seqs: u32) {
        assert!(max_seq_len * n_seqs < data.len() as u32);
        unsafe {
            gtk_sys::gtk_im_context_simple_add_table(self.as_ref().to_glib_none().0,
                                                 data.to_glib_none().0,
                                                 max_seq_len as i32,
                                                 n_seqs as i32);
        }
    }*/
}
