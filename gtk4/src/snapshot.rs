// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Snapshot;
use glib::translate::*;
use graphene::{Point, Rect};
use gsk::ColorStop;

impl Snapshot {
    #[doc(alias = "gtk_snapshot_append_linear_gradient")]
    pub fn append_linear_gradient(
        &self,
        bounds: &Rect,
        start_point: &Point,
        end_point: &Point,
        stops: &[ColorStop],
    ) {
        let n_stops = stops.len() as usize;
        unsafe {
            ffi::gtk_snapshot_append_linear_gradient(
                self.to_glib_none().0,
                bounds.to_glib_none().0,
                start_point.to_glib_none().0,
                end_point.to_glib_none().0,
                stops.to_glib_none().0,
                n_stops,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_radial_gradient")]
    pub fn append_radial_gradient(
        &self,
        bounds: &graphene::Rect,
        center: &graphene::Point,
        hradius: f32,
        vradius: f32,
        start: f32,
        end: f32,
        stops: &[gsk::ColorStop],
    ) {
        let n_stops = stops.len() as usize;
        unsafe {
            ffi::gtk_snapshot_append_radial_gradient(
                self.to_glib_none().0,
                bounds.to_glib_none().0,
                center.to_glib_none().0,
                hradius,
                vradius,
                start,
                end,
                stops.to_glib_none().0,
                n_stops,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_conic_gradient")]
    pub fn append_conic_gradient(
        &self,
        bounds: &graphene::Rect,
        center: &graphene::Point,
        rotation: f32,
        stops: &[gsk::ColorStop],
    ) {
        let n_stops = stops.len() as usize;
        unsafe {
            ffi::gtk_snapshot_append_conic_gradient(
                self.to_glib_none().0,
                bounds.to_glib_none().0,
                center.to_glib_none().0,
                rotation,
                stops.to_glib_none().0,
                n_stops,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_repeating_radial_gradient")]
    pub fn append_repeating_radial_gradient(
        &self,
        bounds: &graphene::Rect,
        center: &graphene::Point,
        hradius: f32,
        vradius: f32,
        start: f32,
        end: f32,
        stops: &[gsk::ColorStop],
    ) {
        let n_stops = stops.len() as usize;
        unsafe {
            ffi::gtk_snapshot_append_repeating_radial_gradient(
                self.to_glib_none().0,
                bounds.to_glib_none().0,
                center.to_glib_none().0,
                hradius,
                vradius,
                start,
                end,
                stops.to_glib_none().0,
                n_stops,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_repeating_linear_gradient")]
    pub fn append_repeating_linear_gradient(
        &self,
        bounds: &Rect,
        start_point: &Point,
        end_point: &Point,
        stops: &[ColorStop],
    ) {
        let n_stops = stops.len() as usize;
        unsafe {
            ffi::gtk_snapshot_append_repeating_linear_gradient(
                self.to_glib_none().0,
                bounds.to_glib_none().0,
                start_point.to_glib_none().0,
                end_point.to_glib_none().0,
                stops.to_glib_none().0,
                n_stops,
            );
        }
    }
}
