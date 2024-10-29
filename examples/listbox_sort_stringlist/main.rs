// Variation on ListBox model example to show use of StringList and StringSorter.

use gtk::{
    glib::{self, clone},
    prelude::*,
};

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.SortedStringList")
        .build();

    application.connect_activate(build_ui);

    application.run()
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .default_width(320)
        .default_height(480)
        .application(application)
        .title("Sorted StringList")
        .build();

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);

    // Create a StringSorter with a property expression to sort
    // StringObjects in a StringList.  StringObject has a "string" property.
    let expression = gtk::PropertyExpression::new(
        gtk::StringObject::static_type(),
        None::<gtk::Expression>,
        "string",
    );
    let sorter = gtk::StringSorter::new(Some(expression));
    sorter.set_ignore_case(true);

    // Create our list store as a StringList and populate with some strings.
    let model = gtk::StringList::new(&["zoo", "abba", "donkey", "sunrise", "river", "phoenix"]);

    // Create a sort model and bind it to the ListStore and the sorter.
    let sort_model = gtk::SortListModel::new(Some(model.clone()), Some(sorter));

    // And then create the UI part, the listbox and bind the sort
    // model to it. Whenever the UI needs to show a new row, e.g. because
    // it was notified that the model changed, it will call the callback
    // with the corresponding item from the model and will ask for a new
    // gtk::ListBoxRow that should be displayed.
    //
    // The gtk::ListBoxRow can contain any possible widgets.
    //  Here we use an Inscription.

    let listbox = gtk::ListBox::new();
    listbox.bind_model(
        Some(&sort_model),
        clone!(
            #[weak(rename_to = _panel)]
            window,
            #[upgrade_or_panic]
            move |obj| {
                let list_object = obj
                    .downcast_ref::<gtk::StringObject>()
                    .expect("The object should be of type `StringObject`.");
                let row = gtk::Inscription::new(Some(&list_object.string()));
                row.set_xalign(0.0);
                row.upcast()
            }
        ),
    );

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_height(480)
        .min_content_width(360)
        .build();

    scrolled_window.set_child(Some(&listbox));

    vbox.append(&scrolled_window);

    window.set_child(Some(&vbox));

    for i in 0..10 {
        model.append(&format!("Name {i}"));
    }

    window.present();
}
