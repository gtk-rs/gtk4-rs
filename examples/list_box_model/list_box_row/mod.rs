mod imp;

use gtk::glib;

use crate::row_data::RowData;

glib::wrapper! {
    pub struct ListBoxRow(ObjectSubclass<imp::ListBoxRow>)
        @extends gtk::Widget, gtk::ListBoxRow,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl ListBoxRow {
    pub fn new(row_data: &RowData) -> Self {
        glib::Object::builder()
            .property("row-data", &row_data)
            .build()
    }
}
