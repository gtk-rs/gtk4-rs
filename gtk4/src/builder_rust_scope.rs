// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{subclass::prelude::*, BuilderCScope, BuilderScope};
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
    ///
    /// // can also be used with template_callbacks
    /// pub struct Callbacks {}
    /// #[gtk::template_callbacks]
    /// impl Callbacks {
    ///     #[template_callback]
    ///     fn button_clicked(button: &gtk::Button) {
    ///         button.set_label("Clicked");
    ///     }
    /// }
    /// Callbacks::add_callbacks_to_scope(&scope);
    /// # }
    /// ```
    pub struct BuilderRustScope(ObjectSubclass<imp::BuilderRustScope>)
        @extends BuilderCScope,
        @implements BuilderScope;
}

impl Default for BuilderRustScope {
    fn default() -> Self {
        Self::new()
    }
}

impl BuilderRustScope {
    pub fn new() -> Self {
        glib::Object::new()
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
        self.imp()
            .callbacks
            .borrow_mut()
            .insert(name.into(), Rc::new(callback));
    }
}

mod imp {
    use super::*;
    use crate::{prelude::*, Builder, BuilderClosureFlags, BuilderError};
    use glib::{translate::*, Closure, RustClosure};
    use std::{cell::RefCell, collections::HashMap};

    type Callback = dyn Fn(&[glib::Value]) -> Option<glib::Value>;

    #[derive(Default)]
    pub struct BuilderRustScope {
        pub callbacks: RefCell<HashMap<String, Rc<Callback>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BuilderRustScope {
        const NAME: &'static str = "GtkBuilderRustScope";
        type Type = super::BuilderRustScope;
        type ParentType = BuilderCScope;
        type Interfaces = (BuilderScope,);
    }

    impl ObjectImpl for BuilderRustScope {}
    impl BuilderCScopeImpl for BuilderRustScope {}

    impl BuilderScopeImpl for BuilderRustScope {
        fn type_from_function(&self, _builder: &Builder, _function_name: &str) -> glib::Type {
            // Override the implementation provided by BuilderCScope and default to the interface
            // default implementation
            glib::Type::INVALID
        }

