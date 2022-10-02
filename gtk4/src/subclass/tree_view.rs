// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`TreeView`](crate::TreeView).

use crate::subclass::prelude::*;
use crate::{MovementStep, TreeIter, TreePath, TreeView, TreeViewColumn};
use glib::translate::*;
use glib::Cast;

pub trait TreeViewImpl: TreeViewImplExt + WidgetImpl {
    fn columns_changed(&self, tree_view: &Self::Type) {
        self.parent_columns_changed(tree_view)
    }

    fn cursor_changed(&self, tree_view: &Self::Type) {
        self.parent_cursor_changed(tree_view)
    }

    fn expand_collapse_cursor_row(
        &self,
        tree_view: &Self::Type,
        logical: bool,
        expand: bool,
        open_all: bool,
    ) -> bool {
        self.parent_expand_collapse_cursor_row(tree_view, logical, expand, open_all)
    }

    fn move_cursor(
        &self,
        tree_view: &Self::Type,
        step: MovementStep,
        count: i32,
        expand: bool,
        modify: bool,
    ) -> bool {
        self.parent_move_cursor(tree_view, step, count, expand, modify)
    }

    fn row_activated(&self, tree_view: &Self::Type, path: &TreePath, column: &TreeViewColumn) {
        self.parent_row_activated(tree_view, path, column)
    }

    fn row_collapsed(&self, tree_view: &Self::Type, iter: &TreeIter, path: &TreePath) {
        self.parent_row_collapsed(tree_view, iter, path)
    }

    fn row_expanded(&self, tree_view: &Self::Type, iter: &TreeIter, path: &TreePath) {
        self.parent_row_expanded(tree_view, iter, path)
    }

    fn select_all(&self, tree_view: &Self::Type) -> bool {
        self.parent_select_all(tree_view)
    }

    fn select_cursor_parent(&self, tree_view: &Self::Type) -> bool {
        self.parent_select_cursor_parent(tree_view)
    }

    fn select_cursor_row(&self, tree_view: &Self::Type, start_editing: bool) -> bool {
        self.parent_select_cursor_row(tree_view, start_editing)
    }

    fn start_interactive_search(&self, tree_view: &Self::Type) -> bool {
        self.parent_start_interactive_search(tree_view)
    }

    fn test_collapse_row(&self, tree_view: &Self::Type, iter: &TreeIter, path: &TreePath) -> bool {
        self.parent_test_collapse_row(tree_view, iter, path)
    }

    fn test_expand_row(&self, tree_view: &Self::Type, iter: &TreeIter, path: &TreePath) -> bool {
        self.parent_test_expand_row(tree_view, iter, path)
    }

    fn toggle_cursor_row(&self, tree_view: &Self::Type) -> bool {
        self.parent_toggle_cursor_row(tree_view)
    }

    fn unselect_all(&self, tree_view: &Self::Type) -> bool {
        self.parent_unselect_all(tree_view)
    }
}

pub trait TreeViewImplExt: ObjectSubclass {
    fn parent_columns_changed(&self, tree_view: &Self::Type);
    fn parent_cursor_changed(&self, tree_view: &Self::Type);
    fn parent_expand_collapse_cursor_row(
        &self,
        tree_view: &Self::Type,
        logical: bool,
        expand: bool,
        open_all: bool,
    ) -> bool;
    fn parent_move_cursor(
        &self,
        tree_view: &Self::Type,
        step: MovementStep,
        count: i32,
        extend: bool,
        modify: bool,
    ) -> bool;
    fn parent_row_activated(
        &self,
        tree_view: &Self::Type,
        path: &TreePath,
        column: &TreeViewColumn,
    );
    fn parent_row_collapsed(&self, tree_view: &Self::Type, iter: &TreeIter, path: &TreePath);
    fn parent_row_expanded(&self, tree_view: &Self::Type, iter: &TreeIter, path: &TreePath);
    fn parent_select_all(&self, tree_view: &Self::Type) -> bool;
    fn parent_select_cursor_parent(&self, tree_view: &Self::Type) -> bool;
    fn parent_select_cursor_row(&self, tree_view: &Self::Type, start_editing: bool) -> bool;
    fn parent_start_interactive_search(&self, tree_view: &Self::Type) -> bool;
    fn parent_test_collapse_row(
        &self,
        tree_view: &Self::Type,
        iter: &TreeIter,
        path: &TreePath,
    ) -> bool;
    fn parent_test_expand_row(
        &self,
        tree_view: &Self::Type,
        iter: &TreeIter,
        path: &TreePath,
    ) -> bool;
    fn parent_toggle_cursor_row(&self, tree_view: &Self::Type) -> bool;
    fn parent_unselect_all(&self, tree_view: &Self::Type) -> bool;
}

