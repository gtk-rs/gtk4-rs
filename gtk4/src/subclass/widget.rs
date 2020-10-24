use gtk_sys;

use std::mem;

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use crate::DirectionType;
use crate::Orientation;
use crate::SizeRequestMode;
use crate::Snapshot;
use crate::StateFlags;
use crate::SystemSetting;
use crate::TextDirection;

use glib::Object;
use Tooltip;
use Widget;

pub trait WidgetImpl: WidgetImplExt + ObjectImpl {
    fn compute_expand(&self, widget: &Self::Type, hexpand_p: &mut bool, vexpand_p: &mut bool) {
        self.parent_compute_expand(widget, hexpand_p, vexpand_p)
    }

    fn contains(&self, widget: &Self::Type, x: f64, y: f64) -> bool {
        self.parent_contains(widget, x, y)
    }

    fn direction_changed(&self, widget: &Self::Type, previous_direction: TextDirection) {
        self.parent_direction_changed(widget, previous_direction)
    }

    fn focus(&self, widget: &Self::Type, direction_type: DirectionType) -> bool {
        self.parent_focus(widget, direction_type)
    }

    fn get_request_mode(&self, widget: &Self::Type) -> SizeRequestMode {
        self.parent_get_request_mode(widget)
    }

    fn grab_focus(&self, widget: &Self::Type) -> bool {
        self.parent_grab_focus(widget)
    }

    fn hide(&self, widget: &Self::Type) {
        self.parent_hide(widget)
    }

    fn keynav_failed(&self, widget: &Self::Type, direction_type: DirectionType) -> bool {
        self.parent_keynav_failed(widget, direction_type)
    }

    fn map(&self, widget: &Self::Type) {
        self.parent_map(widget)
    }

    fn measure(
        &self,
        widget: &Self::Type,
        orientation: Orientation,
        for_size: i32,
    ) -> (i32, i32, i32, i32) {
        self.parent_measure(widget, orientation, for_size)
    }

    fn mnemonic_activate(&self, widget: &Self::Type, group_cycling: bool) -> bool {
        self.parent_mnemonic_activate(widget, group_cycling)
    }

    fn move_focus(&self, widget: &Self::Type, direction_type: DirectionType) {
        self.parent_move_focus(widget, direction_type)
    }

    fn query_tooltip(
        &self,
        widget: &Self::Type,
        x: i32,
        y: i32,
        keyboard_tooltip: bool,
        tooltip: &Tooltip,
    ) -> bool {
        self.parent_query_tooltip(widget, x, y, keyboard_tooltip, tooltip)
    }

    fn realize(&self, widget: &Self::Type) {
        self.parent_realize(widget)
    }

    fn root(&self, widget: &Self::Type) {
        self.parent_root(widget)
    }

    fn set_focus_child(&self, widget: &Self::Type, child: &Widget) {
        self.parent_set_focus_child(widget, child)
    }

    fn show(&self, widget: &Self::Type) {
        self.parent_show(widget)
    }

    fn size_allocate(&self, widget: &Self::Type, width: i32, height: i32, baseline: i32) {
        self.parent_size_allocate(widget, width, height, baseline)
    }

    fn snapshot(&self, widget: &Self::Type, snapshot: &Snapshot) {
        self.parent_snapshot(widget, snapshot)
    }

    fn state_flags_changed(&self, widget: &Self::Type, state_flags: &StateFlags) {
        self.parent_state_flags_changed(widget, state_flags)
    }

    fn system_setting_changed(&self, widget: &Self::Type, settings: &SystemSetting) {
        self.parent_system_setting_changed(widget, settings)
    }

    fn unmap(&self, widget: &Self::Type) {
        self.parent_unmap(widget)
    }

    fn unrealize(&self, widget: &Self::Type) {
        self.parent_unrealize(widget)
    }

    fn unroot(&self, widget: &Self::Type) {
        self.parent_unroot(widget)
    }
}

