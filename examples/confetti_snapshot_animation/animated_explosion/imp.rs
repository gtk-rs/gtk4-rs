use std::cell::{Cell, RefCell};

use gtk::{glib, subclass::prelude::*};

use crate::Explosion;

#[derive(Default)]
pub struct AnimatedExplosion {
    pub(super) explosion: RefCell<Option<Explosion>>,
    pub(super) start: Cell<f64>,
    pub(super) lastupdate: Cell<f64>,
    pub(super) duration: Cell<f64>,
    pub(super) finished: Cell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for AnimatedExplosion {
    const NAME: &'static str = "AnimatedExplosion";
    type Type = super::AnimatedExplosion;
}

impl ObjectImpl for AnimatedExplosion {}
