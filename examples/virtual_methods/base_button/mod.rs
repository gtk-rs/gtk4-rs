/// Public part of the BaseButton
mod imp;

pub use self::imp::PinnedFuture;
use gtk::{
    glib::{self, subclass::prelude::*},
    prelude::*,
    subclass::prelude::*,
};

glib::wrapper! {
    pub struct BaseButton(ObjectSubclass<imp::BaseButton>)
        @extends gtk::Widget, gtk::Button;
}

impl BaseButton {
    pub fn new() -> Self {
        glib::Object::new_default()
    }
}

impl Default for BaseButton {
    fn default() -> Self {
        Self::new()
    }
}

/// Public trait that implements our functions for everything that derives from BaseButton
pub trait BaseButtonExt {
    fn sync_method(&self, extra_text: Option<String>);
    fn async_method(&self) -> PinnedFuture;
}

/// We call into imp::BaseButton_$method_name for each function. These will retrieve the
/// correct class (the base class for the BaseButton or the derived class for DerivedButton)
/// and call the correct implementation of the function.
impl<O: IsA<BaseButton>> BaseButtonExt for O {
    fn sync_method(&self, extra_text: Option<String>) {
        imp::base_button_sync_method(self.upcast_ref::<BaseButton>(), extra_text)
    }

    fn async_method(&self) -> PinnedFuture {
        imp::base_button_async_method(self.upcast_ref::<BaseButton>())
    }
}

/// The BaseButtonImpl that each derived private struct has to implement. See derived_button/imp.rs for how
/// to override functions.
pub trait BaseButtonImpl: ButtonImpl + ObjectImpl + 'static {
    fn sync_method(&self, obj: &BaseButton, extra_text: Option<String>) {
        self.parent_sync_method(obj, extra_text)
    }

    fn async_method(&self, obj: &BaseButton) -> PinnedFuture {
        self.parent_async_method(obj)
    }
}

pub trait BaseButtonImplExt: ObjectSubclass {
    fn parent_sync_method(&self, obj: &BaseButton, extra_text: Option<String>);
    fn parent_async_method(&self, obj: &BaseButton) -> PinnedFuture;
}

impl<T: BaseButtonImpl> BaseButtonImplExt for T {
    fn parent_sync_method(&self, obj: &BaseButton, extra_text: Option<String>) {
        unsafe {
            let data = Self::type_data();
            let parent_class = &*(data.as_ref().parent_class() as *mut imp::BaseButtonClass);
            (parent_class.sync_method)(obj, extra_text)
        }
    }

    fn parent_async_method(&self, obj: &BaseButton) -> PinnedFuture {
        unsafe {
            let data = Self::type_data();
            let parent_class = &*(data.as_ref().parent_class() as *mut imp::BaseButtonClass);
            (parent_class.async_method)(obj)
        }
    }
}

/// Make the BaseButton subclassable
unsafe impl<T: BaseButtonImpl> IsSubclassable<T> for BaseButton {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class.upcast_ref_mut());

        let klass = class.as_mut();
        klass.sync_method = sync_method_trampoline::<T>;
        klass.async_method = async_method_trampoline::<T>;
    }
}

// Virtual method implementation trampolines
fn sync_method_trampoline<T>(this: &BaseButton, extra_text: Option<String>)
where
    T: ObjectSubclass + BaseButtonImpl,
{
    let imp = this.dynamic_cast_ref::<T::Type>().unwrap().imp();
    imp.sync_method(this, extra_text)
}

fn async_method_trampoline<T>(this: &BaseButton) -> PinnedFuture
where
    T: ObjectSubclass + BaseButtonImpl,
{
    let imp = this.dynamic_cast_ref::<T::Type>().unwrap().imp();
    imp.async_method(this)
}
