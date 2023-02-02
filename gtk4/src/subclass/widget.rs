// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`Widget`](crate::Widget).

use crate::{
    prelude::*, subclass::prelude::*, AccessibleRole, BuilderRustScope, BuilderScope,
    DirectionType, LayoutManager, Orientation, Shortcut, SizeRequestMode, Snapshot, StateFlags,
    SystemSetting, TextDirection, Tooltip, Widget,
};
use glib::{subclass::SignalId, translate::*, GString, Variant};

use std::{boxed::Box as Box_, collections::HashMap, fmt, future::Future};

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

impl std::iter::FusedIterator for WidgetActionIter {}

pub trait WidgetImpl: WidgetImplExt + ObjectImpl {
    fn compute_expand(&self, hexpand: &mut bool, vexpand: &mut bool) {
        self.parent_compute_expand(hexpand, vexpand)
    }

    fn contains(&self, x: f64, y: f64) -> bool {
        self.parent_contains(x, y)
    }

    fn direction_changed(&self, previous_direction: TextDirection) {
        self.parent_direction_changed(previous_direction)
    }

    fn focus(&self, direction_type: DirectionType) -> bool {
        self.parent_focus(direction_type)
    }

    #[doc(alias = "get_request_mode")]
    fn request_mode(&self) -> SizeRequestMode {
        self.parent_request_mode()
    }

