mod metadata;
mod note;

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
    factory.connect_setup(|_, list_item| {
        let hbox = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(24)
            .build();

        let title_label = gtk::Label::new(None);
        hbox.append(&title_label);

        let last_modified_label = gtk::Label::new(None);
        hbox.append(&last_modified_label);

        // Instead of binding properties and manually unbinding them, you can create expressions.
        // The value can be obtained even if it is several steps away.
        let metadata_expression = list_item
            .property_expression("item")
            .chain_property::<Note>("metadata");

        metadata_expression
            .chain_property::<Metadata>("title")
            .chain_closure(|args| {
                let title: String = args[1].get().unwrap();
                format!("Title: {}", title)
            })
            .bind(&title_label, "label", gtk::Widget::NONE);

        metadata_expression
            .chain_property::<Metadata>("last-modified")
            .chain_closure(|args| {
                let last_modified: glib::DateTime = args[1].get().unwrap();
                format!("Last Modified: {}", last_modified.format_iso8601().unwrap())
            })
            .bind(&last_modified_label, "label", gtk::Widget::NONE);

        list_item.set_child(Some(&hbox));
    });

    let model = gtk::NoSelection::new(Some(&data()));
    let list_view = gtk::ListView::new(Some(&model), Some(&factory));

    let scrolled_window = gtk::ScrolledWindowBuilder::new()
        .child(&list_view)
        .hscrollbar_policy(gtk::PolicyType::Never)
        .build();

    let window = gtk::ApplicationWindowBuilder::new()
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
        metadata.set_property("title", format!("Note ({})", i));

        let note = Note::new(&metadata);
        vector.push(note);
    }

    let list_store = gio::ListStore::new(Note::static_type());
    list_store.splice(0, 0, &vector);
    list_store
}
