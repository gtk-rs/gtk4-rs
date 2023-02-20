//! Defines the implementation of our model

use gio::subclass::prelude::*;
use gtk::{gio, glib, prelude::*};

use std::cell::RefCell;

// Use `im-rc::Vector` here as it has much better insert/delete performance than a plain `Vec`.
use im_rc::Vector;

use crate::row_data::RowData;

#[derive(Debug, Default)]
pub struct Model(pub(super) RefCell<Vector<RowData>>);

/// Basic declaration of our type for the GObject type system
#[glib::object_subclass]
impl ObjectSubclass for Model {
    const NAME: &'static str = "Model";
    type Type = super::Model;
    type Interfaces = (gio::ListModel,);
}

impl ObjectImpl for Model {}

impl ListModelImpl for Model {
    fn item_type(&self) -> glib::Type {
        RowData::static_type()
    }
    fn n_items(&self) -> u32 {
        self.0.borrow().len() as u32
    }
    fn item(&self, position: u32) -> Option<glib::Object> {
        self.0
            .borrow()
            .get(position as usize)
            .map(|o| o.clone().upcast::<glib::Object>())
    }
}
