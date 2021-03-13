// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{TreeDragSource, TreePath};
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

pub trait TreeDragSourceImpl: ObjectImpl {
    fn row_draggable(&self, _tree_drag_source: &Self::Type, _path: &TreePath) -> bool {
        // Assume the row is draggable by default
        true
    }
    fn drag_data_get(&self, tree_drag_source: &Self::Type, path: &TreePath)
        -> gdk::ContentProvider;
    fn drag_data_delete(&self, tree_drag_source: &Self::Type, path: &TreePath) -> bool;
}

unsafe impl<T: TreeDragSourceImpl> IsImplementable<T> for TreeDragSource {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.row_draggable = Some(tree_drag_source_row_draggable::<T>);
        iface.drag_data_get = Some(tree_drag_source_drag_data_get::<T>);
        iface.drag_data_delete = Some(tree_drag_source_drag_data_delete::<T>);
    }

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}

unsafe extern "C" fn tree_drag_source_row_draggable<T: TreeDragSourceImpl>(
    tree_drag_source: *mut ffi::GtkTreeDragSource,
    pathptr: *mut ffi::GtkTreePath,
) -> glib::ffi::gboolean {
    let instance = &*(tree_drag_source as *mut T::Instance);
    let imp = instance.get_impl();

    let path: Borrowed<TreePath> = from_glib_borrow(pathptr);

    imp.row_draggable(
        from_glib_borrow::<_, TreeDragSource>(tree_drag_source).unsafe_cast_ref(),
        &path,
    )
    .to_glib()
}

unsafe extern "C" fn tree_drag_source_drag_data_get<T: TreeDragSourceImpl>(
    tree_drag_source: *mut ffi::GtkTreeDragSource,
    pathptr: *mut ffi::GtkTreePath,
) -> *mut gdk::ffi::GdkContentProvider {
    let instance = &*(tree_drag_source as *mut T::Instance);
    let imp = instance.get_impl();
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
    let imp = instance.get_impl();
    let path: Borrowed<TreePath> = from_glib_borrow(pathptr);
    imp.drag_data_delete(
        from_glib_borrow::<_, TreeDragSource>(tree_drag_source).unsafe_cast_ref(),
        &path,
    )
    .to_glib()
}