    fn grab_focus(&self) -> bool {
        self.parent_grab_focus()
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    fn hide(&self) {
        self.parent_hide()
    }

    fn keynav_failed(&self, direction_type: DirectionType) -> bool {
        self.parent_keynav_failed(direction_type)
    }

    fn map(&self) {
        self.parent_map()
    }

    fn measure(&self, orientation: Orientation, for_size: i32) -> (i32, i32, i32, i32) {
        self.parent_measure(orientation, for_size)
    }

    fn mnemonic_activate(&self, group_cycling: bool) -> bool {
        self.parent_mnemonic_activate(group_cycling)
    }

    fn move_focus(&self, direction_type: DirectionType) {
        self.parent_move_focus(direction_type)
    }

    fn query_tooltip(&self, x: i32, y: i32, keyboard_tooltip: bool, tooltip: &Tooltip) -> bool {
        self.parent_query_tooltip(x, y, keyboard_tooltip, tooltip)
    }

    fn realize(&self) {
        self.parent_realize()
    }

    fn root(&self) {
        self.parent_root()
    }

    fn set_focus_child(&self, child: Option<&Widget>) {
        self.parent_set_focus_child(child)
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    fn show(&self) {
        self.parent_show()
    }

    fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        self.parent_size_allocate(width, height, baseline)
    }

    fn snapshot(&self, snapshot: &Snapshot) {
        self.parent_snapshot(snapshot)
    }

    fn state_flags_changed(&self, state_flags: &StateFlags) {
        self.parent_state_flags_changed(state_flags)
    }

    fn system_setting_changed(&self, settings: &SystemSetting) {
        self.parent_system_setting_changed(settings)
    }

    fn unmap(&self) {
        self.parent_unmap()
    }

    fn unrealize(&self) {
        self.parent_unrealize()
    }

    fn unroot(&self) {
        self.parent_unroot()
    }
}

pub trait WidgetImplExt: ObjectSubclass {
    fn parent_compute_expand(&self, hexpand: &mut bool, vexpand: &mut bool);
    fn parent_contains(&self, x: f64, y: f64) -> bool;
    fn parent_direction_changed(&self, previous_direction: TextDirection);
    fn parent_focus(&self, direction_type: DirectionType) -> bool;
    fn parent_request_mode(&self) -> SizeRequestMode;
    fn parent_grab_focus(&self) -> bool;
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    fn parent_hide(&self);
    fn parent_keynav_failed(&self, direction_type: DirectionType) -> bool;
    fn parent_map(&self);
    fn parent_measure(&self, orientation: Orientation, for_size: i32) -> (i32, i32, i32, i32);
    fn parent_mnemonic_activate(&self, group_cycling: bool) -> bool;
    fn parent_move_focus(&self, direction_type: DirectionType);
    fn parent_query_tooltip(
        &self,
        x: i32,
        y: i32,
        keyboard_tooltip: bool,
        tooltip: &Tooltip,
    ) -> bool;
    fn parent_realize(&self);
    fn parent_root(&self);
    fn parent_set_focus_child(&self, child: Option<&Widget>);
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    fn parent_show(&self);
    fn parent_size_allocate(&self, width: i32, height: i32, baseline: i32);
    fn parent_snapshot(&self, snapshot: &Snapshot);
    fn parent_state_flags_changed(&self, state_flags: &StateFlags);
    fn parent_system_setting_changed(&self, settings: &SystemSetting);
    fn parent_unmap(&self);
    fn parent_unrealize(&self);
    fn parent_unroot(&self);
}

impl<T: WidgetImpl> WidgetImplExt for T {
    fn parent_compute_expand(&self, hexpand: &mut bool, vexpand: &mut bool) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).compute_expand {
                let mut hexpand_glib = hexpand.into_glib();
                let mut vexpand_glib = vexpand.into_glib();
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    &mut hexpand_glib,
                    &mut vexpand_glib,
                );
                *hexpand = from_glib(hexpand_glib);
                *vexpand = from_glib(vexpand_glib);
            }
        }
    }

    fn parent_contains(&self, x: f64, y: f64) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).contains {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    x,
                    y,
                ))
            } else {
                false
            }
        }
    }

    fn parent_direction_changed(&self, previous_direction: TextDirection) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).direction_changed {
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    previous_direction.into_glib(),
                )
            }
        }
    }

    fn parent_focus(&self, direction_type: DirectionType) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).focus {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    direction_type.into_glib(),
                ))
            } else {
                false
            }
        }
    }

    fn parent_request_mode(&self) -> SizeRequestMode {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            let f = (*parent_class)
                .get_request_mode
                .expect("No parent class impl for \"get_request_mode\"");
            from_glib(f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0))
        }
    }

    fn parent_grab_focus(&self) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).grab_focus {
                from_glib(f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0))
            } else {
                false
            }
        }
    }

    fn parent_hide(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).hide {
                f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }

    fn parent_keynav_failed(&self, direction_type: DirectionType) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).keynav_failed {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    direction_type.into_glib(),
                ))
            } else {
                false
            }
        }
    }

    fn parent_map(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).map {
                f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }

    fn parent_measure(&self, orientation: Orientation, for_size: i32) -> (i32, i32, i32, i32) {
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
                self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
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

    fn parent_mnemonic_activate(&self, group_cycling: bool) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).mnemonic_activate {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    group_cycling.into_glib(),
                ))
            } else {
                false
            }
        }
    }

    fn parent_move_focus(&self, direction_type: DirectionType) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).move_focus {
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    direction_type.into_glib(),
                )
            }
        }
    }

    fn parent_query_tooltip(
        &self,
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
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
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

    fn parent_realize(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).realize {
                f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }

    fn parent_root(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).root {
                f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }

    fn parent_set_focus_child(&self, child: Option<&Widget>) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).set_focus_child {
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    child.to_glib_none().0,
                )
            }
        }
    }

    fn parent_show(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).show {
                f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }

    fn parent_size_allocate(&self, width: i32, height: i32, baseline: i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).size_allocate {
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    width,
                    height,
                    baseline,
                )
            }
        }
    }

    fn parent_snapshot(&self, snapshot: &Snapshot) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).snapshot {
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    snapshot.to_glib_none().0,
                )
            }
        }
    }

    fn parent_state_flags_changed(&self, state_flags: &StateFlags) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).state_flags_changed {
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    state_flags.into_glib(),
                )
            }
        }
    }

    fn parent_system_setting_changed(&self, settings: &SystemSetting) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).system_setting_changed {
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    settings.into_glib(),
                )
            }
        }
    }

    fn parent_unmap(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).unmap {
                f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }

    fn parent_unrealize(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).unrealize {
                f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }

    fn parent_unroot(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).unroot {
                f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: WidgetImpl> IsSubclassable<T> for Widget {
    fn class_init(class: &mut ::glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        assert_initialized_main_thread!();

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
    let imp = instance.imp();

    let widget = imp.obj();
    let widget = widget.unsafe_cast_ref::<Widget>();
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

    imp.compute_expand(&mut hexpand, &mut vexpand);

    *hexpand_ptr = hexpand.into_glib();
    *vexpand_ptr = vexpand.into_glib();
}

unsafe extern "C" fn widget_contains<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    x: f64,
    y: f64,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.contains(x, y).into_glib()
}

unsafe extern "C" fn widget_direction_changed<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    direction_ptr: ffi::GtkTextDirection,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let direction_wrap = from_glib(direction_ptr);

    imp.direction_changed(direction_wrap)
}

