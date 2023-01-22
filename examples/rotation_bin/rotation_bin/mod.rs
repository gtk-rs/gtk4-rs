mod imp;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, glib::Enum)]
#[repr(i32)]
#[enum_type(name = "Rotation")]
pub enum Rotation {
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

impl Default for Rotation {
    fn default() -> Self {
        Rotation::Normal
    }
}

glib::wrapper! {
    pub struct RotationBin(ObjectSubclass<imp::RotationBin>)
        @extends gtk::Widget;
}

impl Default for RotationBin {
    fn default() -> Self {
        Self::new()
    }
}

impl RotationBin {
    pub fn new() -> Self {
        glib::Object::new_default()
    }

    pub fn child(&self) -> Option<impl IsA<gtk::Widget>> {
        self.imp().child(self)
    }

    pub fn rotation(&self) -> Rotation {
        self.imp().rotation(self)
    }

    pub fn set_child(&self, widget: Option<&impl IsA<gtk::Widget>>) {
        self.imp().set_child(self, widget);
    }

    pub fn set_rotation(&self, rotation: Rotation) {
        self.imp().set_rotation(self, rotation);
    }

    pub fn rotate_clockwise(&self) {
        self.imp().rotate_clockwise(self);
    }

    pub fn rotate_counter_clockwise(&self) {
        self.imp().rotate_counter_clockwise(self);
    }
}
