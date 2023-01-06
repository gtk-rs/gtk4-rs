// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, ListStore, TreeIter, TreeModel};
use glib::{translate::*, ToValue, Type, Value};
use libc::c_int;
use std::ptr;

impl ListStore {
    #[doc(alias = "gtk_list_store_newv")]
    #[doc(alias = "gtk_list_store_new")]
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    pub fn new(column_types: &[Type]) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut column_types = column_types
                .iter()
                .map(|t| t.into_glib())
                .collect::<Vec<_>>();
            from_glib_full(ffi::gtk_list_store_newv(
                column_types.len() as c_int,
                column_types.as_mut_ptr(),
            ))
        }
    }

    #[doc(alias = "gtk_list_store_insert_with_values")]
    #[doc(alias = "gtk_list_store_insert_with_valuesv")]
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    pub fn insert_with_values(
        &self,
        position: Option<u32>,
        columns_and_values: &[(u32, &dyn ToValue)],
    ) -> TreeIter {
        unsafe {
            assert!(
                position.unwrap_or(0) <= i32::max_value() as u32,
                "can't have more than {} rows",
                i32::max_value()
            );
            let n_columns =
                ffi::gtk_tree_model_get_n_columns(self.upcast_ref::<TreeModel>().to_glib_none().0)
                    as u32;
            assert!(
                columns_and_values.len() <= n_columns as usize,
                "got values for {} columns but only {n_columns} columns exist",
                columns_and_values.len(),
            );
            for (column, value) in columns_and_values {
                assert!(
                    *column < n_columns,
                    "got column {column} which is higher than the number of columns {n_columns}",
                );
                let type_ = from_glib(ffi::gtk_tree_model_get_column_type(
                    self.upcast_ref::<TreeModel>().to_glib_none().0,
                    *column as c_int,
                ));
                assert!(
                    Value::type_transformable(value.value_type(), type_),
                    "column {column} is of type {type_} but found value of type {}",
                    value.value_type()
                );
            }

            let columns = columns_and_values
                .iter()
                .map(|(c, _)| *c)
                .collect::<Vec<_>>();
            let values = columns_and_values
                .iter()
                .map(|(_, v)| v.to_value())
                .collect::<Vec<_>>();

            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_insert_with_valuesv(
                self.to_glib_none().0,
                iter.to_glib_none_mut().0,
                position.map_or(-1, |n| n as c_int),
                mut_override(columns.as_ptr() as *const c_int),
                mut_override(values.as_ptr() as *const glib::gobject_ffi::GValue),
                columns.len() as c_int,
            );
            iter
        }
    }

    #[doc(alias = "gtk_list_store_reorder")]
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    pub fn reorder(&self, new_order: &[u32]) {
        unsafe {
            let count = ffi::gtk_tree_model_iter_n_children(
                self.upcast_ref::<TreeModel>().to_glib_none().0,
                ptr::null_mut(),
            );
            let safe_count = count as usize == new_order.len();
            debug_assert!(
                safe_count,
                "Incorrect `new_order` slice length. Expected `{count}`, found `{}`.",
                new_order.len()
            );
            let safe_values = new_order.iter().max().map_or(true, |&max| {
                let max = max as i32;
                max >= 0 && max < count
            });
            debug_assert!(
                safe_values,
                "Some `new_order` slice values are out of range. Maximum safe value: \
                 `{}`. The slice contents: `{new_order:?}`",
                count - 1,
            );
            if safe_count && safe_values {
                ffi::gtk_list_store_reorder(
                    self.to_glib_none().0,
                    mut_override(new_order.as_ptr() as *const c_int),
                );
            }
        }
    }

    #[doc(alias = "gtk_list_store_set")]
    #[doc(alias = "gtk_list_store_set_valuesv")]
    #[doc(alias = "gtk_list_store_set_valist")]
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    pub fn set(&self, iter: &TreeIter, columns_and_values: &[(u32, &dyn ToValue)]) {
        unsafe {
            let n_columns =
                ffi::gtk_tree_model_get_n_columns(self.upcast_ref::<TreeModel>().to_glib_none().0)
                    as u32;
            assert!(
                columns_and_values.len() <= n_columns as usize,
                "got values for {} columns but only {n_columns} columns exist",
                columns_and_values.len(),
            );
            for (column, value) in columns_and_values {
                assert!(
                    *column < n_columns,
                    "got column {column} which is higher than the number of columns {n_columns}",
                );
                let type_ = from_glib(ffi::gtk_tree_model_get_column_type(
                    self.upcast_ref::<TreeModel>().to_glib_none().0,
                    *column as c_int,
                ));
                assert!(
                    Value::type_transformable(value.value_type(), type_),
                    "column {column} is of type {type_} but found value of type {}",
                    value.value_type()
                );
            }

            let columns = columns_and_values
                .iter()
                .map(|(c, _)| *c)
                .collect::<Vec<_>>();
            let values = columns_and_values
                .iter()
                .map(|(_, v)| v.to_value())
                .collect::<Vec<_>>();

            ffi::gtk_list_store_set_valuesv(
                self.to_glib_none().0,
                mut_override(iter.to_glib_none().0),
                mut_override(columns.as_ptr() as *const c_int),
                mut_override(values.as_ptr() as *const glib::gobject_ffi::GValue),
                columns.len() as c_int,
            );
        }
    }

    #[doc(alias = "gtk_list_store_set_column_types")]
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    pub fn set_column_types(&self, types: &[glib::Type]) {
        unsafe {
            let types_ptr: Vec<glib::ffi::GType> = types.iter().map(|t| t.into_glib()).collect();
            ffi::gtk_list_store_set_column_types(
                self.to_glib_none().0,
                types.len() as i32,
                mut_override(types_ptr.as_ptr()),
            )
        }
    }

    #[doc(alias = "gtk_list_store_set_value")]
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    pub fn set_value(&self, iter: &TreeIter, column: u32, value: &Value) {
        unsafe {
            let columns =
                ffi::gtk_tree_model_get_n_columns(self.upcast_ref::<TreeModel>().to_glib_none().0)
                    as u32;
            assert!(
                column < columns,
                "got column {column} which is higher than the number of columns {columns}",
            );

            let type_ = from_glib(ffi::gtk_tree_model_get_column_type(
                self.upcast_ref::<TreeModel>().to_glib_none().0,
                column as c_int,
            ));
            assert!(
                Value::type_transformable(value.type_(), type_),
                "column {column} is of type {type_} but found value of type {}",
                value.type_()
            );

            ffi::gtk_list_store_set_value(
                self.to_glib_none().0,
                mut_override(iter.to_glib_none().0),
                column as c_int,
                mut_override(value.to_glib_none().0),
            );
        }
    }
}
