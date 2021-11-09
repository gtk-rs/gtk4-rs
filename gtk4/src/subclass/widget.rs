// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`Widget`](crate::Widget).

use crate::prelude::*;
use crate::subclass::prelude::*;
use crate::{
    AccessibleRole, BuilderRustScope, BuilderScope, DirectionType, LayoutManager, Orientation,
    Shortcut, SizeRequestMode, Snapshot, StateFlags, SystemSetting, TextDirection, Tooltip, Widget,
};
use glib::subclass::SignalId;
use glib::translate::*;
use glib::{Cast, GString, IsA, Variant};
use std::boxed::Box as Box_;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Default)]
struct Internal {
    pub(crate) actions: HashMap<String, glib::ffi::gpointer>,
    pub(crate) scope: Option<*mut <<BuilderRustScope as glib::object::ObjectSubclassIs>::Subclass as ObjectSubclass>::Instance>,
}
unsafe impl Sync for Internal {}
unsafe impl Send for Internal {}

pub struct WidgetActionIter(*mut ffi::GtkWidgetClass, u32);

pub struct WidgetAction(
    glib::Type,
    GString,
    Option<glib::VariantType>,
    Option<GString>,
);

impl WidgetAction {
    // rustdoc-stripper-ignore-next
    /// The type where the action was defined
    pub fn owner(&self) -> glib::Type {
        self.0
    }

    // rustdoc-stripper-ignore-next
    /// The action name
    pub fn name(&self) -> &str {
        self.1.as_ref()
    }

    // rustdoc-stripper-ignore-next
    /// The action parameter type
    pub fn parameter_type(&self) -> Option<&glib::VariantType> {
        self.2.as_ref()
    }

    // rustdoc-stripper-ignore-next
    /// The action property name
    pub fn property_name(&self) -> Option<&str> {
        self.3.as_ref().map(|s| s.as_ref())
    }
}

impl fmt::Debug for WidgetAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WidgetAction")
            .field("owner", &self.owner())
            .field("name", &self.name())
            .field("parameter_type", &self.parameter_type())
            .field("property_name", &self.property_name())
            .finish()
    }
}

impl Iterator for WidgetActionIter {
    type Item = WidgetAction;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut owner = std::mem::MaybeUninit::uninit();
            let mut action_name_ptr = std::ptr::null();
            let mut parameter_type = std::ptr::null();
            let mut property_name_ptr = std::ptr::null();
            let found: bool = from_glib(ffi::gtk_widget_class_query_action(
                self.0,
                self.1,
                owner.as_mut_ptr(),
                &mut action_name_ptr,
                &mut parameter_type,
                &mut property_name_ptr,
            ));
            if found {
                self.1 += 1;
                let property_name: Option<GString> = from_glib_none(property_name_ptr);
                let action_name: GString = from_glib_none(action_name_ptr);

                Some(WidgetAction(
                    from_glib(owner.assume_init()),
                    action_name,
                    from_glib_none(parameter_type),
                    property_name,
                ))
            } else {
                None
            }
        }
    }
}