unsafe extern "C" fn widget_focus<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    direction_type_ptr: ffi::GtkDirectionType,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let direction_type = from_glib(direction_type_ptr);

    imp.focus(direction_type).into_glib()
}

unsafe extern "C" fn widget_get_request_mode<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
) -> ffi::GtkSizeRequestMode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.request_mode().into_glib()
}

unsafe extern "C" fn widget_grab_focus<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.grab_focus().into_glib()
}

unsafe extern "C" fn widget_hide<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.hide()
}

unsafe extern "C" fn widget_keynav_failed<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    direction_type_ptr: ffi::GtkDirectionType,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let direction_type = from_glib(direction_type_ptr);

    imp.keynav_failed(direction_type).into_glib()
}

unsafe extern "C" fn widget_map<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.map()
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
    let imp = instance.imp();
    let orientation = from_glib(orientation_ptr);
    let (min, nat, min_base, nat_base) = imp.measure(orientation, for_size);
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
    let imp = instance.imp();
    let group_cycling: bool = from_glib(group_cycling_ptr);

    imp.mnemonic_activate(group_cycling).into_glib()
}

unsafe extern "C" fn widget_move_focus<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    direction_type_ptr: ffi::GtkDirectionType,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let direction_type = from_glib(direction_type_ptr);

    imp.move_focus(direction_type)
}

unsafe extern "C" fn widget_query_tooltip<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    x: i32,
    y: i32,
    keyboard_tooltip_ptr: glib::ffi::gboolean,
    tooltip_ptr: *mut ffi::GtkTooltip,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let keyboard_tooltip: bool = from_glib(keyboard_tooltip_ptr);
    let tooltip = from_glib_borrow(tooltip_ptr);

    imp.query_tooltip(x, y, keyboard_tooltip, &tooltip)
        .into_glib()
}

unsafe extern "C" fn widget_realize<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.realize()
}

unsafe extern "C" fn widget_root<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.root()
}

unsafe extern "C" fn widget_set_focus_child<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    child_ptr: *mut ffi::GtkWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let child: Borrowed<Option<Widget>> = from_glib_borrow(child_ptr);

    imp.set_focus_child(child.as_ref().as_ref())
}

unsafe extern "C" fn widget_show<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.show()
}

unsafe extern "C" fn widget_size_allocate<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    width: i32,
    height: i32,
    baseline: i32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.size_allocate(width, height, baseline)
}

unsafe extern "C" fn widget_snapshot<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    snapshot_ptr: *mut ffi::GtkSnapshot,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let snapshot = from_glib_borrow(snapshot_ptr);

    imp.snapshot(&snapshot)
}

unsafe extern "C" fn widget_state_flags_changed<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    state_flags_ptr: ffi::GtkStateFlags,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let state_flags = from_glib(state_flags_ptr);

    imp.state_flags_changed(&state_flags)
}

unsafe extern "C" fn widget_system_setting_changed<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    settings_ptr: ffi::GtkSystemSetting,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let settings = from_glib(settings_ptr);

    imp.system_setting_changed(&settings)
}

unsafe extern "C" fn widget_unmap<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.unmap()
}

unsafe extern "C" fn widget_unrealize<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.unrealize()
}