pub trait WidgetImplExt: ObjectSubclass {
    fn parent_compute_expand(
        &self,
        widget: &Self::Type,
        hexpand_p: &mut bool,
        vexpand_p: &mut bool,
    );
    fn parent_contains(&self, widget: &Self::Type, x: f64, y: f64) -> bool;
    fn parent_direction_changed(&self, widget: &Self::Type, previous_direction: TextDirection);
    fn parent_focus(&self, widget: &Self::Type, direction_type: DirectionType) -> bool;
    fn parent_get_request_mode(&self, widget: &Self::Type) -> SizeRequestMode;
    fn parent_grab_focus(&self, widget: &Self::Type) -> bool;
    fn parent_hide(&self, widget: &Self::Type);
    fn parent_keynav_failed(&self, widget: &Self::Type, direction_type: DirectionType) -> bool;
    fn parent_map(&self, widget: &Self::Type);
    fn parent_measure(
        &self,
        widget: &Self::Type,
        orientation: Orientation,
        for_size: i32,
    ) -> (i32, i32, i32, i32);
    fn parent_mnemonic_activate(&self, widget: &Self::Type, group_cycling: bool) -> bool;
    fn parent_move_focus(&self, widget: &Self::Type, direction_type: DirectionType);
    fn parent_query_tooltip(
        &self,
        widget: &Self::Type,
        x: i32,
        y: i32,
        keyboard_tooltip: bool,
        tooltip: &Tooltip,
    ) -> bool;
    fn parent_realize(&self, widget: &Self::Type);
    fn parent_root(&self, widget: &Self::Type);
    fn parent_set_focus_child(&self, widget: &Self::Type, child: &Widget);
    fn parent_show(&self, widget: &Self::Type);
    fn parent_size_allocate(&self, widget: &Self::Type, width: i32, height: i32, baseline: i32);
    fn parent_snapshot(&self, widget: &Self::Type, snapshot: &Snapshot);
    fn parent_state_flags_changed(&self, widget: &Self::Type, state_flags: &StateFlags);
    fn parent_system_setting_changed(&self, widget: &Self::Type, settings: &SystemSetting);
    fn parent_unmap(&self, widget: &Self::Type);
    fn parent_unrealize(&self, widget: &Self::Type);
    fn parent_unroot(&self, widget: &Self::Type);
}

