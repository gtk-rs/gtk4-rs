mod grid_cell;

use crate::grid_cell::Entry;
use crate::grid_cell::GridCell;
use gtk::gio;
use gtk::glib::BoxedAnyObject;
use gtk::prelude::*;

struct Row {
    col1: String,
    col2: String,
}

use std::cell::Ref;

fn main() {
    let app = gtk::Application::new(
        Some("com.github.gtk-rs.examples.columnview-example"),
        Default::default(),
    );
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .default_width(320)
        .default_height(480)
        .application(application)
        .title("ColumnView Example")
        .build();

    let store = gio::ListStore::new(BoxedAnyObject::static_type());

    (0..10000).for_each(|i| {
        store.append(&BoxedAnyObject::new(Row {
            col1: format!("col1 {}", i),
            col2: format!("col2 {}", i),
        }))
    });
    let sel = gtk::SingleSelection::new(Some(&store));
    let columnview = gtk::ColumnView::new(Some(&sel));

    let col1factory = gtk::SignalListItemFactory::new();
    let col2factory = gtk::SignalListItemFactory::new();
    let col1 = gtk::ColumnViewColumn::new(Some("Column 1"), Some(&col1factory));
    let col2 = gtk::ColumnViewColumn::new(Some("Column 2"), Some(&col2factory));
    col1factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let row = GridCell::new();
        item.set_child(Some(&row));
    });

    col1factory.connect_bind(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let child = item.child().and_downcast::<GridCell>().unwrap();
        let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();
        let r: Ref<Row> = entry.borrow();
        let ent = Entry {
            name: r.col1.to_string(),
        };
        child.set_entry(&ent);
    });
    col2factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let row = GridCell::new();
        item.set_child(Some(&row));
    });

    col2factory.connect_bind(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let child = item.child().and_downcast::<GridCell>().unwrap();
        let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();
        let r: Ref<Row> = entry.borrow();
        let ent = Entry {
            name: r.col2.to_string(),
        };
        child.set_entry(&ent);
    });
    columnview.append_column(&col1);
    columnview.append_column(&col2);

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .build();

    scrolled_window.set_child(Some(&columnview));

    window.set_child(Some(&scrolled_window));
    window.show();
}