unsafe extern "C" fn widget_unroot<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.unroot()
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

    fn install_action_async<Fut, F>(
        &mut self,
        action_name: &str,
        parameter_type: Option<&str>,
        activate: F,
    ) where
        F: Fn(
                <<Self as ClassStruct>::Type as ObjectSubclass>::Type,
                String,
                Option<Variant>,
            ) -> Fut
            + 'static
            + Clone,
        Fut: Future<Output = ()>,
    {
        self.install_action(
            action_name,
            parameter_type,
            move |this, action_name, parameter_type| {
                let ctx = glib::MainContext::default();
                let action_name = action_name.to_owned();
                let parameter_type = parameter_type.map(ToOwned::to_owned);
                ctx.spawn_local(glib::clone!(@strong this, @strong action_name, @strong parameter_type, @strong activate => async move {
                    activate(this, action_name, parameter_type).await;
                }));
            },
        );
    }

    #[doc(alias = "gtk_widget_class_install_action")]
    fn install_action<F>(&mut self, action_name: &str, parameter_type: Option<&str>, activate: F)
    where
        F: Fn(&<<Self as ClassStruct>::Type as ObjectSubclass>::Type, &str, Option<&Variant>)
            + 'static,
    {
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

    #[doc(alias = "gtk_widget_class_add_binding")]
    fn add_binding<
        F: Fn(&<<Self as ClassStruct>::Type as ObjectSubclass>::Type, Option<&Variant>) -> bool
            + 'static,
    >(
        &mut self,
        keyval: gdk::Key,
        mods: gdk::ModifierType,
        callback: F,
        arguments: Option<&glib::Variant>,
    ) {
        let shortcut = crate::Shortcut::new(
            Some(crate::KeyvalTrigger::new(keyval, mods)),
            Some(crate::CallbackAction::new(move |widget, args| -> bool {
                unsafe { callback(widget.unsafe_cast_ref(), args) }
            })),
        );
        shortcut.set_arguments(arguments);
        self.add_shortcut(&shortcut);
    }

    #[doc(alias = "gtk_widget_class_add_binding_action")]
    fn add_binding_action(
        &mut self,
        keyval: gdk::Key,
        mods: gdk::ModifierType,
        action_name: &str,
        arguments: Option<&glib::Variant>,
    ) {
        let shortcut = crate::Shortcut::new(
            Some(crate::KeyvalTrigger::new(keyval, mods)),
            Some(crate::NamedAction::new(action_name)),
        );
        shortcut.set_arguments(arguments);
        self.add_shortcut(&shortcut);
    }

    #[doc(alias = "gtk_widget_class_add_binding_signal")]
    fn add_binding_signal(
        &mut self,
        keyval: gdk::Key,
        mods: gdk::ModifierType,
        signal_name: &str,
        arguments: Option<&glib::Variant>,
    ) {
        let type_ = <Self::Type as ObjectSubclassType>::type_();
        assert!(
            SignalId::lookup(signal_name, type_).is_some(),
            "Signal '{signal_name}' doesn't exists for type '{type_}'",
        );

        let shortcut = crate::Shortcut::new(
            Some(crate::KeyvalTrigger::new(keyval, mods)),
            Some(crate::SignalAction::new(signal_name)),
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
            "Signal '{signal_name}' doesn't exists for type '{type_}'",
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
            .impl_offset();
        ffi::gtk_widget_class_bind_template_child_full(
            widget_class,
            name.to_glib_none().0,
            internal.into_glib(),
            private_offset + (offset.get_byte_offset() as isize),
        )
    }

    fn rust_template_scope(&mut self) -> BuilderRustScope {
        assert_initialized_main_thread!();
        unsafe {
            let mut data = <Self::Type as ObjectSubclassType>::type_data();
            let internal = data
                .as_mut()
                .class_data_mut::<Internal>(<Self::Type as ObjectSubclassType>::type_())
                .expect("Something bad happened at class_init, the internal class_data is missing");
            let scope = internal.scope.get_or_insert_with(|| {
                let scope = BuilderRustScope::new();
                self.set_template_scope(&scope);
                scope.into_glib_ptr()
            });
            from_glib_none(*scope)
        }
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

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            if !self.is_bound() {
                let name = Self::name();
                panic!("Failed to retrieve template child. Please check that all fields of type `{name}` have been bound and have a #[template_child] attribute.");
            }
            &*(&self.ptr as *const _ as *const T)
        }
    }
}

