// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use glib::translate::*;
use graphene::{Point, Rect};
use gsk::ColorStop;
use gtk_sys;
use Snapshot;

impl Snapshot {
    pub fn append_linear_gradient(
        &self,
        bounds: &Rect,
        start_point: &Point,
        end_point: &Point,
        stops: &[ColorStop],
    ) {
        let n_stops = stops.len() as usize;
        unsafe {
            gtk_sys::gtk_snapshot_append_linear_gradient(
                self.to_glib_none().0,
                bounds.to_glib_none().0,
                start_point.to_glib_none().0,
                end_point.to_glib_none().0,
                stops.to_glib_none().0,
                n_stops,
            );
        }
    }

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
            gtk_sys::gtk_snapshot_append_radial_gradient(
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
            gtk_sys::gtk_snapshot_append_repeating_radial_gradient(
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

    pub fn append_repeating_linear_gradient(
        &self,
        bounds: &Rect,
        start_point: &Point,
        end_point: &Point,
        stops: &[ColorStop],
    ) {
        let n_stops = stops.len() as usize;
        unsafe {
            gtk_sys::gtk_snapshot_append_repeating_linear_gradient(
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
