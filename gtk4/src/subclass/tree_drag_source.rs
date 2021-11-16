// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`TreeDragSource`](crate::TreeDragSource) interface.

use crate::subclass::prelude::*;
use crate::{TreeDragSource, TreePath};
use glib::translate::*;
use glib::Cast;

pub trait TreeDragSourceImpl: ObjectImpl {
    fn row_draggable(&self, tree_drag_source: &Self::Type, path: &TreePath) -> bool {
        self.parent_row_draggable(tree_drag_source, path)
    }
    fn drag_data_get(&self, tree_drag_source: &Self::Type, path: &TreePath)
        -> gdk::ContentProvider;
    fn drag_data_delete(&self, tree_drag_source: &Self::Type, path: &TreePath) -> bool;
}

pub trait TreeDragSourceImplExt: ObjectSubclass {
    fn parent_row_draggable(&self, _tree_drag_source: &Self::Type, _path: &TreePath) -> bool;
    fn parent_drag_data_get(
        &self,
        tree_drag_source: &Self::Type,
        path: &TreePath,
    ) -> gdk::ContentProvider;
    fn parent_drag_data_delete(&self, tree_drag_source: &Self::Type, path: &TreePath) -> bool;
}

impl<T: TreeDragSourceImpl> TreeDragSourceImplExt for T {
    fn parent_row_draggable(&self, tree_drag_source: &Self::Type, path: &TreePath) -> bool {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<TreeDragSource>()
                as *const ffi::GtkTreeDragSourceIface;

            if let Some(func) = (*parent_iface).row_draggable {
                from_glib(func(
                    tree_drag_source
                        .unsafe_cast_ref::<TreeDragSource>()
                        .to_glib_none()
                        .0,
                    mut_override(path.to_glib_none().0),
                ))
            } else {
                // Assume the row is draggable by default
                true
            }
        }
    }

    fn parent_drag_data_get(
        &self,
        tree_drag_source: &Self::Type,
        path: &TreePath,
    ) -> gdk::ContentProvider {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<TreeDragSource>()
                as *const ffi::GtkTreeDragSourceIface;

            let func = (*parent_iface)
                .drag_data_get
                .expect("no parent \"drag_data_get\" implementation");

            from_glib_full(func(
                tree_drag_source
                    .unsafe_cast_ref::<TreeDragSource>()
                    .to_glib_none()
                    .0,
                mut_override(path.to_glib_none().0),
            ))
        }
    }

    fn parent_drag_data_delete(&self, tree_drag_source: &Self::Type, path: &TreePath) -> bool {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<TreeDragSource>()
                as *const ffi::GtkTreeDragSourceIface;

            let func = (*parent_iface)
                .drag_data_delete
                .expect("no parent \"drag_data_delete\" implementation");

            from_glib(func(
                tree_drag_source
                    .unsafe_cast_ref::<TreeDragSource>()
                    .to_glib_none()
                    .0,
                mut_override(path.to_glib_none().0),
            ))
        }
    }
}

unsafe impl<T: TreeDragSourceImpl> IsImplementable<T> for TreeDragSource {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        assert!(
            crate::rt::is_initialized(),
            "GTK has to be initialized first"
        );

        iface.row_draggable = Some(tree_drag_source_row_draggable::<T>);
        iface.drag_data_get = Some(tree_drag_source_drag_data_get::<T>);
        iface.drag_data_delete = Some(tree_drag_source_drag_data_delete::<T>);
    }
}

unsafe extern "C" fn tree_drag_source_row_draggable<T: TreeDragSourceImpl>(
    tree_drag_source: *mut ffi::GtkTreeDragSource,
    pathptr: *mut ffi::GtkTreePath,
) -> glib::ffi::gboolean {
    let instance = &*(tree_drag_source as *mut T::Instance);
    let imp = instance.impl_();

    let path: Borrowed<TreePath> = from_glib_borrow(pathptr);

    imp.row_draggable(
        from_glib_borrow::<_, TreeDragSource>(tree_drag_source).unsafe_cast_ref(),
        &path,
    )
    .into_glib()
}

unsafe extern "C" fn tree_drag_source_drag_data_get<T: TreeDragSourceImpl>(
    tree_drag_source: *mut ffi::GtkTreeDragSource,
    pathptr: *mut ffi::GtkTreePath,
) -> *mut gdk::ffi::GdkContentProvider {
    let instance = &*(tree_drag_source as *mut T::Instance);
    let imp = instance.impl_();
    let path: Borrowed<TreePath> = from_glib_borrow(pathptr);

    imp.drag_data_get(
        from_glib_borrow::<_, TreeDragSource>(tree_drag_source).unsafe_cast_ref(),
        &path,
    )
    .to_glib_full()
}

unsafe extern "C" fn tree_drag_source_drag_data_delete<T: TreeDragSourceImpl>(
    tree_drag_source: *mut ffi::GtkTreeDragSource,
    pathptr: *mut ffi::GtkTreePath,
) -> glib::ffi::gboolean {
    let instance = &*(tree_drag_source as *mut T::Instance);
    let imp = instance.impl_();
    let path: Borrowed<TreePath> = from_glib_borrow(pathptr);
    imp.drag_data_delete(
        from_glib_borrow::<_, TreeDragSource>(tree_drag_source).unsafe_cast_ref(),
        &path,
    )
    .into_glib()
}
