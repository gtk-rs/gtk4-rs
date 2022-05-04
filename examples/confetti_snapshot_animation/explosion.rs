// Contains the needed data to run a simple particle simulation

use glib::random_double;
use graphene::Vec2;
use gtk::{gdk, glib, graphene};
use std::f32::consts::PI;

pub struct ExplosionParameters {
    pub quantity: usize,
    pub velocity: f32,
    pub velocity_randomness: f32,
    pub shoot_angle: f32,
    pub shoot_angle_randomness: f32,
    pub acceleration: Vec2,
    pub origin: Vec2,
    pub turbolence: f32,
    pub spread: f32,
    pub damp: f32,
}

pub(super) struct Explosion {
    pub(super) velocity: Vec<Vec2>,
    pub(super) acceleration: Vec<Vec2>,
    pub(super) position: Vec<Vec2>,
    pub(super) turbolence: f32,
    pub(super) damp: f32,
    pub(super) color: Vec<gdk::RGBA>,
}

impl Explosion {
    pub fn new(parameters: ExplosionParameters) -> Self {
        let ExplosionParameters {
            quantity,
            origin,
            velocity,
            velocity_randomness,
            shoot_angle,
            shoot_angle_randomness,
            acceleration,
            turbolence,
            spread,
            damp,
        } = parameters;
        Self {
            velocity: (0..quantity)
                .map(|_| {
                    let angle =
                        shoot_angle + (random_double() as f32 - 0.5) * PI * shoot_angle_randomness;
                    let module = (1.0 - random_double() as f32 * velocity_randomness) * velocity;
                    Vec2::new(angle.cos(), angle.sin()).scale(module)
                })
                .collect(),
            acceleration: (0..quantity).map(|_| acceleration).collect(),
            position: (0..quantity)
                .map(|_| {
                    origin.add(
                        &Vec2::new(random_double() as f32 - 0.5, random_double() as f32 - 0.5)
                            .scale(spread),
                    )
                })
                .collect(),
            damp,
            color: {
                (0..quantity)
                    .map(|i| {
                        let hue = 1.0 / quantity as f32 * i as f32;
                        let (r, g, b) = gtk::hsv_to_rgb(hue, 1.0, 1.0);
                        gdk::RGBA::new(r, g, b, 1.0)
                    })
                    .collect()
            },
            turbolence,
        }
    }
    pub fn update(&mut self, dt: f32) {
        for i in 0..self.velocity.len() {
            let tx = (glib::random_double() as f32 - 0.5) * self.turbolence;
            let ty = (glib::random_double() as f32 - 0.5) * self.turbolence;
            let turbolence = Vec2::new(tx, ty).scale(dt);
            let res = self.velocity[i]
                .add(&self.acceleration[i].scale(dt))
                .add(&turbolence);
            self.velocity[i] = res;
        }
        for i in 0..self.velocity.len() {
            let vel = self.velocity[i];
            let dl = vel.scale(dt);
            let res = self.position[i].add(&dl);
            self.position[i] = res;
            let slowing_factor = (vel.length().powf(2.0) - 2.0 * dl.length() * self.damp)
                .clamp(0.0, f32::INFINITY)
                .sqrt()
                / vel.length();
            self.velocity[i] = vel.scale(slowing_factor);
        }
    }
}