impl<T: WidgetImpl> WidgetImplExt for T {
    fn parent_compute_expand(
        &self,
        widget: &Self::Type,
        hexpand_p: &mut bool,
        vexpand_p: &mut bool,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .compute_expand
                .expect("No parent class impl for \"compute_expand\"");
            let mut h: i32 = hexpand_p.to_glib();
            let mut v: i32 = vexpand_p.to_glib();
            f(
                widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                &mut h,
                &mut v,
            );
            *hexpand_p = from_glib(h);
            *vexpand_p = from_glib(v);
        }
    }

    fn parent_contains(&self, widget: &Self::Type, x: f64, y: f64) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).contains {
                from_glib(f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0, x, y))
            } else {
                false
            }
        }
    }

    fn parent_direction_changed(&self, widget: &Self::Type, previous_direction: TextDirection) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).direction_changed {
                f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    previous_direction.to_glib(),
                )
            }
        }
    }

    fn parent_focus(&self, widget: &Self::Type, direction_type: DirectionType) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).focus {
                from_glib(f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    direction_type.to_glib(),
                ))
            } else {
                false
            }
        }
    }

    fn parent_get_request_mode(&self, widget: &Self::Type) -> SizeRequestMode {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .get_request_mode
                .expect("No parent class impl for \"get_request_mode\"");
            from_glib(f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0))
        }
    }

    fn parent_grab_focus(&self, widget: &Self::Type) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).grab_focus {
                from_glib(f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0))
            } else {
                false
            }
        }
    }

    fn parent_hide(&self, widget: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).hide {
                f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }

    fn parent_keynav_failed(&self, widget: &Self::Type, direction_type: DirectionType) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).keynav_failed {
                from_glib(f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    direction_type.to_glib(),
                ))
            } else {
                false
            }
        }
    }

    fn parent_map(&self, widget: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).map {
                f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }

    fn parent_measure(
        &self,
        widget: &Self::Type,
        orientation: Orientation,
        for_size: i32,
    ) -> (i32, i32, i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;

            let f = (*parent_class)
                .measure
                .expect("No parent class impl for \"measure\"");

            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            let mut minimum_base_size = mem::MaybeUninit::uninit();
            let mut natural_base_size = mem::MaybeUninit::uninit();

            f(
                widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                orientation.to_glib(),
                for_size,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
                minimum_base_size.as_mut_ptr(),
                natural_base_size.as_mut_ptr(),
            );

            (
                minimum_size.assume_init(),
                natural_size.assume_init(),
                minimum_base_size.assume_init(),
                natural_base_size.assume_init(),
            )
        }
    }

    fn parent_mnemonic_activate(&self, widget: &Self::Type, group_cycling: bool) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).mnemonic_activate {
                from_glib(f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    group_cycling.to_glib(),
                ))
            } else {
                false
            }
        }
    }

    fn parent_move_focus(&self, widget: &Self::Type, direction_type: DirectionType) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).move_focus {
                f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    direction_type.to_glib(),
                )
            }
        }
    }

    fn parent_query_tooltip(
        &self,
        widget: &Self::Type,
        x: i32,
        y: i32,
        keyboard_tooltip: bool,
        tooltip: &Tooltip,
    ) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).query_tooltip {
                from_glib(f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    x,
                    y,
                    keyboard_tooltip.to_glib(),
                    tooltip.to_glib_none().0,
                ))
            } else {
                false
            }
        }
    }

    fn parent_realize(&self, widget: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).realize {
                f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }

    fn parent_root(&self, widget: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).root {
                f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }

    fn parent_set_focus_child(&self, widget: &Self::Type, child: &Widget) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).set_focus_child {
                f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    child.to_glib_none().0,
                )
            }
        }
    }

    fn parent_show(&self, widget: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).show {
                f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }

    fn parent_size_allocate(&self, widget: &Self::Type, width: i32, height: i32, baseline: i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).size_allocate {
                f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    width,
                    height,
                    baseline,
                )
            }
        }
    }

    fn parent_snapshot(&self, widget: &Self::Type, snapshot: &Snapshot) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).snapshot {
                f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    snapshot.to_glib_none().0,
                )
            }
        }
    }

    fn parent_state_flags_changed(&self, widget: &Self::Type, state_flags: &StateFlags) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).state_flags_changed {
                f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    state_flags.to_glib(),
                )
            }
        }
    }

    fn parent_system_setting_changed(&self, widget: &Self::Type, settings: &SystemSetting) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).system_setting_changed {
                f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    settings.to_glib(),
                )
            }
        }
    }

    fn parent_unmap(&self, widget: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).unmap {
                f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }

    fn parent_unrealize(&self, widget: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).unrealize {
                f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }

    fn parent_unroot(&self, widget: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).unroot {
                f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: WidgetImpl> IsSubclassable<T> for Widget {
    fn override_vfuncs(class: &mut ::glib::Class<Self>) {
        <Object as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();

        klass.compute_expand = Some(widget_compute_expand::<T>);
        klass.contains = Some(widget_contains::<T>);
        klass.direction_changed = Some(widget_direction_changed::<T>);
        klass.focus = Some(widget_focus::<T>);
        klass.get_request_mode = Some(widget_get_request_mode::<T>);
        klass.grab_focus = Some(widget_grab_focus::<T>);
        klass.hide = Some(widget_hide::<T>);
        klass.keynav_failed = Some(widget_keynav_failed::<T>);
        klass.map = Some(widget_map::<T>);
        klass.measure = Some(widget_measure::<T>);
        klass.mnemonic_activate = Some(widget_mnemonic_activate::<T>);
        klass.move_focus = Some(widget_move_focus::<T>);
        klass.query_tooltip = Some(widget_query_tooltip::<T>);
        klass.realize = Some(widget_realize::<T>);
        klass.root = Some(widget_root::<T>);
        klass.set_focus_child = Some(widget_set_focus_child::<T>);
        klass.show = Some(widget_show::<T>);
        klass.size_allocate = Some(widget_size_allocate::<T>);
        klass.snapshot = Some(widget_snapshot::<T>);
        klass.state_flags_changed = Some(widget_state_flags_changed::<T>);
        klass.system_setting_changed = Some(widget_system_setting_changed::<T>);
        klass.unmap = Some(widget_unmap::<T>);
        klass.unrealize = Some(widget_unrealize::<T>);
        klass.unroot = Some(widget_unroot::<T>);
    }
}

unsafe extern "C" fn widget_compute_expand<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    hexpand_ptr: *mut glib_sys::gboolean,
    vexpand_ptr: *mut glib_sys::gboolean,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let mut hexpand: bool = from_glib(*hexpand_ptr);
    let mut vexpand: bool = from_glib(*vexpand_ptr);

    imp.compute_expand(wrap.unsafe_cast_ref(), &mut hexpand, &mut vexpand);
    *hexpand_ptr = hexpand.to_glib();
    *vexpand_ptr = vexpand.to_glib();
}

unsafe extern "C" fn widget_contains<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    x: f64,
    y: f64,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.contains(wrap.unsafe_cast_ref(), x, y).to_glib()
}

unsafe extern "C" fn widget_direction_changed<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    direction_ptr: gtk_sys::GtkTextDirection,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let direction_wrap = from_glib(direction_ptr);

    imp.direction_changed(wrap.unsafe_cast_ref(), direction_wrap)
}

unsafe extern "C" fn widget_focus<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    direction_type_ptr: gtk_sys::GtkDirectionType,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let direction_type = from_glib(direction_type_ptr);

    imp.focus(wrap.unsafe_cast_ref(), direction_type).to_glib()
}

