mod metadata;
mod note;

use glib::closure;

use gtk::gio;
use gtk::glib;
use gtk::prelude::*;

use self::metadata::Metadata;
use self::note::Note;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.expressions"),
        Default::default(),
    );

    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(app: &gtk::Application) {
    let factory = gtk::SignalListItemFactory::new();
    factory.connect_setup(|_, item| {
        // In gtk4 < 4.8, you don't need the following line
        // as gtk used to pass GtkListItem directly. In order to make that API
        // generic for potentially future new APIs, it was switched to taking a GObject in 4.8
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let hbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(24)
            .build();

        let title_label = gtk::Label::new(None);
        hbox.append(&title_label);

        let last_modified_label = gtk::Label::new(None);
        hbox.append(&last_modified_label);

        // Instead of binding properties and manually unbinding them, you can create expressions.
        // The value can be obtained even if it is several steps away.
        item.property_expression("item")
            .chain_property::<Note>("metadata")
            .chain_property::<Metadata>("title")
            .chain_closure_with_callback(|args| {
                let title: String = args[1].get().unwrap();
                format!("Title: {title}")
            })
            .bind(&title_label, "label", gtk::Widget::NONE);

        // Property expressions can also start from the `this` value, which is set as the last
        // argument to the `bind` function.
        gtk::ListItem::this_expression("item")
            .chain_property::<Note>("metadata")
            .chain_property::<Metadata>("last-modified")
            .chain_closure::<String>(closure!(
                |_: gtk::ListItem, last_modified: glib::DateTime| {
                    format!("Last Modified: {}", last_modified.format_iso8601().unwrap())
                }
            ))
            .bind(&last_modified_label, "label", Some(item));

        item.set_child(Some(&hbox));
    });

    let model = gtk::NoSelection::new(Some(data()));
    let list_view = gtk::ListView::new(Some(model), Some(factory));

    let scrolled_window = gtk::ScrolledWindow::builder()
        .child(&list_view)
        .hscrollbar_policy(gtk::PolicyType::Never)
        .build();

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Expressions")
        .default_height(500)
        .child(&scrolled_window)
        .build();

    window.show();
}

fn data() -> gio::ListStore {
    let mut vector: Vec<Note> = Vec::new();

    for i in 0..=100 {
        let metadata = Metadata::new();
        metadata.set_property("title", format!("Note ({i})"));

        let note = Note::new(&metadata);
        vector.push(note);
    }

    let list_store = gio::ListStore::new(Note::static_type());
    list_store.splice(0, 0, &vector);
    list_store
}
