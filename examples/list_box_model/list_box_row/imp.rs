use std::cell::RefCell;

use gtk::{
    glib::{self, clone, ParamSpec, ParamSpecObject, Value},
    prelude::*,
    subclass::prelude::*,
    ResponseType,
};

use crate::row_data::RowData;

#[derive(Default, Debug)]
pub struct ListBoxRow {
    row_data: RefCell<Option<RowData>>,
}

#[glib::object_subclass]
impl ObjectSubclass for ListBoxRow {
    const NAME: &'static str = "ExListBoxRow";
    type ParentType = gtk::ListBoxRow;
    type Type = super::ListBoxRow;
}

impl ObjectImpl for ListBoxRow {
    fn properties() -> &'static [ParamSpec] {
        use once_cell::sync::Lazy;
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecObject::builder::<RowData>("row-data")
                .construct_only()
                .build()]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "row-data" => {
                let row_data = value.get().unwrap();
                self.row_data.replace(row_data);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "row-data" => self.row_data.borrow().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self) {
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
        let label = gtk::Label::new(None);
        item.bind_property("name", &label, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        hbox.append(&label);
        let spin_button = gtk::SpinButton::with_range(0.0, 100.0, 1.0);
        item.bind_property("count", &spin_button, "value")
            .flags(
                glib::BindingFlags::DEFAULT
                    | glib::BindingFlags::SYNC_CREATE
                    | glib::BindingFlags::BIDIRECTIONAL,
            )
            .build();
        hbox.append(&spin_button);

        // When the edit button is clicked, a new modal dialog is created for editing
        // the corresponding row
        let edit_button = gtk::Button::with_label("Edit");
        edit_button.connect_clicked(clone!(@weak item, @weak obj => move |_| {
            let parent_window = obj.root().map(|root| root.downcast::<gtk::Window>().unwrap());
            let dialog = gtk::Dialog::with_buttons(
                Some("Edit Item"),
                parent_window.as_ref(),
                gtk::DialogFlags::MODAL,
                &[("Close", ResponseType::Close)],
            );
            dialog.set_default_response(ResponseType::Close);
            dialog.connect_response(|dialog, _| dialog.close());

            let content_area = dialog.content_area();

            // Similarly to the label and spin button inside the listbox, the text entry
            // and spin button in the edit dialog are connected via property bindings to
            // the item. Any changes will be immediately reflected inside the item and
            // by the listbox
            let entry = gtk::Entry::new();
            item.bind_property("name", &entry, "text")
                .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
                .build();

            // Activating the entry (enter) will send response `ResponseType::Close` to the dialog
            entry.connect_activate(clone!(@weak dialog => move |_| {
                dialog.response(ResponseType::Close);
            }));
            content_area.append(&entry);

            let spin_button = gtk::SpinButton::with_range(0.0, 100.0, 1.0);
            item.bind_property("count", &spin_button, "value")
                .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
                .build();
            content_area.append(&spin_button);

            dialog.show()
        }));
        hbox.append(&edit_button);

        obj.set_child(Some(&hbox));

        // When a row is activated (select + enter) we simply emit the clicked
        // signal on the corresponding edit button to open the edit dialog
        obj.connect_activate(clone!(@weak edit_button => move |_| {
            edit_button.emit_clicked();
        }));
    }
}

impl WidgetImpl for ListBoxRow {}
impl ListBoxRowImpl for ListBoxRow {}
