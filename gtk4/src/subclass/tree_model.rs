// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::{TreeIter, TreeModel, TreeModelFlags, TreePath};
use glib::translate::*;
use glib::{Cast, Value};

/// # Safety:
///
/// The TreeModel trait is unsafe because it expect you to create
/// a TreeIter, creating such objects along with the optional
/// `ref_node` & `unref_node` functions cannot be done with safe Rust
pub unsafe trait TreeModelImpl: ObjectImpl {
    fn row_changed(&self, tree_model: &Self::Type, path: &TreePath, iter: &TreeIter) {
        self.parent_row_changed(tree_model, path, iter)
    }

    fn row_inserted(&self, tree_model: &Self::Type, path: &TreePath, iter: &TreeIter) {
        self.parent_row_inserted(tree_model, path, iter)
    }

    fn row_has_child_toggled(&self, tree_model: &Self::Type, path: &TreePath, iter: &TreeIter) {
        self.parent_row_has_child_toggled(tree_model, path, iter)
    }

    fn row_deleted(&self, tree_model: &Self::Type, path: &TreePath) {
        self.parent_row_deleted(tree_model, path)
    }

    fn flags(&self, tree_model: &Self::Type) -> TreeModelFlags {
        self.parent_flags(tree_model)
    }

    fn n_columns(&self, tree_model: &Self::Type) -> i32;
    fn column_type(&self, tree_model: &Self::Type, index: i32) -> glib::Type;
    fn iter(&self, tree_model: &Self::Type, path: &TreePath) -> Option<TreeIter>;
    fn path(&self, tree_model: &Self::Type, iter: &TreeIter) -> TreePath;
    fn value(&self, tree_model: &Self::Type, iter: &TreeIter, index: i32) -> Value;
    fn iter_next(&self, tree_model: &Self::Type) -> Option<TreeIter>;

    fn iter_previous(&self, tree_model: &Self::Type) -> Option<TreeIter> {
        self.parent_iter_previous(tree_model)
    }

    fn iter_has_child(&self, tree_model: &Self::Type, iter: &TreeIter) -> bool;
    fn iter_n_children(&self, tree_model: &Self::Type, iter: Option<&TreeIter>) -> i32;
    fn iter_nth_child(
        &self,
        tree_model: &Self::Type,
        parent: Option<&TreeIter>,
        index: i32,
    ) -> Option<TreeIter>;
    fn iter_parent(&self, tree_model: &Self::Type, child: &TreeIter) -> Option<TreeIter>;

    fn ref_node(&self, tree_model: &Self::Type, iter: &TreeIter) {
        self.parent_ref_node(tree_model, iter)
    }

    fn unref_node(&self, tree_model: &Self::Type, iter: &TreeIter) {
        self.parent_unref_node(tree_model, iter)
    }
}

pub trait TreeModelImplExt: ObjectSubclass {
    fn parent_row_changed(&self, tree_model: &Self::Type, path: &TreePath, iter: &TreeIter);
    fn parent_row_inserted(&self, tree_model: &Self::Type, path: &TreePath, iter: &TreeIter);
    fn parent_row_has_child_toggled(
        &self,
        tree_model: &Self::Type,
        path: &TreePath,
        iter: &TreeIter,
    );
    fn parent_row_deleted(&self, tree_model: &Self::Type, path: &TreePath);
    fn parent_flags(&self, tree_model: &Self::Type) -> TreeModelFlags;
    fn parent_iter_previous(&self, tree_model: &Self::Type) -> Option<TreeIter>;
    fn parent_ref_node(&self, tree_model: &Self::Type, iter: &TreeIter);
    fn parent_unref_node(&self, tree_model: &Self::Type, iter: &TreeIter);
}

impl<O: TreeModelImpl> TreeModelImplExt for O {
    fn parent_row_changed(&self, tree_model: &Self::Type, path: &TreePath, iter: &TreeIter) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<TreeModel>() as *const ffi::GtkTreeModelIface;

