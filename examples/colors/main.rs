use std::cell::RefCell;
use std::rc::Rc;

use gtk::gdk;
use gtk::glib;
use gtk::glib::clone;
use gtk::glib::closure;
use gtk::{pango, prelude::*};

mod color;
mod store;

use color::Color;
use store::ColorStore;

const SORTERS: [&str; 7] = [
    "Unsorted",
    "Hue",
    "Saturation",
    "Value",
    "Red",
    "Green",
    "Blue",
];

const SIZES: [&str; 8] = [
    "8", "64", "512", "4096", "32768", "262144", "2097152", "16777216",
];

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.colors"),
        Default::default(),
    );

    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .default_width(600)
        .default_height(400)
        .application(app)
        .title("Colors")
        .build();

    let header = gtk::HeaderBar::new();
    window.set_titlebar(Some(&header));

    let model = ColorStore::new(8);

    let sort_model = gtk::SortListModel::new(Some(&model), gtk::Sorter::NONE);
    sort_model.set_incremental(true);
    let selection = gtk::MultiSelection::new(Some(&sort_model));

    let selected = Rc::new(RefCell::new(4096u32));

    let refill_btn = gtk::Button::with_mnemonic("_Refill");
    refill_btn.connect_clicked(clone!(@strong model, @strong selected => move |_btn| {
        model.set_property("size", 0u32);
        _btn.add_tick_callback(clone!(@strong model => move |_, _| {
            let limit = model.limit();
            model.set_property("size", limit.min(model.size() + 1.max(limit / 4096)));
            Continue(model.size() < limit)
        }));
    }));
    header.pack_start(&refill_btn);

    let progress = gtk::ProgressBar::builder()
        .hexpand(true)
        .valign(gtk::Align::Start)
        .visible(false)
        .build();

    sort_model.connect_pending_notify(clone!(@strong progress => move |model| {
        let total = model.n_items();
        let total = total.max(1); /* avoid div by 0 below */
        let pending = model.pending();

        progress.set_visible(pending != 0);
        progress.set_fraction((total - pending) as f64 / total as f64);
    }));

    let attrs = pango::AttrList::new();
    attrs.insert(pango::AttrFontFeatures::new("tnum"));

    let label = gtk::Label::builder()
        .label("0 /")
        .attributes(&attrs)
        .xalign(1.0)
        .build();

    header.pack_start(&label);

    let dropdown = gtk::DropDown::from_strings(&SIZES);
    dropdown.connect_selected_notify(clone!(@strong selected, @strong label => move |dropdown| {
        let num_str = SIZES[dropdown.selected() as usize];
        if let Ok(num) = num_str.parse::<u32>() {
            model.set_property("size", num);
            model.set_property("limit", num);

            label.set_label(&format!("{} /", num_str));

            *selected.borrow_mut() = num;
        }
    }));
    dropdown.set_selected(3);
    header.pack_start(&dropdown);

    let dropdown = gtk::DropDown::from_strings(&SORTERS);
    dropdown.connect_selected_notify(clone!(@strong sort_model => move |a| {
        let sorter = match SORTERS[a.selected() as usize].to_lowercase().as_str() {
            "unsorted" => None,
            name => {
                let sorter = gtk::NumericSorter::new(Some(&gtk::PropertyExpression::new(Color::static_type(), None::<&gtk::Expression>, name)));
                sorter.set_sort_order(gtk::SortType::Descending);
                Some(sorter)
            },
        };
        sort_model.set_sorter(sorter.as_ref());
    }));

    header.pack_end(&dropdown);
    header.pack_end(&gtk::Label::new(Some("Sort By: ")));

    let dropdown = gtk::DropDown::from_strings(&["Colors", "Everything"]);
    header.pack_end(&dropdown);

    let attrs = pango::AttrList::new();
    attrs.insert(pango::AttrFontFeatures::new("tnum"));

    let label = gtk::Label::builder()
        .label("Show: ")
        .attributes(&attrs)
        .xalign(1.0)
        .build();
    header.pack_end(&label);

    let grid_view = gtk::GridView::builder()
        .model(&selection)
        .hscroll_policy(gtk::ScrollablePolicy::Natural)
        .vscroll_policy(gtk::ScrollablePolicy::Natural)
        .max_columns(24)
        .enable_rubberband(true)
        .build();

    dropdown.connect_selected_item_notify(clone!(@strong grid_view => move |dropdown| {
        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(clone!(@strong dropdown => move |_, list_item| {
            let expression = gtk::ConstantExpression::new(list_item);
            let color_expression = gtk::PropertyExpression::new(
                gtk::ListItem::static_type(),
                Some(&expression),
                "item",
            );

            let child = match dropdown.selected_item().and_then(|i| i.downcast_ref::<gtk::StringObject>().map(|o| o.string())).as_deref() {
                Some("Colors") => {
                    let picture = gtk::Picture::new();
                    color_expression.bind(&picture, "paintable", None::<&glib::Object>);
                    picture.set_size_request(32, 32);

                    Some(picture.upcast::<gtk::Widget>())
                },

                Some("Everything") => {
                    let expression2 = gtk::PropertyExpression::new(
                        Color::static_type(),
                        Some(&color_expression),
                        "color",
                    ).chain_closure::<String>(closure!(|_: Option<glib::Object>, color: &gdk::RGBA| {
                        format!("<b>R:</b> {} <b>G:</b> {} <b>B:</b> {}",
                        (color.red() * 255.),
                        (color.green() * 255.),
                        (color.blue() * 255.))
                    }));

                    let boxx = gtk::Box::new(gtk::Orientation::Vertical, 0);

                    let picture = gtk::Picture::new();
                    color_expression.bind(&picture, "paintable", None::<&glib::Object>);
                    boxx.append(&picture);

                    let rgb_label = gtk::Label::new(None);
                    rgb_label.set_use_markup(true);
                    expression2.bind(&rgb_label, "label", Some(&rgb_label));
                    boxx.append(&rgb_label);

                    Some(boxx.upcast::<gtk::Widget>())
                }

                _ => None
            };

            list_item.set_child(child.as_ref());
        }));

        grid_view.set_factory(Some(&factory));
    }));

    dropdown.set_selected(0);
    dropdown.notify("selected-item");

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&grid_view)
        .build();

    let overlay = gtk::Overlay::new();
    overlay.add_overlay(&scrolled_window);
    overlay.add_overlay(&progress);

    window.set_child(Some(&overlay));
    window.show();
}
