use gtk_sys;

use glib::translate::*;

use glib::subclass::prelude::*;

use super::widget::WidgetImpl;
use Container;
use ContainerClass;
use Widget;
use WidgetPath;

pub trait ContainerImpl: ContainerImplExt + WidgetImpl + 'static {
    fn add(&self, container: &Container, widget: &Widget) {
        self.parent_add(container, widget)
    }

    fn remove(&self, container: &Container, widget: &Widget) {
        self.parent_remove(container, widget)
    }

    fn check_resize(&self, container: &Container) {
        self.parent_check_resize(container)
    }

    fn set_focus_child(&self, container: &Container, widget: &Widget) {
        self.parent_set_focus_child(container, widget)
    }

    fn child_type(&self, container: &Container) -> glib::Type {
        self.parent_child_type(container)
    }

    fn get_path_for_child(&self, container: &Container, widget: &Widget) -> WidgetPath {
        self.parent_get_path_for_child(container, widget)
    }
}

pub trait ContainerImplExt {
    fn parent_add(&self, container: &Container, widget: &Widget);
    fn parent_remove(&self, container: &Container, widget: &Widget);
    fn parent_check_resize(&self, container: &Container);
    fn parent_set_focus_child(&self, container: &Container, widget: &Widget);
    fn parent_child_type(&self, container: &Container) -> glib::Type;
    fn parent_get_path_for_child(&self, container: &Container, widget: &Widget) -> WidgetPath;
}

impl<T: ContainerImpl + ObjectImpl> ContainerImplExt for T {
    fn parent_add(&self, container: &Container, widget: &Widget) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkContainerClass;
            let f = (*parent_class)
                .add
                .expect("No parent class impl for \"add\"");
            f(container.to_glib_none().0, widget.to_glib_none().0)
        }
    }

    fn parent_remove(&self, container: &Container, widget: &Widget) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkContainerClass;
            let f = (*parent_class)
                .remove
                .expect("No parent class impl for \"remove\"");
            f(container.to_glib_none().0, widget.to_glib_none().0)
        }
    }

    fn parent_check_resize(&self, container: &Container) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkContainerClass;
            let f = (*parent_class)
                .check_resize
                .expect("No parent class impl for \"check_resize\"");
            f(container.to_glib_none().0)
        }
    }

    fn parent_set_focus_child(&self, container: &Container, widget: &Widget) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkContainerClass;
            let f = (*parent_class)
                .set_focus_child
                .expect("No parent class impl for \"set_focus_child\"");
            f(container.to_glib_none().0, widget.to_glib_none().0)
        }
    }

    fn parent_child_type(&self, container: &Container) -> glib::Type {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkContainerClass;
            let f = (*parent_class)
                .child_type
                .expect("No parent class impl for \"child_type\"");
            from_glib(f(container.to_glib_none().0))
        }
    }

    fn parent_get_path_for_child(&self, container: &Container, widget: &Widget) -> WidgetPath {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkContainerClass;
            let f = (*parent_class)
                .get_path_for_child
                .expect("No parent class impl for \"get_path_for_child\"");
            from_glib_borrow(f(container.to_glib_none().0, widget.to_glib_none().0))
        }
    }
}

unsafe impl<T: ObjectSubclass + ContainerImpl> IsSubclassable<T> for ContainerClass {
    fn override_vfuncs(&mut self) {
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gtk_sys::GtkContainerClass);
            klass.add = Some(container_add::<T>);
            klass.remove = Some(container_remove::<T>);
            klass.check_resize = Some(container_check_resize::<T>);
            klass.set_focus_child = Some(container_set_focus_child::<T>);
            klass.child_type = Some(container_child_type::<T>);
            klass.get_path_for_child = Some(container_get_path_for_child::<T>);
        }
    }
}

unsafe extern "C" fn container_add<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkContainer,
    wdgtptr: *mut gtk_sys::GtkWidget,
) where
    T: ContainerImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Container = from_glib_borrow(ptr);
    let widget: Widget = from_glib_borrow(wdgtptr);

    imp.add(&wrap, &widget)
}

unsafe extern "C" fn container_remove<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkContainer,
    wdgtptr: *mut gtk_sys::GtkWidget,
) where
    T: ContainerImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Container = from_glib_borrow(ptr);
    let widget: Widget = from_glib_borrow(wdgtptr);

    imp.remove(&wrap, &widget)
}

unsafe extern "C" fn container_check_resize<T: ObjectSubclass>(ptr: *mut gtk_sys::GtkContainer)
where
    T: ContainerImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Container = from_glib_borrow(ptr);

    imp.check_resize(&wrap)
}

unsafe extern "C" fn container_set_focus_child<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkContainer,
    wdgtptr: *mut gtk_sys::GtkWidget,
) where
    T: ContainerImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Container = from_glib_borrow(ptr);
    let widget: Widget = from_glib_borrow(wdgtptr);

    imp.set_focus_child(&wrap, &widget)
}

unsafe extern "C" fn container_child_type<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkContainer,
) -> glib_sys::GType
where
    T: ContainerImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Container = from_glib_borrow(ptr);

    imp.child_type(&wrap).to_glib()
}

unsafe extern "C" fn container_get_path_for_child<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkContainer,
    wdgtptr: *mut gtk_sys::GtkWidget,
) -> *mut gtk_sys::GtkWidgetPath
where
    T: ContainerImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Container = from_glib_borrow(ptr);
    let widget: Widget = from_glib_borrow(wdgtptr);

    imp.get_path_for_child(&wrap, &widget).to_glib_none().0
}