pub trait WidgetImpl: WidgetImplExt + ObjectImpl {
    fn compute_expand(&self, widget: &Self::Type, hexpand: &mut bool, vexpand: &mut bool) {
        self.parent_compute_expand(widget, hexpand, vexpand)
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

    #[doc(alias = "get_request_mode")]
    fn request_mode(&self, widget: &Self::Type) -> SizeRequestMode {
        self.parent_request_mode(widget)
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

    fn set_focus_child(&self, widget: &Self::Type, child: Option<&Widget>) {
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
    fn parent_compute_expand(&self, widget: &Self::Type, hexpand: &mut bool, vexpand: &mut bool);
    fn parent_contains(&self, widget: &Self::Type, x: f64, y: f64) -> bool;
    fn parent_direction_changed(&self, widget: &Self::Type, previous_direction: TextDirection);
    fn parent_focus(&self, widget: &Self::Type, direction_type: DirectionType) -> bool;
    fn parent_request_mode(&self, widget: &Self::Type) -> SizeRequestMode;
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
    fn parent_set_focus_child(&self, widget: &Self::Type, child: Option<&Widget>);
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
    fn parent_compute_expand(&self, widget: &Self::Type, hexpand: &mut bool, vexpand: &mut bool) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            let widget = widget.unsafe_cast_ref::<Widget>();
            if let Some(f) = (*parent_class).compute_expand {
                let mut hexpand_glib = hexpand.into_glib();
                let mut vexpand_glib = vexpand.into_glib();
                f(
                    widget.to_glib_none().0,
                    &mut hexpand_glib,
                    &mut vexpand_glib,
                );
                *hexpand = from_glib(hexpand_glib);
                *vexpand = from_glib(vexpand_glib);
            }
        }
    }

    fn parent_contains(&self, widget: &Self::Type, x: f64, y: f64) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).direction_changed {
                f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    previous_direction.into_glib(),
                )
            }
        }
    }

    fn parent_focus(&self, widget: &Self::Type, direction_type: DirectionType) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).focus {
                from_glib(f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    direction_type.into_glib(),
                ))
            } else {
                false
            }
        }
    }

    fn parent_request_mode(&self, widget: &Self::Type) -> SizeRequestMode {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            let f = (*parent_class)
                .get_request_mode
                .expect("No parent class impl for \"get_request_mode\"");
            from_glib(f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0))
        }
    }

    fn parent_grab_focus(&self, widget: &Self::Type) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).hide {
                f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }

    fn parent_keynav_failed(&self, widget: &Self::Type, direction_type: DirectionType) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).keynav_failed {
                from_glib(f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    direction_type.into_glib(),
                ))
            } else {
                false
            }
        }
    }

    fn parent_map(&self, widget: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;

            let f = (*parent_class)
                .measure
                .expect("No parent class impl for \"measure\"");

            let mut min = 0;
            let mut nat = 0;
            let mut min_base = -1;
            let mut nat_base = -1;
            f(
                widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                orientation.into_glib(),
                for_size,
                &mut min,
                &mut nat,
                &mut min_base,
                &mut nat_base,
            );
            (min, nat, min_base, nat_base)
        }
    }

    fn parent_mnemonic_activate(&self, widget: &Self::Type, group_cycling: bool) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).mnemonic_activate {
                from_glib(f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    group_cycling.into_glib(),
                ))
            } else {
                false
            }
        }
    }

    fn parent_move_focus(&self, widget: &Self::Type, direction_type: DirectionType) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).move_focus {
                f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    direction_type.into_glib(),
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).query_tooltip {
                from_glib(f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    x,
                    y,
                    keyboard_tooltip.into_glib(),
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).realize {
                f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }

    fn parent_root(&self, widget: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).root {
                f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }

    fn parent_set_focus_child(&self, widget: &Self::Type, child: Option<&Widget>) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).show {
                f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }

    fn parent_size_allocate(&self, widget: &Self::Type, width: i32, height: i32, baseline: i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).state_flags_changed {
                f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    state_flags.into_glib(),
                )
            }
        }
    }

    fn parent_system_setting_changed(&self, widget: &Self::Type, settings: &SystemSetting) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).system_setting_changed {
                f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    settings.into_glib(),
                )
            }
        }
    }

    fn parent_unmap(&self, widget: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).unmap {
                f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }

    fn parent_unrealize(&self, widget: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).unrealize {
                f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }

    fn parent_unroot(&self, widget: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).unroot {
                f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: WidgetImpl> IsSubclassable<T> for Widget {
    fn class_init(class: &mut ::glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        assert!(
            crate::rt::is_initialized(),
            "GTK has to be initialized first"
        );

        let klass = class.as_mut();
        unsafe {
            let mut data = T::type_data();
            let data = data.as_mut();
            // Used to store actions for `install_action` and `rust_builder_scope`
            data.set_class_data(<T as ObjectSubclassType>::type_(), Internal::default());
        }

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
    ptr: *mut ffi::GtkWidget,
    hexpand_ptr: *mut glib::ffi::gboolean,
    vexpand_ptr: *mut glib::ffi::gboolean,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    let widget = wrap.unsafe_cast_ref::<Widget>();
    let mut hexpand: bool = if widget.is_hexpand_set() {
        widget.hexpands()
    } else {
        from_glib(*hexpand_ptr)
    };
    let mut vexpand: bool = if widget.is_vexpand_set() {
        widget.vexpands()
    } else {
        from_glib(*vexpand_ptr)
    };

    imp.compute_expand(wrap.unsafe_cast_ref(), &mut hexpand, &mut vexpand);

    *hexpand_ptr = hexpand.into_glib();
    *vexpand_ptr = vexpand.into_glib();
}

unsafe extern "C" fn widget_contains<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    x: f64,
    y: f64,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.contains(wrap.unsafe_cast_ref(), x, y).into_glib()
}

unsafe extern "C" fn widget_direction_changed<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    direction_ptr: ffi::GtkTextDirection,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let direction_wrap = from_glib(direction_ptr);

    imp.direction_changed(wrap.unsafe_cast_ref(), direction_wrap)
}