impl<T: TreeViewImpl> TreeViewImplExt for T {
    fn parent_columns_changed(&self, tree_view: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            // columns-changed is a signal
            if let Some(f) = (*parent_class).columns_changed {
                f(tree_view.unsafe_cast_ref::<TreeView>().to_glib_none().0)
            }
        }
    }

    fn parent_cursor_changed(&self, tree_view: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            // cursor-changed is a signal
            if let Some(f) = (*parent_class).cursor_changed {
                f(tree_view.unsafe_cast_ref::<TreeView>().to_glib_none().0)
            }
        }
    }

    fn parent_expand_collapse_cursor_row(
        &self,
        tree_view: &Self::Type,
        logical: bool,
        expand: bool,
        open_all: bool,
    ) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            // expand-collapse-cursor-row is a signal
            if let Some(f) = (*parent_class).expand_collapse_cursor_row {
                from_glib(f(
                    tree_view.unsafe_cast_ref::<TreeView>().to_glib_none().0,
                    logical.into_glib(),
                    expand.into_glib(),
                    open_all.into_glib(),
                ))
            } else {
                false
            }
        }
    }

    fn parent_move_cursor(
        &self,
        tree_view: &Self::Type,
        step: MovementStep,
        count: i32,
        extend: bool,
        modify: bool,
    ) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            // move-cursor is a signal
            if let Some(f) = (*parent_class).move_cursor {
                from_glib(f(
                    tree_view.unsafe_cast_ref::<TreeView>().to_glib_none().0,
                    step.into_glib(),
                    count,
                    extend.into_glib(),
                    modify.into_glib(),
                ))
            } else {
                // return false if step is not supported
                false
            }
        }
    }

    fn parent_row_activated(
        &self,
        tree_view: &Self::Type,
        path: &TreePath,
        column: &TreeViewColumn,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            // row-activated is a signal
            if let Some(f) = (*parent_class).row_activated {
                f(
                    tree_view.unsafe_cast_ref::<TreeView>().to_glib_none().0,
                    path.to_glib_none().0 as *mut _,
                    column.to_glib_none().0,
                );
            }
        }
    }

    fn parent_row_collapsed(&self, tree_view: &Self::Type, iter: &TreeIter, path: &TreePath) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            // row-collapsed is a signal
            if let Some(f) = (*parent_class).row_collapsed {
                f(
                    tree_view.unsafe_cast_ref::<TreeView>().to_glib_none().0,
                    iter.to_glib_none().0 as *mut _,
                    path.to_glib_none().0 as *mut _,
                )
            }
        }
    }

    fn parent_row_expanded(&self, tree_view: &Self::Type, iter: &TreeIter, path: &TreePath) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            // row-expanded is a signal
            if let Some(f) = (*parent_class).row_expanded {
                f(
                    tree_view.unsafe_cast_ref::<TreeView>().to_glib_none().0,
                    iter.to_glib_none().0 as *mut _,
                    path.to_glib_none().0 as *mut _,
                )
            }
        }
    }

    fn parent_select_all(&self, tree_view: &Self::Type) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            // select-all is a signal
            if let Some(f) = (*parent_class).select_all {
                from_glib(f(tree_view.unsafe_cast_ref::<TreeView>().to_glib_none().0))
            } else {
                false
            }
        }
    }

    fn parent_select_cursor_parent(&self, tree_view: &Self::Type) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            // select-cursor-parent is a signal
            if let Some(f) = (*parent_class).select_cursor_parent {
                from_glib(f(tree_view.unsafe_cast_ref::<TreeView>().to_glib_none().0))
            } else {
                false
            }
        }
    }

    fn parent_select_cursor_row(&self, tree_view: &Self::Type, start_editing: bool) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            // select-cursor-row is a signal
            if let Some(f) = (*parent_class).select_cursor_row {
                from_glib(f(
                    tree_view.unsafe_cast_ref::<TreeView>().to_glib_none().0,
                    start_editing.into_glib(),
                ))
            } else {
                false
            }
        }
    }

    fn parent_start_interactive_search(&self, tree_view: &Self::Type) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            // start-interactive-search is a signal
            if let Some(f) = (*parent_class).start_interactive_search {
                from_glib(f(tree_view.unsafe_cast_ref::<TreeView>().to_glib_none().0))
            } else {
                false
            }
        }
    }

    fn parent_test_collapse_row(
        &self,
        tree_view: &Self::Type,
        iter: &TreeIter,
        path: &TreePath,
    ) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            // test-collapse-row is a signal
            if let Some(f) = (*parent_class).test_collapse_row {
                from_glib(f(
                    tree_view.unsafe_cast_ref::<TreeView>().to_glib_none().0,
                    iter.to_glib_none().0 as *mut _,
                    path.to_glib_none().0 as *mut _,
                ))
            } else {
                // false to allow collapsing, true to reject it
                false
            }
        }
    }

    fn parent_test_expand_row(
        &self,
        tree_view: &Self::Type,
        iter: &TreeIter,
        path: &TreePath,
    ) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            // test-expand-row is a signal
            if let Some(f) = (*parent_class).test_expand_row {
                from_glib(f(
                    tree_view.unsafe_cast_ref::<TreeView>().to_glib_none().0,
                    iter.to_glib_none().0 as *mut _,
                    path.to_glib_none().0 as *mut _,
                ))
            } else {
                // false to allow expanding, true to reject it
                false
            }
        }
    }

    fn parent_toggle_cursor_row(&self, tree_view: &Self::Type) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            // toggle-cursor-row is a signal
            if let Some(f) = (*parent_class).toggle_cursor_row {
                from_glib(f(tree_view.unsafe_cast_ref::<TreeView>().to_glib_none().0))
            } else {
                false
            }
        }
    }

    fn parent_unselect_all(&self, tree_view: &Self::Type) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            // unselect-all is a signal
            if let Some(f) = (*parent_class).unselect_all {
                from_glib(f(tree_view.unsafe_cast_ref::<TreeView>().to_glib_none().0))
            } else {
                false
            }
        }
    }
}