            if let Some(f) = (*parent_iface).row_changed.as_ref() {
                f(
                    tree_model.unsafe_cast_ref::<TreeModel>().to_glib_none().0,
                    mut_override(path.to_glib_none().0),
                    mut_override(iter.to_glib_none().0),
                );
            }
        }
    }

    fn parent_row_inserted(&self, tree_model: &Self::Type, path: &TreePath, iter: &TreeIter) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<TreeModel>() as *const ffi::GtkTreeModelIface;

            if let Some(f) = (*parent_iface).row_inserted.as_ref() {
                f(
                    tree_model.unsafe_cast_ref::<TreeModel>().to_glib_none().0,
                    mut_override(path.to_glib_none().0),
                    mut_override(iter.to_glib_none().0),
                );
            }
        }
    }

    fn parent_row_has_child_toggled(
        &self,
        tree_model: &Self::Type,
        path: &TreePath,
        iter: &TreeIter,
    ) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<TreeModel>() as *const ffi::GtkTreeModelIface;

            if let Some(f) = (*parent_iface).row_has_child_toggled.as_ref() {
                f(
                    tree_model.unsafe_cast_ref::<TreeModel>().to_glib_none().0,
                    mut_override(path.to_glib_none().0),
                    mut_override(iter.to_glib_none().0),
                );
            }
        }
    }

    fn parent_row_deleted(&self, tree_model: &Self::Type, path: &TreePath) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<TreeModel>() as *const ffi::GtkTreeModelIface;

            if let Some(f) = (*parent_iface).row_deleted.as_ref() {
                f(
                    tree_model.unsafe_cast_ref::<TreeModel>().to_glib_none().0,
                    mut_override(path.to_glib_none().0),
                );
            }
        }
    }

    fn parent_flags(&self, tree_model: &Self::Type) -> TreeModelFlags {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<TreeModel>() as *const ffi::GtkTreeModelIface;

            let f = (*parent_iface).get_flags.as_ref().unwrap();
            let ret = f(tree_model.unsafe_cast_ref::<TreeModel>().to_glib_none().0);

            from_glib(ret)
        }
    }

    fn parent_iter_previous(&self, tree_model: &Self::Type) -> Option<TreeIter> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<TreeModel>() as *const ffi::GtkTreeModelIface;

            let iter = std::ptr::null_mut();
            let f = (*parent_iface).iter_previous.as_ref().unwrap();
            let ret: bool = from_glib(f(
                tree_model.unsafe_cast_ref::<TreeModel>().to_glib_none().0,
                iter,
            ));

            if ret {
                let iter = from_glib_none(iter);
                Some(iter)
            } else {
                None
            }
        }
    }

    fn parent_ref_node(&self, tree_model: &Self::Type, iter: &TreeIter) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<TreeModel>() as *const ffi::GtkTreeModelIface;

            if let Some(f) = (*parent_iface).ref_node.as_ref() {
                f(
                    tree_model.unsafe_cast_ref::<TreeModel>().to_glib_none().0,
                    mut_override(iter.to_glib_none().0),
                );
            }
        }
    }

    fn parent_unref_node(&self, tree_model: &Self::Type, iter: &TreeIter) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<TreeModel>() as *const ffi::GtkTreeModelIface;

            if let Some(f) = (*parent_iface).unref_node.as_ref() {
                f(
                    tree_model.unsafe_cast_ref::<TreeModel>().to_glib_none().0,
                    mut_override(iter.to_glib_none().0),
                );
            }
        }
    }
}

unsafe impl<T: TreeModelImpl> IsImplementable<T> for TreeModel {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.row_changed = Some(tree_model_row_changed::<T>);
        iface.row_inserted = Some(tree_model_row_inserted::<T>);
        iface.row_has_child_toggled = Some(tree_model_row_has_child_toggled::<T>);
        iface.row_deleted = Some(tree_model_row_deleted::<T>);
        iface.get_flags = Some(tree_model_get_flags::<T>);
        iface.get_n_columns = Some(tree_model_get_n_columns::<T>);
        iface.get_column_type = Some(tree_model_get_column_type::<T>);
        iface.get_iter = Some(tree_model_get_iter::<T>);
        iface.get_path = Some(tree_model_get_path::<T>);
        iface.get_value = Some(tree_model_get_value::<T>);
        iface.iter_next = Some(tree_model_iter_next::<T>);
        iface.iter_previous = Some(tree_model_iter_previous::<T>);
        iface.iter_has_child = Some(tree_model_iter_has_child::<T>);
        iface.iter_n_children = Some(tree_model_iter_n_children::<T>);
        iface.iter_nth_child = Some(tree_model_iter_nth_child::<T>);
        iface.iter_parent = Some(tree_model_iter_parent::<T>);
        iface.ref_node = Some(tree_model_ref_node::<T>);
        iface.unref_node = Some(tree_model_unref_node::<T>);
    }

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}

unsafe extern "C" fn tree_model_row_changed<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    treeptr: *mut ffi::GtkTreePath,
    iterptr: *mut ffi::GtkTreeIter,
) {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.impl_();

    let tree: Borrowed<TreePath> = from_glib_borrow(treeptr);
    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);

    imp.row_changed(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        &tree,
        &iter,
    )
}

unsafe extern "C" fn tree_model_row_inserted<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    treeptr: *mut ffi::GtkTreePath,
    iterptr: *mut ffi::GtkTreeIter,
) {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.impl_();

    let tree: Borrowed<TreePath> = from_glib_borrow(treeptr);
    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);

    imp.row_inserted(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        &tree,
        &iter,
    )
}

