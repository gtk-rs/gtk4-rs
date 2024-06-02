use std::cell::{Cell, RefCell};

use gtk::{glib, gsk, prelude::*, subclass::prelude::*};

fn child_size(child: &impl IsA<gtk::Widget>) -> ((i32, i32), (i32, i32)) {
    let (horizontal_minimal, horizontal_natural, _, _) =
        child.measure(gtk::Orientation::Horizontal, -1);
    let (vertical_minimal, vertical_natural, _, _) = child.measure(gtk::Orientation::Vertical, -1);

    (
        (horizontal_minimal, horizontal_natural),
        (vertical_minimal, vertical_natural),
    )
}

#[derive(Debug, Default, glib::Properties)]
#[properties(wrapper_type = super::SqueezerBin)]
pub struct SqueezerBin {
    #[property(get, set = Self::set_child, explicit_notify, nullable)]
    pub(super) child: RefCell<Option<gtk::Widget>>,
    #[property(get, set = Self::set_keep_aspect_ratio, explicit_notify)]
    pub(super) keep_aspect_ratio: Cell<bool>,
}

impl SqueezerBin {
    fn set_child(&self, widget: Option<&gtk::Widget>) {
        if widget == self.child.borrow().as_ref() {
            return;
        }

        if let Some(child) = self.child.borrow_mut().take() {
            child.unparent();
        }

        if let Some(w) = widget {
            self.child.replace(Some(w.clone()));
            w.set_parent(&*self.obj());
        }

        self.obj().queue_resize();
        self.obj().notify_child();
    }

    fn set_keep_aspect_ratio(&self, keep_aspect_ratio: bool) {
        if self.keep_aspect_ratio.get() == keep_aspect_ratio {
            return;
        }

        self.keep_aspect_ratio.set(keep_aspect_ratio);

        self.obj().queue_resize();
        self.obj().notify_keep_aspect_ratio();
    }
}

#[glib::object_subclass]
impl ObjectSubclass for SqueezerBin {
    const NAME: &'static str = "SqueezerBin";
    type ParentType = gtk::Widget;
    type Type = super::SqueezerBin;
}

#[glib::derived_properties]
impl ObjectImpl for SqueezerBin {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();

        obj.set_halign(gtk::Align::Fill);
        obj.set_valign(gtk::Align::Fill);
        obj.set_hexpand(true);
        obj.set_vexpand(true);
    }

    fn dispose(&self) {
        if let Some(child) = self.obj().child() {
            child.unparent();
        }
    }
}

impl WidgetImpl for SqueezerBin {
    fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        let widget = self.obj();
        if let Some(child) = widget.child() {
            let ((_, horizontal_size), (_, vertical_size)) = child_size(&child);

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
                Some(transform),
            );
        }
    }
}
