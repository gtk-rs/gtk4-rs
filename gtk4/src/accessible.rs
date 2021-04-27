// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Accessible, AccessibleProperty, AccessibleRelation, AccessibleState};
use glib::translate::*;
use glib::{IsA, ToValue};

pub trait AccessibleExtManual {
    #[doc(alias = "gtk_accessible_update_property")]
    #[doc(alias = "gtk_accessible_update_property_value")]
    fn update_property(&self, properties: &[(AccessibleProperty, &dyn ToValue)]);

    #[doc(alias = "gtk_accessible_update_relation")]
    #[doc(alias = "gtk_accessible_update_relation_value")]
    fn update_relation(&self, relations: &[(AccessibleRelation, &dyn ToValue)]);

    #[doc(alias = "gtk_accessible_update_state")]
    #[doc(alias = "gtk_accessible_update_state_value")]
    fn update_state(&self, states: &[(AccessibleState, &dyn ToValue)]);
}

impl<O: IsA<Accessible>> AccessibleExtManual for O {
    fn update_property(&self, properties: &[(AccessibleProperty, &dyn ToValue)]) {
        unsafe {
            let properties_ptr: Vec<ffi::GtkAccessibleProperty> =
                properties.iter().map(|(k, _)| k.into_glib()).collect();
            let values: Vec<glib::gobject_ffi::GValue> = properties
                .iter()
                .map(|(_, v)| *v.to_value().to_glib_none().0)
                .collect();

            ffi::gtk_accessible_update_property_value(
                self.as_ref().to_glib_none().0,
                properties.len() as i32,
                mut_override(properties_ptr.as_ptr()),
                values.as_ptr(),
            )
        }
    }

    fn update_relation(&self, relations: &[(AccessibleRelation, &dyn ToValue)]) {
        unsafe {
            let relations_ptr: Vec<ffi::GtkAccessibleRelation> =
                relations.iter().map(|(k, _)| k.into_glib()).collect();
            let values: Vec<glib::gobject_ffi::GValue> = relations
                .iter()
                .map(|(_, v)| *v.to_value().to_glib_none().0)
                .collect();

            ffi::gtk_accessible_update_relation_value(
                self.as_ref().to_glib_none().0,
                relations.len() as i32,
                mut_override(relations_ptr.as_ptr()),
                values.as_ptr(),
            )
        }
    }

    fn update_state(&self, states: &[(AccessibleState, &dyn ToValue)]) {
        unsafe {
            let values: Vec<glib::gobject_ffi::GValue> = states
                .iter()
                .map(|(_, v)| *v.to_value().to_glib_none().0)
                .collect();
            let states_ptr: Vec<ffi::GtkAccessibleState> =
                states.iter().map(|(k, _)| k.into_glib()).collect();

            ffi::gtk_accessible_update_state_value(
                self.as_ref().to_glib_none().0,
                states.len() as i32,
                mut_override(states_ptr.as_ptr()),
                values.as_ptr(),
            )
        }
    }
}