unsafe impl<T: TreeViewImpl> IsSubclassable<T> for TreeView {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.columns_changed = Some(tree_view_columns_changed::<T>);
        klass.cursor_changed = Some(tree_view_cursor_changed::<T>);
        klass.expand_collapse_cursor_row = Some(tree_view_expand_collapse_cursor_row::<T>);
        klass.move_cursor = Some(tree_view_move_cursor::<T>);
        klass.row_activated = Some(tree_view_row_activated::<T>);
        klass.row_collapsed = Some(tree_view_row_collapsed::<T>);
        klass.row_expanded = Some(tree_view_row_expanded::<T>);
        klass.select_all = Some(tree_view_select_all::<T>);
        klass.select_cursor_parent = Some(tree_view_select_cursor_parent::<T>);
        klass.select_cursor_row = Some(tree_view_select_cursor_row::<T>);
        klass.start_interactive_search = Some(tree_view_start_interactive_search::<T>);
        klass.test_collapse_row = Some(tree_view_test_collapse_row::<T>);
        klass.test_expand_row = Some(tree_view_test_expand_row::<T>);
        klass.toggle_cursor_row = Some(tree_view_toggle_cursor_row::<T>);
        klass.unselect_all = Some(tree_view_unselect_all::<T>);
    }
}

