// gtk::Dialog was deprecated and applications are supposed
// to use plain gtk::Window and structure it however they wish.
#![allow(deprecated)]
use std::cell::RefCell;

use gtk::{
    glib::{self, clone},
    prelude::*,
    subclass::prelude::*,
};

use crate::row_data::RowData;

#[derive(Default, glib::Properties, Debug)]
#[properties(wrapper_type = super::ListBoxRow)]
pub struct ListBoxRow {
    #[property(get, set, construct_only)]
    row_data: RefCell<Option<RowData>>,
}

#[glib::object_subclass]
impl ObjectSubclass for ListBoxRow {
    const NAME: &'static str = "ExListBoxRow";
    type ParentType = gtk::ListBoxRow;
    type Type = super::ListBoxRow;
}

#[glib::derived_properties]
impl ObjectImpl for ListBoxRow {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();

        let item = self.row_data.borrow();
        let item = item.as_ref().cloned().unwrap();

        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

        // Create the label and spin button that shows the two values
        // of the item. We bind the properties for the two values to the
        // corresponding properties of the widgets so that they are automatically
        // updated whenever the item is changing. By specifying SYNC_CREATE the
        // widget will automatically get the initial value of the item set.
        //
        // In case of the spin button the binding is bidirectional, that is any
        // change of value in the spin button will be automatically reflected in
        // the item.
        let label = gtk::Label::default();
        item.bind_property("name", &label, "label")
            .sync_create()
            .build();
        hbox.append(&label);
        let spin_button = gtk::SpinButton::with_range(0.0, 100.0, 1.0);
        item.bind_property("count", &spin_button, "value")
            .sync_create()
            .bidirectional()
            .build();
        hbox.append(&spin_button);

        // When the edit button is clicked, a new modal dialog is created for editing
        // the corresponding row
        let edit_button = gtk::Button::with_label("Edit");
        edit_button.connect_clicked(clone!(
            #[weak]
            item,
            #[weak]
            obj,
            move |_| {
                let parent_window = obj.root().and_downcast::<gtk::Window>();
                let dialog = gtk::Dialog::with_buttons(
                    Some("Edit Item"),
                    parent_window.as_ref(),
                    gtk::DialogFlags::MODAL,
                    &[("Close", gtk::ResponseType::Close)],
                );
                dialog.set_default_response(gtk::ResponseType::Close);
                dialog.connect_response(|dialog, _| dialog.close());

                let content_area = dialog.content_area();

                // Similarly to the label and spin button inside the listbox, the text entry
                // and spin button in the edit dialog are connected via property bindings to
                // the item. Any changes will be immediately reflected inside the item and
                // by the listbox
                let entry = gtk::Entry::new();
                item.bind_property("name", &entry, "text")
                    .sync_create()
                    .bidirectional()
                    .build();

                // Activating the entry (enter) will send response `ResponseType::Close` to the dialog
                entry.connect_activate(clone!(
                    #[weak]
                    dialog,
                    move |_| {
                        dialog.response(gtk::ResponseType::Close);
                    }
                ));
                content_area.append(&entry);

                let spin_button = gtk::SpinButton::with_range(0.0, 100.0, 1.0);
                item.bind_property("count", &spin_button, "value")
                    .sync_create()
                    .bidirectional()
                    .build();
                content_area.append(&spin_button);

                dialog.present()
            }
        ));
        hbox.append(&edit_button);

        obj.set_child(Some(&hbox));

        // When a row is activated (select + enter) we simply emit the clicked
        // signal on the corresponding edit button to open the edit dialog
        obj.connect_activate(clone!(
            #[weak]
            edit_button,
            move |_| {
                edit_button.emit_clicked();
            }
        ));
    }
}

impl WidgetImpl for ListBoxRow {}
impl ListBoxRowImpl for ListBoxRow {}