        fn create_closure(
            &self,
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
                        &format!("No function named `{function_name}`"),
                    )
                })
                .map(|callback| {
                    let callback = callback.clone();
                    let swapped = flags.contains(BuilderClosureFlags::SWAPPED);
                    let object = object.cloned().or_else(|| builder.current_object());
                    if let Some(object) = object {
                        // passing a pointer here is safe as `watch_closure` ensures we have a ref
                        let object_ptr = object.as_ptr();
                        let closure = if swapped {
                            RustClosure::new_local(move |args| {
                                let mut args = args.to_owned();
                                let obj_v = unsafe {
                                    let object: Borrowed<glib::Object> =
                                        from_glib_borrow(object_ptr);
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
                                let obj_v = unsafe {
                                    let object: Borrowed<glib::Object> =
                                        from_glib_borrow(object_ptr);
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

#[cfg(test)]
mod tests {
    use super::BuilderRustScope;
    use crate::{self as gtk4, prelude::*, subclass::prelude::*, Builder};

    const SIGNAL_XML: &str = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <interface>
      <object class="GtkButton" id="button">
        <property name="label">Hello World</property>
        <signal name="clicked" handler="button_clicked"/>
      </object>
    </interface>
    "##;

    #[crate::test]
    fn test_rust_builder_scope_signal_handler() {
        use crate::Button;

        pub struct Callbacks {}
        #[template_callbacks]
        impl Callbacks {
            #[template_callback]
            fn button_clicked(button: &Button) {
                skip_assert_initialized!();
                assert_eq!(button.label().unwrap().as_str(), "Hello World");
                button.set_label("Clicked");
            }
        }

        let builder = Builder::new();
        let scope = BuilderRustScope::new();
        Callbacks::add_callbacks_to_scope(&scope);
        builder.set_scope(Some(&scope));
        builder.add_from_string(SIGNAL_XML).unwrap();
        let button = builder.object::<Button>("button").unwrap();
        button.emit_clicked();
        assert_eq!(button.label().unwrap().as_str(), "Clicked");
    }

    const CLOSURE_XML: &str = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <interface>
      <object class="GtkEntry" id="entry_a"/>
      <object class="GtkEntry" id="entry_b">
        <binding name="text">
          <closure type="gchararray" function="string_uppercase">
            <lookup type="GtkEntry" name="text">entry_a</lookup>
          </closure>
        </binding>
      </object>
    </interface>
    "##;

    #[crate::test]
    fn test_rust_builder_scope_closure() {
        use crate::Entry;

        pub struct StringCallbacks {}
        #[template_callbacks]
        impl StringCallbacks {
            #[template_callback(function)]
            fn uppercase(s: &str) -> String {
                skip_assert_initialized!();
                s.to_uppercase()
            }
        }

        let builder = Builder::new();
        let scope = BuilderRustScope::new();
        StringCallbacks::add_callbacks_to_scope_prefixed(&scope, "string_");
        builder.set_scope(Some(&scope));
        builder.add_from_string(CLOSURE_XML).unwrap();
        let entry_a = builder.object::<Entry>("entry_a").unwrap();
        let entry_b = builder.object::<Entry>("entry_b").unwrap();
        entry_a.set_text("Hello World");
        assert_eq!(entry_b.text().as_str(), "HELLO WORLD");
    }

    #[crate::test]
    #[should_panic(
        expected = "Closure returned a value of type guint64 but caller expected gchararray"
    )]
    fn test_rust_builder_scope_closure_return_mismatch() {
        use crate::Entry;

        pub struct StringCallbacks {}
        #[template_callbacks]
        impl StringCallbacks {
            #[template_callback(function, name = "uppercase")]
            fn to_u64(s: &str) -> u64 {
                skip_assert_initialized!();
                s.parse().unwrap_or(0)
            }
        }

        let builder = Builder::new();
        let scope = BuilderRustScope::new();
        StringCallbacks::add_callbacks_to_scope_prefixed(&scope, "string_");
        builder.set_scope(Some(&scope));
        builder.add_from_string(CLOSURE_XML).unwrap();
        let entry_a = builder.object::<Entry>("entry_a").unwrap();
        entry_a.set_text("Hello World");
    }

    const DISPOSE_XML: &str = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <interface>
      <object class="MyObject" id="obj">
        <signal name="destroyed" handler="my_object_destroyed" object="obj" swapped="true" />
      </object>
    </interface>
    "##;

    #[crate::test]
    fn test_rust_builder_scope_object_during_dispose() {
        use glib::subclass::Signal;
        use once_cell::sync::Lazy;
        use std::{cell::Cell, rc::Rc};

        #[derive(Debug, Default)]
        pub struct MyObjectPrivate {
            counter: Rc<Cell<u64>>,
        }
        #[glib::object_subclass]
        impl ObjectSubclass for MyObjectPrivate {
            const NAME: &'static str = "MyObject";
            type Type = MyObject;
        }
        impl ObjectImpl for MyObjectPrivate {
            fn signals() -> &'static [Signal] {
                static SIGNALS: Lazy<Vec<Signal>> =
                    Lazy::new(|| vec![Signal::builder("destroyed").build()]);
                SIGNALS.as_ref()
            }
            fn dispose(&self) {
                self.obj().emit_by_name::<()>("destroyed", &[]);
            }
        }
        glib::wrapper! {
            pub struct MyObject(ObjectSubclass<MyObjectPrivate>);
        }
        #[template_callbacks]
        impl MyObject {
            #[template_callback]
            fn my_object_destroyed(&self) {
                self.imp().counter.set(self.imp().counter.get() + 1);
            }
        }

        let counter = {
            MyObject::static_type();
            let builder = Builder::new();
            let scope = BuilderRustScope::new();
            MyObject::add_callbacks_to_scope(&scope);
            builder.set_scope(Some(&scope));
            builder.add_from_string(DISPOSE_XML).unwrap();
            let obj = builder.object::<MyObject>("obj").unwrap();
            obj.imp().counter.clone()
        };
        assert_eq!(counter.get(), 1);
    }
}
