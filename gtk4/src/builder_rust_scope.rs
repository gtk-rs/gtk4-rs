// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::BuilderScope;
use std::rc::Rc;

glib::wrapper! {
    // rustdoc-stripper-ignore-next
    /// An implementation of [`BuilderScope`](crate::BuilderScope) that can bind Rust callbacks.
    ///
    /// ```no_run
    /// # use gtk4 as gtk;
    /// use gtk::prelude::*;
    /// use gtk::subclass::prelude::*;
    ///
    /// # fn main() {
    /// let builder = gtk::Builder::new();
    /// let scope = gtk::BuilderRustScope::new();
    /// scope.add_callback("print_label", |values| {
    ///     let button = values[1].get::<gtk::Button>().unwrap();
    ///     println!("{}", button.label().unwrap().as_str());
    ///     None
    /// });
    /// builder.set_scope(Some(&scope));
    /// # }
    /// ```
    pub struct BuilderRustScope(ObjectSubclass<imp::BuilderRustScope>)
        @implements BuilderScope;
}

impl Default for BuilderRustScope {
    fn default() -> Self {
        Self::new()
    }
}

impl BuilderRustScope {
    pub fn new() -> Self {
        glib::Object::new(&[]).unwrap()
    }
    // rustdoc-stripper-ignore-next
    /// Adds a Rust callback to the scope with the given `name`. The callback can then be accessed
    /// from a [`Builder`](crate::Builder) by referring to it in the builder XML, or by using
    /// [`Builder::create_closure`](crate::Builder::create_closure).
    pub fn add_callback<N: Into<String>, F: Fn(&[glib::Value]) -> Option<glib::Value> + 'static>(
        &self,
        name: N,
        callback: F,
    ) {
        imp::BuilderRustScope::from_instance(self)
            .callbacks
            .borrow_mut()
            .insert(name.into(), Rc::new(callback));
    }
}

mod imp {
    use crate::prelude::*;
    use crate::subclass::prelude::*;
    use crate::{Builder, BuilderClosureFlags, BuilderError, BuilderScope};
    use glib::translate::*;
    use glib::{Closure, RustClosure};
    use std::{cell::RefCell, collections::HashMap, rc::Rc};

    type Callback = dyn Fn(&[glib::Value]) -> Option<glib::Value>;

    #[derive(Default)]
    pub struct BuilderRustScope {
        pub callbacks: RefCell<HashMap<String, Rc<Callback>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BuilderRustScope {
        const NAME: &'static str = "GtkBuilderRustScope";
        type Type = super::BuilderRustScope;
        type Interfaces = (BuilderScope,);
    }

    impl ObjectImpl for BuilderRustScope {}

    impl BuilderScopeImpl for BuilderRustScope {
        fn create_closure(
            &self,
            _builder_scope: &Self::Type,
            builder: &Builder,
            function_name: &str,
            flags: BuilderClosureFlags,
            object: Option<&glib::Object>,
        ) -> Result<Closure, glib::Error> {
            self.callbacks
                .borrow()
                .get(function_name)
                .ok_or_else(|| {
                    glib::Error::new(
                        BuilderError::InvalidFunction,
                        &format!("No function named `{}`", function_name),
                    )
                })
                .map(|callback| {
                    let callback = callback.clone();
                    let swapped = flags.contains(BuilderClosureFlags::SWAPPED);
                    let object = object.cloned().or_else(|| builder.current_object());
                    if let Some(object) = object {
                        let object_weak = object.downgrade();
                        let closure = if swapped {
                            RustClosure::new_local(move |args| {
                                let mut args = args.to_owned();
                                let object = object_weak.upgrade().expect(
                                    "Internal error: Object destroyed before closure invalidated",
                                );
                                let obj_v = unsafe {
                                    let mut v = glib::Value::uninitialized();
                                    glib::gobject_ffi::g_value_init(
                                        v.to_glib_none_mut().0,
                                        object.type_().into_glib(),
                                    );
                                    glib::gobject_ffi::g_value_set_object(
                                        v.to_glib_none_mut().0,
                                        object.as_object_ref().to_glib_none().0,
                                    );
                                    v
                                };
                                args.push(obj_v);
                                let len = args.len();
                                args.swap(0, len - 1);
                                callback(&args)
                            })
                        } else {
                            RustClosure::new_local(move |args| {
                                let mut args = args.to_owned();
                                let object = object_weak
                                    .upgrade()
                                    .expect("Object destroyed before closure invalidated");
                                let obj_v = unsafe {
                                    let mut v = glib::Value::uninitialized();
                                    glib::gobject_ffi::g_value_init(
                                        v.to_glib_none_mut().0,
                                        object.type_().into_glib(),
                                    );
                                    glib::gobject_ffi::g_value_set_object(
                                        v.to_glib_none_mut().0,
                                        object.as_object_ref().to_glib_none().0,
                                    );
                                    v
                                };
                                args.push(obj_v);
                                callback(&args)
                            })
                        };
                        object.watch_closure(closure.as_ref());
                        closure.as_ref().clone()
                    } else {
                        if swapped {
                            RustClosure::new_local(move |args| {
                                let mut args = args.to_owned();
                                if !args.is_empty() {
                                    let len = args.len();
                                    args.swap(0, len - 1);
                                }
                                callback(&args)
                            })
                        } else {
                            RustClosure::new_local(move |args| callback(args))
                        }
                        .as_ref()
                        .clone()
                    }
                })
        }
    }
}