impl<T> TemplateChild<T>
where
    T: ObjectType + FromGlibPtrNone<*mut <T as ObjectType>::GlibType>,
{
    pub(crate) fn name<'a>() -> &'a str {
        T::static_type().name()
    }

    #[track_caller]
    pub fn get(&self) -> T {
        self.try_get()
            .unwrap_or_else(|| {
                let name = Self::name();
                panic!("Failed to retrieve template child. Please check that all fields of type `{name}` have been bound and have a #[template_child] attribute.");
            })
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

// rustdoc-stripper-ignore-next
/// A trait for setting up template children inside
/// [`class_init`](glib::subclass::types::ObjectSubclass::class_init). This trait is implemented
/// automatically by the [`CompositeTemplate`](crate::CompositeTemplate) macro.
pub trait CompositeTemplate: WidgetImpl {
    fn bind_template(klass: &mut Self::Class);
    fn check_template_children(widget: &<Self as ObjectSubclass>::Type);
}

// rustdoc-stripper-ignore-next
/// An extension trait for [`ClassStruct`](glib::subclass::types::ClassStruct) types to allow
/// binding a composite template directly on `self`. This is a convenience wrapper around
/// the [`CompositeTemplate`] trait.
pub trait CompositeTemplateClass {
    // rustdoc-stripper-ignore-next
    /// Binds the template callbacks from this type into the default template scope for `self`.
    fn bind_template(&mut self);
}

impl<T, U> CompositeTemplateClass for T
where
    T: ClassStruct<Type = U>,
    U: ObjectSubclass<Class = T> + CompositeTemplate,
{
    fn bind_template(&mut self) {
        <U as CompositeTemplate>::bind_template(self);
    }
}

pub type TemplateCallback = (&'static str, fn(&[glib::Value]) -> Option<glib::Value>);

// rustdoc-stripper-ignore-next
/// A trait for setting up template callbacks inside
/// [`class_init`](glib::subclass::types::ObjectSubclass::class_init). This trait is implemented
/// automatically by the [`template_callbacks`](crate::template_callbacks) macro.
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
            scope.add_callback(format!("{prefix}{name}"), func);
        }
    }
}

// rustdoc-stripper-ignore-next
/// An extension trait for [`ClassStruct`](glib::subclass::types::ClassStruct) types to allow
/// binding private template callbacks directly on `self`. This is a convenience wrapper around
/// the [`CompositeTemplateCallbacks`] trait.
pub trait CompositeTemplateCallbacksClass {
    // rustdoc-stripper-ignore-next
    /// Binds the template callbacks from the subclass type into the default template scope for `self`.
    fn bind_template_callbacks(&mut self);
}

impl<T, U> CompositeTemplateCallbacksClass for T
where
    T: ClassStruct<Type = U> + WidgetClassSubclassExt,
    U: ObjectSubclass<Class = T> + CompositeTemplateCallbacks,
{
    fn bind_template_callbacks(&mut self) {
        <U as CompositeTemplateCallbacks>::bind_template_callbacks(self);
    }
}

// rustdoc-stripper-ignore-next
/// An extension trait for [`ClassStruct`](glib::subclass::types::ClassStruct) types to allow
/// binding the instance template callbacks directly on `self`. This is a convenience wrapper around
/// the [`CompositeTemplateCallbacks`] trait.
pub trait CompositeTemplateInstanceCallbacksClass {
    // rustdoc-stripper-ignore-next
    /// Binds the template callbacks from the instance type into the default template scope for `self`.
    fn bind_template_instance_callbacks(&mut self);
}

impl<T, U, V> CompositeTemplateInstanceCallbacksClass for T
where
    T: ClassStruct<Type = U> + WidgetClassSubclassExt,
    U: ObjectSubclass<Class = T, Type = V>,
    V: CompositeTemplateCallbacks,
{
    fn bind_template_instance_callbacks(&mut self) {
        <V as CompositeTemplateCallbacks>::bind_template_callbacks(self);
    }
}

pub trait CompositeTemplateInitializingExt {
    fn init_template(&self);
}

impl<T> CompositeTemplateInitializingExt for glib::subclass::InitializingObject<T>
where
    T: WidgetImpl + CompositeTemplate,
    <T as ObjectSubclass>::Type: IsA<Widget>,
{
    fn init_template(&self) {
        unsafe {
            let widget = self
                .as_ref()
                .unsafe_cast_ref::<<T as ObjectSubclass>::Type>();
            ffi::gtk_widget_init_template(widget.as_ref().to_glib_none().0);

            <T as CompositeTemplate>::check_template_children(widget);
        }
    }
}

pub trait CompositeTemplateDisposeExt {
    #[cfg(any(feature = "v4_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_8")))]
    fn dispose_template(&self);
}

impl<T> CompositeTemplateDisposeExt for T
where
    T: WidgetImpl + CompositeTemplate,
    <T as ObjectSubclass>::Type: IsA<Widget>,
{
    #[cfg(any(feature = "v4_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_8")))]
    fn dispose_template(&self) {
        unsafe {
            ffi::gtk_widget_dispose_template(
                self.obj().upcast_ref::<Widget>().to_glib_none().0,
                <T as ObjectSubclass>::Type::static_type().into_glib(),
            );
        }
    }
}