unsafe extern "C" fn widget_get_request_mode<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
) -> gtk_sys::GtkSizeRequestMode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.get_request_mode(wrap.unsafe_cast_ref()).to_glib()
}

unsafe extern "C" fn widget_grab_focus<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.grab_focus(wrap.unsafe_cast_ref()).to_glib()
}

unsafe extern "C" fn widget_hide<T: WidgetImpl>(ptr: *mut gtk_sys::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.hide(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn widget_keynav_failed<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    direction_type_ptr: gtk_sys::GtkDirectionType,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let direction_type = from_glib(direction_type_ptr);

    imp.keynav_failed(wrap.unsafe_cast_ref(), direction_type)
        .to_glib()
}

unsafe extern "C" fn widget_map<T: WidgetImpl>(ptr: *mut gtk_sys::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.map(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn widget_measure<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    orientation_ptr: gtk_sys::GtkOrientation,
    for_size: i32,
    min_ptr: *mut libc::c_int,
    nat_ptr: *mut libc::c_int,
    min_base_ptr: *mut libc::c_int,
    nat_base_ptr: *mut libc::c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let orientation = from_glib(orientation_ptr);

    let (min, nat, min_base, nat_base) = imp.measure(wrap.unsafe_cast_ref(), orientation, for_size);
    if !min_ptr.is_null() {
        *min_ptr = min;
    }
    if !nat_ptr.is_null() {
        *nat_ptr = nat;
    }
    if !min_base_ptr.is_null() {
        *min_base_ptr = min_base;
    }
    if !nat_base_ptr.is_null() {
        *nat_base_ptr = nat_base;
    }
}

unsafe extern "C" fn widget_mnemonic_activate<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    group_cycling_ptr: glib_sys::gboolean,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let group_cycling: bool = from_glib(group_cycling_ptr);

    imp.mnemonic_activate(wrap.unsafe_cast_ref(), group_cycling)
        .to_glib()
}

unsafe extern "C" fn widget_move_focus<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    direction_type_ptr: gtk_sys::GtkDirectionType,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let direction_type = from_glib(direction_type_ptr);

    imp.move_focus(wrap.unsafe_cast_ref(), direction_type)
}

unsafe extern "C" fn widget_query_tooltip<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    x: i32,
    y: i32,
    keyboard_tooltip_ptr: glib_sys::gboolean,
    tooltip_ptr: *mut gtk_sys::GtkTooltip,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    let keyboard_tooltip: bool = from_glib(keyboard_tooltip_ptr);
    let tooltip = from_glib_borrow(tooltip_ptr);

    imp.query_tooltip(wrap.unsafe_cast_ref(), x, y, keyboard_tooltip, &tooltip)
        .to_glib()
}

unsafe extern "C" fn widget_realize<T: WidgetImpl>(ptr: *mut gtk_sys::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.realize(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn widget_root<T: WidgetImpl>(ptr: *mut gtk_sys::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.root(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn widget_set_focus_child<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    child_ptr: *mut gtk_sys::GtkWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let child = from_glib_borrow(child_ptr);

    imp.set_focus_child(wrap.unsafe_cast_ref(), &child)
}

unsafe extern "C" fn widget_show<T: WidgetImpl>(ptr: *mut gtk_sys::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.show(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn widget_size_allocate<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    width: i32,
    height: i32,
    baseline: i32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.size_allocate(wrap.unsafe_cast_ref(), width, height, baseline)
}

unsafe extern "C" fn widget_snapshot<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    snapshot_ptr: *mut gtk_sys::GtkSnapshot,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let snapshot = from_glib_borrow(snapshot_ptr);

    imp.snapshot(wrap.unsafe_cast_ref(), &snapshot)
}

unsafe extern "C" fn widget_state_flags_changed<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    state_flags_ptr: gtk_sys::GtkStateFlags,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let state_flags = from_glib(state_flags_ptr);

    imp.state_flags_changed(wrap.unsafe_cast_ref(), &state_flags)
}

unsafe extern "C" fn widget_system_setting_changed<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    settings_ptr: gtk_sys::GtkSystemSetting,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let settings = from_glib(settings_ptr);

    imp.system_setting_changed(wrap.unsafe_cast_ref(), &settings)
}

unsafe extern "C" fn widget_unmap<T: WidgetImpl>(ptr: *mut gtk_sys::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.unmap(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn widget_unrealize<T: WidgetImpl>(ptr: *mut gtk_sys::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.unrealize(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn widget_unroot<T: WidgetImpl>(ptr: *mut gtk_sys::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.unroot(wrap.unsafe_cast_ref())
}
