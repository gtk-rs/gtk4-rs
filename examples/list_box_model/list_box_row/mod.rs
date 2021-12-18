mod imp;

use gtk::{self, glib};

use crate::row_data::RowData;

glib::wrapper! {
    pub struct ListBoxRow(ObjectSubclass<imp::ListBoxRow>)
        @extends gtk::Widget, gtk::ListBoxRow;
}

impl ListBoxRow {
    pub fn new(row_data: &RowData, parent_window: &gtk::Window) -> Self {
        glib::Object::new(&[("rowdata", &row_data), ("parent-window", &parent_window)]).unwrap()
    }
}
