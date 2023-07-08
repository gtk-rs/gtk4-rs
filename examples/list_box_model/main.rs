mod list_box_row;
mod model;
pub mod row_data;

use gtk::{
    glib::{self, clone},
    prelude::*,
    ResponseType,
};
use list_box_row::ListBoxRow;
use row_data::RowData;

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.listbox-model")
        .build();

    application.connect_activate(build_ui);

    application.run()
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .default_width(320)
        .default_height(480)
        .application(application)
        .title("Custom Model")
        .build();

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);

    // Create our list store and specify that the type stored in the
    // list should be the RowData GObject we define at the bottom
    let model = model::Model::new();

    // And then create the UI part, the listbox and bind the list store
    // model to it. Whenever the UI needs to show a new row, e.g. because
    // it was notified that the model changed, it will call the callback
    // with the corresponding item from the model and will ask for a new
    // gtk::ListBoxRow that should be displayed.
    //
    // The gtk::ListBoxRow can contain any possible widgets.

    let listbox = gtk::ListBox::new();
    listbox.bind_model(
        Some(&model),
        clone!(@weak window => @default-panic, move |item| {
            ListBoxRow::new(
                item.downcast_ref::<RowData>()
                    .expect("RowData is of wrong type"),
            )
            .upcast::<gtk::Widget>()
        }),
    );

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_height(480)
        .min_content_width(360)
        .build();

    scrolled_window.set_child(Some(&listbox));

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    // The add button opens a new dialog which is basically the same as the edit
    // dialog, except that we don't have a corresponding item yet at that point
    // and only create it once the Ok button in the dialog is clicked, and only
    // then add it to the model. Once added to the model, it will immediately
    // appear in the listbox UI
    let add_button = gtk::Button::with_label("Add");
    add_button.connect_clicked(clone!(@weak window, @weak model => move |_| {
        let dialog = gtk::Dialog::with_buttons(
            Some("Add Item"),
            Some(&window),
            gtk::DialogFlags::MODAL,
            &[("Ok", ResponseType::Ok), ("Cancel", ResponseType::Cancel)],
        );
        dialog.set_default_response(ResponseType::Ok);
        let content_area = dialog.content_area();
        let entry = gtk::Entry::new();
        entry.connect_activate(clone!(@weak dialog => move |_| {
            dialog.response(ResponseType::Ok);
        }));
        content_area.append(&entry);
        let spin_button = gtk::SpinButton::with_range(0.0, 100.0, 1.0);
        content_area.append(&spin_button);
        dialog.connect_response(clone!(@weak model, @weak entry, @weak spin_button => move |dialog, resp| {
            let text = entry.text();
            if !text.is_empty() && resp == ResponseType::Ok {
                model.append(&RowData::new(&text, spin_button.value() as u32));
            }
            dialog.close();
        }));

        dialog.show()
    }));

    hbox.append(&add_button);

    // Via the delete button we delete the item from the model that
    // is at the index of the selected row. Also deleting from the
    // model is immediately reflected in the listbox.
    let delete_button = gtk::Button::with_label("Delete");
    delete_button.connect_clicked(clone!(@weak model, @weak listbox => move |_| {
        let selected = listbox.selected_row();

        if let Some(selected) = selected {
            let idx = selected.index();
            model.remove(idx as u32);
        }
    }));
    hbox.append(&delete_button);

    vbox.append(&hbox);
    vbox.append(&scrolled_window);

    window.set_child(Some(&vbox));

    for i in 0..10 {
        model.append(&RowData::new(&format!("Name {i}"), i * 10));
    }

    window.present();
}