unsafe extern "C" fn widget_focus<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    direction_type_ptr: ffi::GtkDirectionType,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let direction_type = from_glib(direction_type_ptr);

    imp.focus(wrap.unsafe_cast_ref(), direction_type)
        .into_glib()
}

unsafe extern "C" fn widget_get_request_mode<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
) -> ffi::GtkSizeRequestMode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.request_mode(wrap.unsafe_cast_ref()).into_glib()
}

unsafe extern "C" fn widget_grab_focus<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.grab_focus(wrap.unsafe_cast_ref()).into_glib()
}

unsafe extern "C" fn widget_hide<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.hide(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn widget_keynav_failed<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    direction_type_ptr: ffi::GtkDirectionType,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let direction_type = from_glib(direction_type_ptr);

    imp.keynav_failed(wrap.unsafe_cast_ref(), direction_type)
        .into_glib()
}

unsafe extern "C" fn widget_map<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.map(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn widget_measure<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    orientation_ptr: ffi::GtkOrientation,
    for_size: i32,
    min_ptr: *mut libc::c_int,
    nat_ptr: *mut libc::c_int,
    min_base_ptr: *mut libc::c_int,
    nat_base_ptr: *mut libc::c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
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
    ptr: *mut ffi::GtkWidget,
    group_cycling_ptr: glib::ffi::gboolean,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let group_cycling: bool = from_glib(group_cycling_ptr);

    imp.mnemonic_activate(wrap.unsafe_cast_ref(), group_cycling)
        .into_glib()
}

unsafe extern "C" fn widget_move_focus<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    direction_type_ptr: ffi::GtkDirectionType,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let direction_type = from_glib(direction_type_ptr);

    imp.move_focus(wrap.unsafe_cast_ref(), direction_type)
}

unsafe extern "C" fn widget_query_tooltip<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    x: i32,
    y: i32,
    keyboard_tooltip_ptr: glib::ffi::gboolean,
    tooltip_ptr: *mut ffi::GtkTooltip,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    let keyboard_tooltip: bool = from_glib(keyboard_tooltip_ptr);
    let tooltip = from_glib_borrow(tooltip_ptr);

    imp.query_tooltip(wrap.unsafe_cast_ref(), x, y, keyboard_tooltip, &tooltip)
        .into_glib()
}

unsafe extern "C" fn widget_realize<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.realize(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn widget_root<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.root(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn widget_set_focus_child<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    child_ptr: *mut ffi::GtkWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let child: Borrowed<Option<Widget>> = from_glib_borrow(child_ptr);

    imp.set_focus_child(wrap.unsafe_cast_ref(), child.as_ref().as_ref())
}

unsafe extern "C" fn widget_show<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.show(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn widget_size_allocate<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    width: i32,
    height: i32,
    baseline: i32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.size_allocate(wrap.unsafe_cast_ref(), width, height, baseline)
}

unsafe extern "C" fn widget_snapshot<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    snapshot_ptr: *mut ffi::GtkSnapshot,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let snapshot = from_glib_borrow(snapshot_ptr);

    imp.snapshot(wrap.unsafe_cast_ref(), &snapshot)
}

unsafe extern "C" fn widget_state_flags_changed<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    state_flags_ptr: ffi::GtkStateFlags,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let state_flags = from_glib(state_flags_ptr);

    imp.state_flags_changed(wrap.unsafe_cast_ref(), &state_flags)
}

