/// Public API of `BaseButton`.
mod imp;

use std::{future::Future, pin::Pin};

use gtk::{glib, prelude::*, subclass::prelude::*};

/// Type alias for pinned boxed futures that output `T`.
pub type PinnedFuture<T> = Pin<Box<dyn Future<Output = T>>>;

glib::wrapper! {
    /// Public type for the `BaseButton` instances.
    pub struct BaseButton(ObjectSubclass<imp::BaseButton>)
        @extends gtk::Widget, gtk::Button;
}

impl Default for BaseButton {
    fn default() -> Self {
        glib::Object::new()
    }
}

/// Public trait that implements our functions for everything that derives from
/// `BaseButton`.
///
/// These are public methods that can be called on any instance.
pub trait BaseButtonExt: IsA<BaseButton> {
    /// Caller for a non-virtual method on `BaseButton`.
    ///
    /// This directly calls the method inside the implementation module.
    fn non_virtual_method(&self) {
        self.upcast_ref::<BaseButton>().imp().non_virtual_method();
    }

    /// Caller for a virtual method on `BaseButton`.
    ///
    /// This retrieves this instance's class and calls the function pointer in
    /// it.
    #[allow(dead_code)]
    fn sync_method(&self, extra_text: Option<&str>) {
        let obj = self.upcast_ref::<BaseButton>();
        (obj.class().as_ref().sync_method)(obj, extra_text);
    }

    /// Caller for an async virtual method on `BaseButton`.
    ///
    /// This retrieves this instance's class and calls the function pointer in
    /// it.
    ///
    /// Once async functions in traits are supported this should become one.
    fn async_method(&self) -> PinnedFuture<Result<(), glib::Error>> {
        let obj = self.upcast_ref::<BaseButton>();
        (obj.class().as_ref().async_method)(obj)
    }
}

impl<O: IsA<BaseButton>> BaseButtonExt for O {}

/// The `BaseButtonImpl` trait that each derived implementation struct has to
/// implement.
///
/// See `derived_button/imp.rs` for how to override virtual methods.
pub trait BaseButtonImpl: ButtonImpl + ObjectSubclass<Type: IsA<BaseButton>> {
    /// Default implementation of a virtual method.
    ///
    /// This always calls into the implementation of the parent class so that if
    /// the subclass does not explicitly implement it, the behaviour of its
    /// parent class will be preserved.
    fn sync_method(&self, extra_text: Option<&str>) {
        self.parent_sync_method(extra_text)
    }

    /// Default implementation of an async virtual method.
    ///
    /// This always calls into the implementation of the parent class so that if
    /// the subclass does not explicitly implement it, the behaviour of its
    /// parent class will be preserved.
    fn async_method(&self) -> PinnedFuture<Result<(), glib::Error>> {
        self.parent_async_method()
    }
}

/// Public trait with "protected" methods for everything implementing
/// `BaseButton`.
///
/// These are supposed to be called only from inside implementations of
/// `BaseButton` subclasses.
pub trait BaseButtonImplExt: BaseButtonImpl {
    /// Retrieves the parent class' implementation of the virtual method and
    /// calls it.
    fn parent_sync_method(&self, extra_text: Option<&str>) {
        unsafe {
            let data = Self::type_data();
            let parent_class = &*(data.as_ref().parent_class() as *mut Class);
            (parent_class.sync_method)(self.obj().unsafe_cast_ref(), extra_text)
        }
    }

    /// Retrieves the parent class' implementation of the async virtual method
    /// and calls it.
    fn parent_async_method(&self) -> PinnedFuture<Result<(), glib::Error>> {
        unsafe {
            let data = Self::type_data();
            let parent_class = &*(data.as_ref().parent_class() as *mut Class);
            (parent_class.async_method)(self.obj().unsafe_cast_ref())
        }
    }
}

impl<T: BaseButtonImpl> BaseButtonImplExt for T {}

/// This allows to implement subclasses of `BaseButton`.
unsafe impl<T: BaseButtonImpl> IsSubclassable<T> for BaseButton {
    /// Called whenever the class of a `BaseButton` subclass is initialized,
    /// i.e. usually right before the first instance of it is created.
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class.upcast_ref_mut());

        // Override the virtual method function pointers to call directly into the
        // `BaseButtonImpl` of the subclass.
        //
        // Note that this is only called for actual subclasses and not `BaseButton`
        // itself: `BaseButton` does not implement `BaseButtonImpl` and handles
        // this inside `ObjectSubclass::class_init()` for providing the default
        // implementation of the virtual methods.
        let klass = class.as_mut();
        klass.sync_method = |obj, extra_text| unsafe {
            let imp = obj.unsafe_cast_ref::<T::Type>().imp();
            imp.sync_method(extra_text)
        };
        klass.async_method = |obj| unsafe {
            let imp = obj.unsafe_cast_ref::<T::Type>().imp();
            imp.async_method()
        };
    }
}

/// GObject class struct with the function pointers for the virtual methods.
///
/// This must be `#[repr(C)]`.
#[repr(C)]
pub struct Class {
    pub parent_class: gtk::ffi::GtkButtonClass,

    // If these functions are meant to be called from C, you need to make these functions
    // `unsafe extern "C" fn` & use FFI-safe types (usually raw pointers).
    pub sync_method: fn(&BaseButton, extra_text: Option<&str>),
    pub async_method: fn(&BaseButton) -> PinnedFuture<Result<(), glib::Error>>,
}

/// Make it possible to use this struct as class struct in an `ObjectSubclass`
/// trait implementation.
///
/// This is `unsafe` to enforce that the struct is `#[repr(C)]`.
unsafe impl ClassStruct for Class {
    type Type = imp::BaseButton;
}

/// Deref directly to the parent class' class struct.
impl std::ops::Deref for Class {
    type Target = glib::Class<<<Self as ClassStruct>::Type as ObjectSubclass>::ParentType>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(&self.parent_class as *const _ as *const _) }
    }
}