unsafe extern "C" fn tree_view_columns_changed<T: TreeViewImpl>(ptr: *mut ffi::GtkTreeView) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TreeView> = from_glib_borrow(ptr);

    imp.columns_changed(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn tree_view_cursor_changed<T: TreeViewImpl>(ptr: *mut ffi::GtkTreeView) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TreeView> = from_glib_borrow(ptr);

    imp.cursor_changed(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn tree_view_expand_collapse_cursor_row<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
    logical: glib::ffi::gboolean,
    expand: glib::ffi::gboolean,
    open_all: glib::ffi::gboolean,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TreeView> = from_glib_borrow(ptr);

    imp.expand_collapse_cursor_row(
        wrap.unsafe_cast_ref(),
        from_glib(logical),
        from_glib(expand),
        from_glib(open_all),
    )
    .into_glib()
}

unsafe extern "C" fn tree_view_move_cursor<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
    step: ffi::GtkMovementStep,
    count: i32,
    extend: glib::ffi::gboolean,
    modify: glib::ffi::gboolean,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TreeView> = from_glib_borrow(ptr);

    imp.move_cursor(
        wrap.unsafe_cast_ref(),
        from_glib(step),
        count,
        from_glib(extend),
        from_glib(modify),
    )
    .into_glib()
}

unsafe extern "C" fn tree_view_row_activated<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
    pathptr: *mut ffi::GtkTreePath,
    columnptr: *mut ffi::GtkTreeViewColumn,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TreeView> = from_glib_borrow(ptr);
    let path: Borrowed<TreePath> = from_glib_borrow(pathptr);
    let column: Borrowed<TreeViewColumn> = from_glib_borrow(columnptr);

    imp.row_activated(wrap.unsafe_cast_ref(), &path, &column)
}

unsafe extern "C" fn tree_view_row_collapsed<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
    iterptr: *mut ffi::GtkTreeIter,
    pathptr: *mut ffi::GtkTreePath,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TreeView> = from_glib_borrow(ptr);
    let path: Borrowed<TreePath> = from_glib_borrow(pathptr);
    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);

    imp.row_collapsed(wrap.unsafe_cast_ref(), &iter, &path)
}

unsafe extern "C" fn tree_view_row_expanded<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
    iterptr: *mut ffi::GtkTreeIter,
    pathptr: *mut ffi::GtkTreePath,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TreeView> = from_glib_borrow(ptr);
    let path: Borrowed<TreePath> = from_glib_borrow(pathptr);
    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);

    imp.row_expanded(wrap.unsafe_cast_ref(), &iter, &path)
}

unsafe extern "C" fn tree_view_select_all<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TreeView> = from_glib_borrow(ptr);

    imp.select_all(wrap.unsafe_cast_ref()).into_glib()
}

unsafe extern "C" fn tree_view_select_cursor_parent<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TreeView> = from_glib_borrow(ptr);

    imp.select_cursor_parent(wrap.unsafe_cast_ref()).into_glib()
}

unsafe extern "C" fn tree_view_select_cursor_row<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
    start_editing: glib::ffi::gboolean,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TreeView> = from_glib_borrow(ptr);

    imp.select_cursor_row(wrap.unsafe_cast_ref(), from_glib(start_editing))
        .into_glib()
}

unsafe extern "C" fn tree_view_start_interactive_search<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TreeView> = from_glib_borrow(ptr);

    imp.start_interactive_search(wrap.unsafe_cast_ref())
        .into_glib()
}

unsafe extern "C" fn tree_view_test_collapse_row<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
    iterptr: *mut ffi::GtkTreeIter,
    pathptr: *mut ffi::GtkTreePath,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TreeView> = from_glib_borrow(ptr);
    let path: Borrowed<TreePath> = from_glib_borrow(pathptr);
    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);

    imp.test_collapse_row(wrap.unsafe_cast_ref(), &iter, &path)
        .into_glib()
}

unsafe extern "C" fn tree_view_test_expand_row<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
    iterptr: *mut ffi::GtkTreeIter,
    pathptr: *mut ffi::GtkTreePath,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TreeView> = from_glib_borrow(ptr);
    let path: Borrowed<TreePath> = from_glib_borrow(pathptr);
    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);

    imp.test_expand_row(wrap.unsafe_cast_ref(), &iter, &path)
        .into_glib()
}

unsafe extern "C" fn tree_view_toggle_cursor_row<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TreeView> = from_glib_borrow(ptr);

    imp.toggle_cursor_row(wrap.unsafe_cast_ref()).into_glib()
}

unsafe extern "C" fn tree_view_unselect_all<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TreeView> = from_glib_borrow(ptr);

    imp.unselect_all(wrap.unsafe_cast_ref()).into_glib()
}