unsafe extern "C" fn widget_system_setting_changed<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    settings_ptr: ffi::GtkSystemSetting,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let settings = from_glib(settings_ptr);

    imp.system_setting_changed(wrap.unsafe_cast_ref(), &settings)
}

unsafe extern "C" fn widget_unmap<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.unmap(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn widget_unrealize<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.unrealize(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn widget_unroot<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.unroot(wrap.unsafe_cast_ref())
}

#[allow(clippy::missing_safety_doc)]
pub unsafe trait WidgetClassSubclassExt: ClassStruct {
    #[doc(alias = "gtk_widget_class_set_template")]
    fn set_template_bytes(&mut self, template: &glib::Bytes) {
        unsafe {
            let widget_class = self as *mut _ as *mut ffi::GtkWidgetClass;
            ffi::gtk_widget_class_set_template(widget_class, template.to_glib_none().0);
        }
    }

    fn set_template(&mut self, template: &[u8]) {
        let template_bytes = glib::Bytes::from(template);
        self.set_template_bytes(&template_bytes);
    }

    fn set_template_static(&mut self, template: &'static [u8]) {
        let template_bytes = glib::Bytes::from_static(template);
        self.set_template_bytes(&template_bytes);
    }

    #[doc(alias = "gtk_widget_class_set_template_from_resource")]
    fn set_template_from_resource(&mut self, resource_name: &str) {
        unsafe {
            let widget_class = self as *mut _ as *mut ffi::GtkWidgetClass;
            ffi::gtk_widget_class_set_template_from_resource(
                widget_class,
                resource_name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_widget_class_install_action")]
    fn install_action<
        F: Fn(&<<Self as ClassStruct>::Type as ObjectSubclass>::Type, &str, Option<&Variant>)
            + 'static,
    >(
        &mut self,
        action_name: &str,
        parameter_type: Option<&str>,
        activate: F,
    ) {
        unsafe {
            // We store the activate callbacks in a HashMap<action_name, activate>
            // so that we can retrieve f later on the activate_trampoline call
            let mut data = <Self::Type as ObjectSubclassType>::type_data();
            let data = data.as_mut();

            let f: Box_<F> = Box_::new(activate);

            let internal = data
                .class_data_mut::<Internal>(<Self::Type as ObjectSubclassType>::type_())
                .expect("Something bad happened at class_init, the internal class_data is missing");
            let callback_ptr = Box_::into_raw(f) as glib::ffi::gpointer;
            internal
                .actions
                .insert(action_name.to_string(), callback_ptr);

            unsafe extern "C" fn activate_trampoline<F, S>(
                this: *mut ffi::GtkWidget,
                action_name: *const libc::c_char,
                parameter: *mut glib::ffi::GVariant,
            ) where
                S: ClassStruct,
                <S as ClassStruct>::Type: ObjectSubclass,
                F: Fn(&<<S as ClassStruct>::Type as ObjectSubclass>::Type, &str, Option<&Variant>)
                    + 'static,
            {
                let action_name = GString::from_glib_borrow(action_name);

                let data = <S::Type as ObjectSubclassType>::type_data();
                let internal = data
                    .as_ref()
                    .class_data::<Internal>(<S::Type as ObjectSubclassType>::type_())
                    .unwrap();
                let activate_callback = *internal
                    .actions
                    .get(&action_name.to_string())
                    .unwrap_or_else(|| {
                        panic!("Action name '{}' was not found", action_name.as_str());
                    });

                let widget = Widget::from_glib_borrow(this);

                let f: &F = &*(activate_callback as *const F);
                f(
                    widget.unsafe_cast_ref(),
                    &action_name,
                    Option::<Variant>::from_glib_borrow(parameter)
                        .as_ref()
                        .as_ref(),
                )
            }
            let widget_class = self as *mut _ as *mut ffi::GtkWidgetClass;
            let callback = activate_trampoline::<F, Self>;
            ffi::gtk_widget_class_install_action(
                widget_class,
                action_name.to_glib_none().0,
                parameter_type.to_glib_none().0,
                Some(callback),
            );
        }
    }

    #[doc(alias = "gtk_widget_class_query_action")]
    fn query_action(&self) -> WidgetActionIter {
        let widget_class = self as *const _ as *mut ffi::GtkWidgetClass;
        WidgetActionIter(widget_class, 0)
    }

    #[doc(alias = "gtk_widget_class_set_template_scope")]
    fn set_template_scope<S: IsA<BuilderScope>>(&mut self, scope: &S) {
        unsafe {
            let widget_class = self as *mut _ as *mut ffi::GtkWidgetClass;
            ffi::gtk_widget_class_set_template_scope(widget_class, scope.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_widget_class_bind_template_child_full")]
    fn bind_template_child(&mut self, name: &str) {
        unsafe {
            let widget_class = self as *mut _ as *mut ffi::GtkWidgetClass;
            ffi::gtk_widget_class_bind_template_child_full(
                widget_class,
                name.to_glib_none().0,
                false as glib::ffi::gboolean,
                0,
            );
        }
    }

    #[doc(alias = "gtk_widget_class_add_binding")]
    fn add_binding<
        F: Fn(&<<Self as ClassStruct>::Type as ObjectSubclass>::Type, Option<&Variant>) -> bool
            + 'static,
    >(
        &mut self,
        keyval: gdk::keys::Key,
        mods: gdk::ModifierType,
        callback: F,
        arguments: Option<&glib::Variant>,
    ) {
        let shortcut = crate::Shortcut::new(
            Some(&crate::KeyvalTrigger::new(keyval, mods)),
            Some(&crate::CallbackAction::new(move |widget, args| -> bool {
                unsafe { callback(widget.unsafe_cast_ref(), args) }
            })),
        );
        shortcut.set_arguments(arguments);
        self.add_shortcut(&shortcut);
    }

    #[doc(alias = "gtk_widget_class_add_binding_action")]
    fn add_binding_action(
        &mut self,
        keyval: gdk::keys::Key,
        mods: gdk::ModifierType,
        action_name: &str,
        arguments: Option<&glib::Variant>,
    ) {
        let shortcut = crate::Shortcut::new(
            Some(&crate::KeyvalTrigger::new(keyval, mods)),
            Some(&crate::NamedAction::new(action_name)),
        );
        shortcut.set_arguments(arguments);
        self.add_shortcut(&shortcut);
    }

    #[doc(alias = "gtk_widget_class_add_binding_signal")]
    fn add_binding_signal(
        &mut self,
        keyval: gdk::keys::Key,
        mods: gdk::ModifierType,
        signal_name: &str,
        arguments: Option<&glib::Variant>,
    ) {
        let type_ = <Self::Type as ObjectSubclassType>::type_();
        assert!(
            SignalId::lookup(signal_name, type_).is_some(),
            "Signal '{}' doesn't exists for type '{}'",
            signal_name,
            type_
        );

        let shortcut = crate::Shortcut::new(
            Some(&crate::KeyvalTrigger::new(keyval, mods)),
            Some(&crate::SignalAction::new(signal_name)),
        );
        shortcut.set_arguments(arguments);
        self.add_shortcut(&shortcut);
    }

    #[doc(alias = "gtk_widget_class_add_shortcut")]
    fn add_shortcut(&mut self, shortcut: &Shortcut) {
        unsafe {
            let widget_class = self as *mut _ as *mut ffi::GtkWidgetClass;
            ffi::gtk_widget_class_add_shortcut(widget_class, shortcut.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_widget_class_install_property_action")]
    fn install_property_action(&mut self, action_name: &str, property_name: &str) {
        unsafe {
            let widget_class = self as *mut _ as *mut ffi::GtkWidgetClass;
            ffi::gtk_widget_class_install_property_action(
                widget_class,
                action_name.to_glib_none().0,
                property_name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_widget_class_get_activate_signal")]
    #[doc(alias = "get_activate_signal")]
    fn activate_signal(&self) -> Option<SignalId> {
        unsafe {
            let widget_class = self as *const _ as *mut ffi::GtkWidgetClass;
            let signal_id = ffi::gtk_widget_class_get_activate_signal(widget_class);
            if signal_id == 0 {
                None
            } else {
                Some(from_glib(signal_id))
            }
        }
    }

    #[doc(alias = "gtk_widget_class_set_activate_signal")]
    fn set_activate_signal(&mut self, signal_id: SignalId) {
        unsafe {
            let widget_class = self as *mut _ as *mut ffi::GtkWidgetClass;
            ffi::gtk_widget_class_set_activate_signal(widget_class, signal_id.into_glib())
        }
    }

    #[doc(alias = "gtk_widget_class_set_activate_signal_from_name")]
    fn set_activate_signal_from_name(&mut self, signal_name: &str) {
        let type_ = <Self::Type as ObjectSubclassType>::type_();
        assert!(
            SignalId::lookup(signal_name, type_).is_some(),
            "Signal '{}' doesn't exists for type '{}'",
            signal_name,
            type_
        );

        unsafe {
            let widget_class = self as *mut _ as *mut ffi::GtkWidgetClass;
            ffi::gtk_widget_class_set_activate_signal_from_name(
                widget_class,
                signal_name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_widget_class_set_layout_manager_type")]
    fn set_layout_manager_type<T: IsA<LayoutManager>>(&mut self) {
        unsafe {
            let widget_class = self as *mut _ as *mut ffi::GtkWidgetClass;
            ffi::gtk_widget_class_set_layout_manager_type(
                widget_class,
                T::static_type().into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_widget_class_get_layout_manager_type")]
    #[doc(alias = "get_layout_manager_type")]
    fn layout_manager_type(&self) -> glib::Type {
        unsafe {
            let widget_class = self as *const _ as *mut ffi::GtkWidgetClass;
            from_glib(ffi::gtk_widget_class_get_layout_manager_type(widget_class))
        }
    }

    #[doc(alias = "gtk_widget_class_set_css_name")]
    fn set_css_name(&mut self, name: &str) {
        unsafe {
            let widget_class = self as *mut _ as *mut ffi::GtkWidgetClass;
            ffi::gtk_widget_class_set_css_name(widget_class, name.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_widget_class_get_css_name")]
    #[doc(alias = "get_css_name")]
    fn css_name(&self) -> glib::GString {
        unsafe {
            let widget_class = self as *const _ as *mut ffi::GtkWidgetClass;
            from_glib_none(ffi::gtk_widget_class_get_css_name(widget_class))
        }
    }

    #[doc(alias = "gtk_widget_class_set_accessible_role")]
    fn set_accessible_role(&mut self, role: AccessibleRole) {
        unsafe {
            let widget_class = self as *mut _ as *mut ffi::GtkWidgetClass;
            ffi::gtk_widget_class_set_accessible_role(widget_class, role.into_glib());
        }
    }

    #[doc(alias = "gtk_widget_class_get_accessible_role")]
    #[doc(alias = "get_accessible_role")]
    fn accessible_role(&self) -> AccessibleRole {
        unsafe {
            let widget_class = self as *const _ as *mut ffi::GtkWidgetClass;
            from_glib(ffi::gtk_widget_class_get_accessible_role(widget_class))
        }
    }

    #[allow(clippy::missing_safety_doc)]
    #[doc(alias = "gtk_widget_class_bind_template_child_full")]
    unsafe fn bind_template_child_with_offset<T>(
        &mut self,
        name: &str,
        internal: bool,
        offset: field_offset::FieldOffset<Self::Type, TemplateChild<T>>,
    ) where
        T: ObjectType + FromGlibPtrNone<*mut <T as ObjectType>::GlibType>,
    {
        let widget_class = self as *mut _ as *mut ffi::GtkWidgetClass;
        let private_offset = <Self::Type as ObjectSubclassType>::type_data()
            .as_ref()
            .private_offset;
        ffi::gtk_widget_class_bind_template_child_full(
            widget_class,
            name.to_glib_none().0,
            internal.into_glib(),
            private_offset + (offset.get_byte_offset() as isize),
        )
    }

    fn rust_template_scope(&mut self) -> BuilderRustScope {
        assert_initialized_main_thread!();
        let mut data = <Self::Type as ObjectSubclassType>::type_data();
        let internal = unsafe {
            data.as_mut()
                .class_data_mut::<Internal>(<Self::Type as ObjectSubclassType>::type_())
                .expect("Something bad happened at class_init, the internal class_data is missing")
        };
        let scope = internal.scope.get_or_insert_with(|| {
            let scope = BuilderRustScope::new();
            self.set_template_scope(&scope);
            scope.to_glib_full()
        });
        unsafe { from_glib_none(*scope) }
    }
}

unsafe impl<T: ClassStruct> WidgetClassSubclassExt for T where T::Type: WidgetImpl {}

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct TemplateChild<T>
where
    T: ObjectType + FromGlibPtrNone<*mut <T as ObjectType>::GlibType>,
{
    ptr: *mut <T as ObjectType>::GlibType,
}

impl<T> Default for TemplateChild<T>
where
    T: ObjectType + FromGlibPtrNone<*mut <T as ObjectType>::GlibType>,
{
    fn default() -> Self {
        T::static_type();

        Self {
            ptr: std::ptr::null_mut(),
        }
    }
}

impl<T> std::ops::Deref for TemplateChild<T>
where
    T: ObjectType + FromGlibPtrNone<*mut <T as ObjectType>::GlibType>,
{
    type Target = T;

    // rustdoc-stripper-ignore-next
    /// # Safety
    ///
    /// Since the template child may not be properly bound,
    /// this cast is potentially dangerous if, for example,
    /// the template child isn't bound or is of the wrong type.
    /// The caller is responsible for ensuring that the template
    /// child is bound and of the right type.
    fn deref(&self) -> &Self::Target {
        unsafe {
            assert!(!self.ptr.is_null());
            &*(&self.ptr as *const _ as *const T)
        }
    }
}

impl<T> TemplateChild<T>
where
    T: ObjectType + FromGlibPtrNone<*mut <T as ObjectType>::GlibType>,
{
    #[track_caller]
    pub fn get(&self) -> T {
        self.try_get()
            .expect("Failed to retrieve template child. Please check that it has been bound.")
    }

    // rustdoc-stripper-ignore-next
    /// Determines if the child has been bound. This is primarily
    /// useful for implementing the [`Buildable`][`crate::Buildable`] interface.
    pub fn is_bound(&self) -> bool {
        !self.ptr.is_null()
    }

    // rustdoc-stripper-ignore-next
    /// Returns Some(child) if the widget has been bound.
    pub fn try_get(&self) -> Option<T> {
        unsafe { Option::<T>::from_glib_none(self.ptr) }
    }
}

pub trait CompositeTemplate: WidgetImpl {
    fn bind_template(klass: &mut Self::Class);
}

pub type TemplateCallback = (&'static str, fn(&[glib::Value]) -> Option<glib::Value>);

pub trait CompositeTemplateCallbacks {
    const CALLBACKS: &'static [TemplateCallback];

    // rustdoc-stripper-ignore-next
    /// Binds the template callbacks from this type into the default template scope for `klass`.
    fn bind_template_callbacks<T: WidgetClassSubclassExt>(klass: &mut T) {
        Self::add_callbacks_to_scope(&klass.rust_template_scope());
    }
    // rustdoc-stripper-ignore-next
    /// Binds the template callbacks from this type into the default template scope for `klass`,
    /// prepending `prefix` to each callback name.
    fn bind_template_callbacks_prefixed<T: WidgetClassSubclassExt>(klass: &mut T, prefix: &str) {
        Self::add_callbacks_to_scope_prefixed(&klass.rust_template_scope(), prefix);
    }
    // rustdoc-stripper-ignore-next
    /// Binds the template callbacks from this type into `scope`.
    fn add_callbacks_to_scope(scope: &BuilderRustScope) {
        for (name, func) in Self::CALLBACKS {
            scope.add_callback(*name, func);
        }
    }
    // rustdoc-stripper-ignore-next
    /// Binds the template callbacks from this type into `scope`, prepending `prefix` to each
    /// callback name.
    fn add_callbacks_to_scope_prefixed(scope: &BuilderRustScope, prefix: &str) {
        for (name, func) in Self::CALLBACKS {
            scope.add_callback(format!("{}{}", prefix, name), func);
        }
    }
}
