// Take a look at the license at the top of the repository in the LICENSE file.

use crate::prelude::*;
use crate::{CellArea, CellRenderer};
use glib::translate::*;
use glib::{value::FromValue, IsA, ToValue};

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`CellArea`](crate::CellArea).
#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
pub trait CellAreaExtManual {
    #[doc(alias = "gtk_cell_area_add_with_properties")]
    fn add_with_properties(
        &self,
        renderer: &impl IsA<CellRenderer>,
        properties: &[(&str, &dyn ToValue)],
    );

    #[doc(alias = "gtk_cell_area_cell_get_valist")]
    #[doc(alias = "gtk_cell_area_cell_get_property")]
    fn cell_get_value(&self, renderer: &impl IsA<CellRenderer>, property_name: &str)
        -> glib::Value;

    // rustdoc-stripper-ignore-next
    /// Similar to [`Self::cell_get_value`] but panics if the value is of a different type.
    #[doc(alias = "gtk_cell_area_cell_get_valist")]
    #[doc(alias = "gtk_cell_area_cell_get_property")]
    fn cell_get<V: for<'b> FromValue<'b> + 'static>(
        &self,
        renderer: &impl IsA<CellRenderer>,
        property_name: &str,
    ) -> V;

    #[doc(alias = "gtk_cell_area_cell_set_valist")]
    #[doc(alias = "gtk_cell_area_cell_set_property")]
    fn cell_set(&self, renderer: &impl IsA<CellRenderer>, property_name: &str, value: &dyn ToValue);
}

impl<O: IsA<CellArea>> CellAreaExtManual for O {
    fn add_with_properties(
        &self,
        renderer: &impl IsA<CellRenderer>,
        properties: &[(&str, &dyn ToValue)],
    ) {
        self.as_ref().add(renderer);
        properties.iter().for_each(|(property_name, value)| {
            self.as_ref().cell_set(renderer, property_name, *value);
        });
    }

    fn cell_get_value(
        &self,
        renderer: &impl IsA<CellRenderer>,
        property_name: &str,
    ) -> glib::Value {
        unsafe {
            let cell_class = glib::Class::<CellArea>::from_type(O::static_type()).unwrap();
            let pspec: Option<glib::ParamSpec> =
                from_glib_none(ffi::gtk_cell_area_class_find_cell_property(
                    cell_class.as_ref() as *const _ as *mut ffi::GtkCellAreaClass,
                    property_name.to_glib_none().0,
                ));
            let pspec = pspec
                .unwrap_or_else(|| panic!("The CellArea property {property_name} doesn't exists"));
            let mut value = glib::Value::from_type(pspec.value_type());
            ffi::gtk_cell_area_cell_get_property(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value
        }
    }

    fn cell_get<V: for<'b> FromValue<'b> + 'static>(
        &self,
        renderer: &impl IsA<CellRenderer>,
        property_name: &str,
    ) -> V {
        let value = self.cell_get_value(renderer, property_name);
        value
            .get_owned::<V>()
            .expect("Failed to get value of renderer")
    }

    fn cell_set(
        &self,
        renderer: &impl IsA<CellRenderer>,
        property_name: &str,
        value: &dyn ToValue,
    ) {
        unsafe {
            let cell_class = glib::Class::<CellArea>::from_type(O::static_type()).unwrap();
            let pspec: Option<glib::ParamSpec> =
                from_glib_none(ffi::gtk_cell_area_class_find_cell_property(
                    cell_class.as_ref() as *const _ as *mut ffi::GtkCellAreaClass,
                    property_name.to_glib_none().0,
                ));
            let pspec = pspec
                .unwrap_or_else(|| panic!("The CellArea property {property_name} doesn't exists"));

            assert!(
                pspec.value_type().is_a(value.value_type()),
                "The CellArea property's value is of wrong type. Expected '{}' but got '{}'",
                pspec.value_type(),
                value.value_type()
            );

            ffi::gtk_cell_area_cell_set_property(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                value.to_value().to_glib_none().0,
            );
        }
    }
}