unsafe extern "C" fn tree_model_row_has_child_toggled<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    treeptr: *mut ffi::GtkTreePath,
    iterptr: *mut ffi::GtkTreeIter,
) {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.impl_();

    let tree: Borrowed<TreePath> = from_glib_borrow(treeptr);
    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);

    imp.row_has_child_toggled(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        &tree,
        &iter,
    )
}

unsafe extern "C" fn tree_model_row_deleted<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    treeptr: *mut ffi::GtkTreePath,
) {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.impl_();

    let tree: Borrowed<TreePath> = from_glib_borrow(treeptr);

    imp.row_deleted(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        &tree,
    )
}

unsafe extern "C" fn tree_model_get_flags<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
) -> ffi::GtkTreeModelFlags {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.impl_();

    imp.flags(from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref())
        .into_glib()
}

unsafe extern "C" fn tree_model_get_n_columns<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
) -> i32 {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.impl_();

    imp.n_columns(from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref())
}

unsafe extern "C" fn tree_model_get_column_type<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    index: i32,
) -> glib::ffi::GType {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.impl_();

    imp.column_type(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        index,
    )
    .into_glib()
}

unsafe extern "C" fn tree_model_get_iter<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
    pathptr: *mut ffi::GtkTreePath,
) -> glib::ffi::gboolean {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.impl_();

    let path: Borrowed<TreePath> = from_glib_borrow(pathptr);

    let ret = imp.iter(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        &path,
    );
    if let Some(iter) = ret {
        *iterptr = *iter.to_glib_none().0;
        true.into_glib()
    } else {
        false.into_glib()
    }
}

unsafe extern "C" fn tree_model_get_path<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
) -> *mut ffi::GtkTreePath {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.impl_();

    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);

    imp.path(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        &iter,
    )
    .to_glib_full() as *mut _
}

unsafe extern "C" fn tree_model_get_value<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
    index: i32,
    valueptr: *mut glib::gobject_ffi::GValue,
) {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.impl_();

    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);

    let ret = imp.value(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        &iter,
        index,
    );
    let ret = std::mem::ManuallyDrop::new(ret);
    *valueptr = *ret.to_glib_none().0;
}

unsafe extern "C" fn tree_model_iter_next<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
) -> glib::ffi::gboolean {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.impl_();

    let ret = imp.iter_next(from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref());
    if let Some(iter) = ret {
        *iterptr = *iter.to_glib_none().0;
        true.into_glib()
    } else {
        *iterptr = *std::ptr::null_mut();
        false.into_glib()
    }
}

unsafe extern "C" fn tree_model_iter_previous<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
) -> glib::ffi::gboolean {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.impl_();

    let ret = imp.iter_previous(from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref());
    if let Some(iter) = ret {
        *iterptr = *iter.to_glib_none().0;
        true.into_glib()
    } else {
        *iterptr = *std::ptr::null_mut();
        false.into_glib()
    }
}

unsafe extern "C" fn tree_model_iter_has_child<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
) -> glib::ffi::gboolean {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.impl_();

    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);
    imp.iter_has_child(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        &iter,
    )
    .into_glib()
}

unsafe extern "C" fn tree_model_iter_n_children<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
) -> i32 {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.impl_();

    let iter: Borrowed<Option<TreeIter>> = from_glib_borrow(iterptr);

    imp.iter_n_children(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        iter.as_ref().as_ref(),
    )
}

unsafe extern "C" fn tree_model_iter_nth_child<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    parent_iterptr: *mut ffi::GtkTreeIter,
    child_iterptr: *mut ffi::GtkTreeIter,
    index: i32,
) -> glib::ffi::gboolean {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.impl_();

    let parent: Borrowed<Option<TreeIter>> = from_glib_borrow(parent_iterptr);

    let ret = imp.iter_nth_child(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        parent.as_ref().as_ref(),
        index,
    );
    if let Some(child_iter) = ret {
        *child_iterptr = *child_iter.to_glib_none().0;
        true.into_glib()
    } else {
        *child_iterptr = *std::ptr::null_mut();
        false.into_glib()
    }
}

unsafe extern "C" fn tree_model_iter_parent<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    parent_iterptr: *mut ffi::GtkTreeIter,
    child_iterptr: *mut ffi::GtkTreeIter,
) -> glib::ffi::gboolean {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.impl_();

    let child: Borrowed<TreeIter> = from_glib_borrow(child_iterptr);

    let ret = imp.iter_parent(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        &child,
    );
    if let Some(parent_iter) = ret {
        *parent_iterptr = *parent_iter.to_glib_none().0;
        true.into_glib()
    } else {
        *parent_iterptr = *std::ptr::null_mut();
        false.into_glib()
    }
}

unsafe extern "C" fn tree_model_ref_node<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
) {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.impl_();

    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);

    imp.ref_node(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        &iter,
    )
}

unsafe extern "C" fn tree_model_unref_node<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
) {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.impl_();

    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);

    imp.unref_node(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        &iter,
    )
}
