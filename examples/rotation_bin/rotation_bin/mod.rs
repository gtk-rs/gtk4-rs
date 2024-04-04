mod imp;

use gtk::{glib, prelude::*, subclass::prelude::*};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, glib::Enum)]
#[repr(i32)]
#[enum_type(name = "Rotation")]
pub enum Rotation {
    #[default]
    Normal = 0,
    Deg90 = 90,
    Deg180 = 180,
    Deg270 = 270,
}

impl Rotation {
    pub fn rotate_clockwise(self) -> Self {
        match self {
            Self::Normal => Self::Deg90,
            Self::Deg90 => Self::Deg180,
            Self::Deg180 => Self::Deg270,
            Self::Deg270 => Self::Normal,
        }
    }

    pub fn rotate_counter_clockwise(self) -> Self {
        match self {
            Self::Normal => Self::Deg270,
            Self::Deg90 => Self::Normal,
            Self::Deg180 => Self::Deg90,
            Self::Deg270 => Self::Deg180,
        }
    }
}

impl From<Rotation> for f32 {
    fn from(item: Rotation) -> f32 {
        match item {
            Rotation::Normal => 0.0,
            Rotation::Deg90 => 90.0,
            Rotation::Deg180 => 180.0,
            Rotation::Deg270 => 270.0,
        }
    }
}

glib::wrapper! {
    pub struct RotationBin(ObjectSubclass<imp::RotationBin>)
        @extends gtk::Widget;
}

impl Default for RotationBin {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl RotationBin {
    pub(super) fn rotate_clockwise(&self) {
        let r = self.rotation();
        self.set_rotation(r.rotate_clockwise());
        self.notify("rotation");
    }

    pub(super) fn rotate_counter_clockwise(&self) {
        let r = self.rotation();
        self.set_rotation(r.rotate_counter_clockwise());
        self.notify("rotation");
    }

    pub(super) fn set_rotation(&self, rotation: Rotation) {
        if self.imp().rotation.get() != rotation {
            self.imp().rotation.replace(rotation);
            self.queue_resize();
            self.notify("rotation");
        }
    }

    pub(super) fn set_child(&self, widget: Option<&impl IsA<gtk::Widget>>) {
        let imp = self.imp();
        let widget = widget.map(|w| w.upcast_ref());
        if widget == imp.child.borrow().as_ref() {
            return;
        }

        if let Some(child) = imp.child.borrow_mut().take() {
            child.unparent();
        }

        if let Some(w) = widget {
            imp.child.replace(Some(w.clone()));
            w.set_parent(self);
        }

        self.queue_resize();
        self.notify("child")
    }
}
