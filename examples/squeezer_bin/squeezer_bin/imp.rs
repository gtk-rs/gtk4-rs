use gtk::glib;
use gtk::glib::{ParamSpec, Value};
use gtk::gsk;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use once_cell::sync::Lazy;
use std::cell::{Cell, RefCell};

fn child_size(child: &impl IsA<gtk::Widget>) -> ((i32, i32), (i32, i32)) {
    let (horizontal_minimal, horizontal_natural, _, _) =
        child.measure(gtk::Orientation::Horizontal, -1);
    let (vertical_minimal, vertical_natural, _, _) = child.measure(gtk::Orientation::Vertical, -1);

    (
        (horizontal_minimal, horizontal_natural),
        (vertical_minimal, vertical_natural),
    )
}

#[derive(Debug, Default)]
pub struct SqueezerBin {
    child: RefCell<Option<gtk::Widget>>,
    keep_aspect_ratio: Cell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for SqueezerBin {
    const NAME: &'static str = "SqueezerBin";
    type ParentType = gtk::Widget;
    type Type = super::SqueezerBin;
}

impl ObjectImpl for SqueezerBin {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
            vec![
                glib::ParamSpecObject::builder::<gtk::Widget>("child")
                    .explicit_notify()
                    .build(),
                glib::ParamSpecBoolean::builder("keep-aspect-ratio")
                    .explicit_notify()
                    .build(),
            ]
        });

        PROPERTIES.as_ref()
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        let obj = self.instance();
        match pspec.name() {
            "child" => self.child(&obj).to_value(),
            "keep-aspect-ratio" => self.keep_aspect_ratio(&obj).to_value(),
            _ => unimplemented!(),
        }
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        let obj = self.instance();
        match pspec.name() {
            "child" => {
                self.set_child(&obj, value.get::<gtk::Widget>().ok().as_ref());
            }
            "keep-aspect-ratio" => {
                self.set_keep_aspect_ratio(&obj, value.get().unwrap());
            }
            _ => unimplemented!(),
        }
    }

    fn constructed(&self) {
        let obj = self.instance();

        obj.set_halign(gtk::Align::Fill);
        obj.set_valign(gtk::Align::Fill);
        obj.set_hexpand(true);
        obj.set_vexpand(true);

        self.parent_constructed();
    }

    fn dispose(&self) {
        if let Some(child) = self.instance().child() {
            child.unparent();
        }
    }
}

impl WidgetImpl for SqueezerBin {
    fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        let widget = self.instance();
        if let Some(child) = widget.child() {
            let ((_, horizontal_size), (_, vertical_size)) = child_size(&*widget);

            let (mut horizontal_zoom, mut vertical_zoom) = (
                width as f32 / horizontal_size as f32,
                height as f32 / vertical_size as f32,
            );

            if widget.keep_aspect_ratio() {
                if horizontal_zoom < vertical_zoom {
                    vertical_zoom = horizontal_zoom;
                } else {
                    horizontal_zoom = vertical_zoom;
                }
            }

            let transform = gsk::Transform::new().scale(horizontal_zoom, vertical_zoom);

            child.allocate(
                (width as f32 / horizontal_zoom) as i32,
                (height as f32 / vertical_zoom) as i32,
                baseline,
                Some(&transform),
            );
        }
    }
}

impl SqueezerBin {
    pub(super) fn child(&self, _obj: &super::SqueezerBin) -> Option<gtk::Widget> {
        self.child.borrow().clone()
    }

    pub(super) fn set_child(
        &self,
        obj: &super::SqueezerBin,
        widget: Option<&impl IsA<gtk::Widget>>,
    ) {
        let widget = widget.map(|w| w.as_ref());
        if widget == self.child.borrow().as_ref() {
            return;
        }

        if let Some(child) = self.child.borrow_mut().take() {
            child.unparent();
        }

        if let Some(w) = widget {
            self.child.replace(Some(w.clone()));
            w.set_parent(obj);
        }

        obj.queue_resize();
        obj.notify("child")
    }

    pub(super) fn keep_aspect_ratio(&self, _obj: &super::SqueezerBin) -> bool {
        self.keep_aspect_ratio.get()
    }

    pub(super) fn set_keep_aspect_ratio(&self, obj: &super::SqueezerBin, keep_aspect_ratio: bool) {
        if self.keep_aspect_ratio.get() == keep_aspect_ratio {
            return;
        }

        self.keep_aspect_ratio.set(keep_aspect_ratio);

        obj.queue_resize();
        obj.notify("keep-aspect-ratio")
    }
}
