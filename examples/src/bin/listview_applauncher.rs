use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

fn setup_listitem_cb(_: &gtk::SignalListItemFactory, list_item: &gtk::ListItem) {
    let box_ = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    let image = gtk::Image::new();
    image.set_icon_size(gtk::IconSize::Large);
    box_.append(&image);
    let label = gtk::Label::new(Some(""));
    box_.append(&label);
    list_item.set_child(Some(&box_));
}

fn bind_listitem_cb(_: &gtk::SignalListItemFactory, list_item: &gtk::ListItem) {
    let child = list_item.get_child().unwrap();
    let image = child.get_first_child().unwrap();
    let label = image.get_next_sibling().unwrap();
    let item = list_item.get_item().unwrap();

    let app_info: gio::AppInfo = item.downcast().unwrap();

    if let Some(icon) = app_info.get_icon() {
        image
            .downcast::<gtk::Image>()
            .unwrap()
            .set_from_gicon(&icon);
        let name = gio::AppInfo::get_display_name(&app_info).unwrap();
        label.downcast::<gtk::Label>().unwrap().set_label(&name);
    }
}

fn create_application_list() -> gio::ListStore {
    let store = gio::ListStore::new(gio::AppInfo::static_type());
    let apps = gio::AppInfo::get_all();
    for l in apps {
        store.append(&l)
    }

    store
}

fn activate_cb(list: &gtk::ListView, position: u32) {
    let model = list.get_model().unwrap();
    let l_model: gio::ListModel = model.upcast();
    let info = l_model.get_object(position).unwrap();

    let app_info = info.downcast::<gio::AppInfo>().unwrap();

    let display = list.get_display();
    let context = gdk::Display::get_app_launch_context(&display);
    if !app_info.launch(&[], Some(&context)).is_ok() {
        let root = list.get_root().unwrap();
        let window: gtk::Window = root.downcast().unwrap();

        let dialog = gtk::MessageDialog::new(
            Some(&window),
            gtk::DialogFlags::empty(),
            gtk::MessageType::Info,
            gtk::ButtonsType::Ok,
            &format!(
                "Could not launch {}",
                app_info.get_display_name().unwrap().as_str()
            ),
        );

        dialog.connect_response(|dialog, _| {
            dialog.close();
        });

        dialog.show();
    }
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_default_size(300, 200);

    let factory = gtk::SignalListItemFactory::new();
    factory.connect_setup(setup_listitem_cb);
    factory.connect_bind(bind_listitem_cb);

    let model = create_application_list();

    let selection_model = gtk::SingleSelection::new(Some(&model));
    let list = gtk::ListView::new(Some(&selection_model), Some(&factory));

    list.connect_activate(activate_cb);

    let sw = gtk::ScrolledWindow::new();
    window.set_child(Some(&sw));
    sw.set_child(Some(&list));

    window.set_application(Some(application));
    application.add_window(&window);
    window.present();
}

pub fn main() {
    let application = gtk::Application::new(
        Some("org.gtk-rs.examples.listview_applauncher"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
