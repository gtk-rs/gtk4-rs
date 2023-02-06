// Take a look at the license at the top of the repository in the LICENSE file.

//! # GTK 4 Macros
//!
//! The crate aims to provide useful macros to use with the GTK 4 Rust bindings.

mod attribute_parser;
#[cfg(feature = "blueprint")]
mod blueprint;
mod composite_template_derive;
mod template_callbacks_attribute;
mod util;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for using a composite template in a widget.
///
/// The `template` attribute specifies where the template should be loaded
/// from;  it can be a `file`, a `resource`, or a `string`.
///
/// The `template_child` attribute is used to mark all internal widgets
/// we need to have programmatic access to. It can take two parameters:
/// - `id` which defaults to the item name if not defined
/// - `internal_child` whether the child should be accessible as an “internal-child”, defaults to `false`
///
/// # Example
///
/// Specify that `MyWidget` is using a composite template and load the
/// template file the `composite_template.ui` file.
///
/// Then, in the [`ObjectSubclass`] implementation you will need to call
/// [`bind_template`] in the [`class_init`] function, and [`init_template`] in
/// [`instance_init`] function.
///
/// [`ObjectSubclass`]: ../glib/subclass/types/trait.ObjectSubclass.html
/// [`bind_template`]: ../gtk4/subclass/widget/trait.CompositeTemplate.html#tymethod.bind_template
/// [`class_init`]: ../glib/subclass/types/trait.ObjectSubclass.html#method.class_init
/// [`init_template`]: ../gtk4/subclass/prelude/trait.CompositeTemplateInitializingExt.html#tymethod.init_template
/// [`instance_init`]: ../glib/subclass/types/trait.ObjectSubclass.html#method.instance_init
///
/// ```no_run
/// # fn main() {}
/// use gtk::{glib, prelude::*, subclass::prelude::*};
///
/// mod imp {
///     use super::*;
///
///     #[derive(Debug, Default, gtk::CompositeTemplate)]
///     #[template(file = "test/template.ui")]
///     pub struct MyWidget {
///         #[template_child]
///         pub label: TemplateChild<gtk::Label>,
///         #[template_child(id = "my_button_id")]
///         pub button: TemplateChild<gtk::Button>,
///     }
///
///     #[glib::object_subclass]
///     impl ObjectSubclass for MyWidget {
///         const NAME: &'static str = "MyWidget";
///         type Type = super::MyWidget;
///         type ParentType = gtk::Box;
///
///         fn class_init(klass: &mut Self::Class) {
///             klass.bind_template();
///         }
///
///         fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
///             obj.init_template();
///         }
///     }
///
///     impl ObjectImpl for MyWidget {}
///     impl WidgetImpl for MyWidget {}
///     impl BoxImpl for MyWidget {}
/// }
///
/// glib::wrapper! {
///     pub struct MyWidget(ObjectSubclass<imp::MyWidget>) @extends gtk::Widget, gtk::Box;
/// }
///
/// impl MyWidget {
///     pub fn new() -> Self {
///         glib::Object::new()
///     }
/// }
/// ```
///
/// The [`CompositeTemplate`] macro can also be used with [Blueprint](https://jwestman.pages.gitlab.gnome.org/blueprint-compiler/)
/// if the feature `blueprint` is enabled.
///
/// ```ignore
/// # fn main() {}
/// use gtk::{glib, prelude::*, subclass::prelude::*};
///
/// mod imp {
///     use super::*;
///
///     #[derive(Debug, Default, gtk::CompositeTemplate)]
///     #[template(string = "
///     template MyWidget : Widget {
///         Label label {
///             label: 'foobar';
///         }
///
///         Label my_label2 {
///             label: 'foobaz';
///         }
///     }
///     ")]
///     pub struct MyWidget {
///         #[template_child]
///         pub label: TemplateChild<gtk::Label>,
///         #[template_child(id = "my_label2")]
///         pub label2: gtk::TemplateChild<gtk::Label>,
///     }
///
///     #[glib::object_subclass]
///     impl ObjectSubclass for MyWidget {
///         const NAME: &'static str = "MyWidget";
///         type Type = super::MyWidget;
///         type ParentType = gtk::Widget;
///         fn class_init(klass: &mut Self::Class) {
///             klass.bind_template();
///         }
///         fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
///             obj.init_template();
///         }
///     }
///
///     impl ObjectImpl for MyWidget {
///         fn dispose(&self) {
///             while let Some(child) = self.obj().first_child() {
///                 child.unparent();
///             }
///         }
///     }
///     impl WidgetImpl for MyWidget {}
/// }
///
/// glib::wrapper! {
///     pub struct MyWidget(ObjectSubclass<imp::MyWidget>) @extends gtk::Widget;
/// }
/// ```
///
#[proc_macro_derive(CompositeTemplate, attributes(template, template_child))]
#[proc_macro_error]
pub fn composite_template_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let gen = composite_template_derive::impl_composite_template(&input);
    gen.into()
}

/// Attribute macro for creating template callbacks from Rust methods.
///
/// Widgets with [`CompositeTemplate`] can then make use of these callbacks from within their
/// template XML definition. The attribute must be applied to an `impl` statement of a struct.
/// Functions marked as callbacks within the `impl` will be stored in a static array. Then, in the
/// [`ObjectSubclass`] implementation you will need to call [`bind_template_callbacks`] and/or
/// [`bind_template_instance_callbacks`] in the [`class_init`] function.
///
/// Template callbacks can be specified on both a widget's public wrapper `impl` or on its private
/// subclass `impl`, or from external types. If callbacks are specified on the public wrapper, then
/// `bind_template_instance_callbacks` must be called in `class_init`. If callbacks are specified
/// on the private subclass, then `bind_template_callbacks` must be called in `class_init`. To use
/// the callbacks from an external type, call [`T::bind_template_callbacks`] in `class_init`, where
/// `T` is the other type. See the example below for usage of all three.
///
/// These callbacks can be bound using the `<signal>` or `<closure>` tags in the template file.
/// Note that the arguments and return type will only be checked at run time when the method is
/// invoked.
///
/// Template callbacks can optionally take `self` or `&self` as a first parameter. In this case,
/// the attribute `swapped="true"` will usually have to be set on the `<signal>` or `<closure>` tag
/// in order to invoke the function correctly. Note that by-value `self` will only work with
/// template callbacks on the wrapper type.
///
/// Template callbacks that have no return value can also be `async`, in which case the callback
/// will be spawned as new future on the default main context using
/// [`glib::MainContext::spawn_local`]. Invoking the callback multiple times will spawn an
/// additional future each time it is invoked. This means that multiple futures for an async
/// callback can be active at any given time, so care must be taken to avoid any kind of data
/// races. Async callbacks may prefer communicating back to the caller or widget over channels
/// instead of mutating internal widget state, or may want to make use of a locking flag to ensure
/// only one future can be active at once. Widgets may also want to show a visual indicator such as
/// a [`Spinner`] while the future is active to communicate to the user that a background task is
/// running.
///
/// The following options are supported on the attribute:
/// - `functions` makes all callbacks use the `function` attribute by default. (see below)
///
/// The `template_callback` attribute is used to mark methods that will be exposed to the template
/// scope. It can take the following options:
/// - `name` renames the callback. Defaults to the function name if not defined.
/// - `function` ignores the first value when calling the callback and disallows `self`.  Useful
/// for callbacks called from `<closure>` tags.
/// - `function = false` reverts the effects of `functions` used on the `impl`, so the callback
/// gets the first value and can take `self` again. Mainly useful for callbacks that are invoked
/// with `swapped="true"`.
///
/// The `rest` attribute can be placed on the last argument of a template callback. This attribute
/// must be used on an argument of type <code>&\[[glib::Value]\]</code> and will pass in the
/// remaining arguments. The first and last values will be omitted from the slice if this callback
/// is a `function`.
///
/// Arguments and return types in template callbacks have some special restrictions, similar to the
/// restrictions on [`glib::closure`]. Each argument's type must implement
/// <code>[From]&lt;Type> for [glib::Value]</code>. The last argument can also be
/// <code>&\[[glib::Value]\]</code> annotated with `#[rest]` as described above. The return type of
/// a callback, if present, must implement [`glib::FromValue`]. Type-checking of inputs and outputs
/// is done at run-time; if the argument types or return type do not match the type of the signal
/// or closure then the callback will panic. To implement your own type checking or to use dynamic
/// typing, an argument's type can be left as a <code>&[glib::Value]</code>. This can also be used
/// if you need custom unboxing, such as if the target type does not implement `FromValue`.
///
/// [`glib::closure`]: ../glib/macro.closure.html
/// [`glib::wrapper`]: ../glib/macro.wrapper.html
/// [`ObjectSubclass`]: ../glib/subclass/types/trait.ObjectSubclass.html
/// [`class_init`]: ../glib/subclass/types/trait.ObjectSubclass.html#method.class_init
/// [`bind_template_callbacks`]: ../gtk4/subclass/widget/trait.CompositeTemplateCallbacksClass.html#tymethod.bind_template_callbacks
/// [`bind_template_instance_callbacks`]: ../gtk4/subclass/widget/trait.CompositeTemplateInstanceCallbacksClass.html#tymethod.bind_template_instance_callbacks
/// [`T::bind_template_callbacks`]: ../gtk4/subclass/widget/trait.CompositeTemplateCallbacks.html#method.bind_template_callbacks
/// [`glib::FromValue`]: ../glib/value/trait.FromValue.html
/// [`glib::ToValue`]: ../glib/value/trait.ToValue.html
/// [glib::Value]: ../glib/value/struct.Value.html
/// [`glib::MainContext::spawn_local`]: ../glib/struct.MainContext.html#method.spawn_local
/// [`Spinner`]: ../gtk4/struct.Spinner.html
///
/// # Example
///
/// ```no_run
/// # fn main() {}
/// use gtk::{glib, prelude::*, subclass::prelude::*};
///
/// mod imp {
///     use super::*;
///
///     #[derive(Debug, Default, gtk::CompositeTemplate)]
///     #[template(file = "test/template_callbacks.ui")]
///     pub struct MyWidget {
///         #[template_child]
///         pub label: TemplateChild<gtk::Label>,
///         #[template_child(id = "my_button_id")]
///         pub button: TemplateChild<gtk::Button>,
///     }
///
///     #[glib::object_subclass]
///     impl ObjectSubclass for MyWidget {
///         const NAME: &'static str = "MyWidget";
///         type Type = super::MyWidget;
///         type ParentType = gtk::Box;
///
///         fn class_init(klass: &mut Self::Class) {
///             klass.bind_template();
///             // Bind the private callbacks
///             klass.bind_template_callbacks();
///             // Bind the public callbacks
///             klass.bind_template_instance_callbacks();
///             // Bind callbacks from another struct
///             super::Utility::bind_template_callbacks(klass);
///         }
///
///         fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
///             obj.init_template();
///         }
///     }
///
///     #[gtk::template_callbacks]
///     impl MyWidget {
///         #[template_callback]
///         fn button_clicked(&self, button: &gtk::Button) {
///             button.set_label("I was clicked!");
///             self.label.set_label("The button was clicked!");
///         }
///         #[template_callback(function, name = "strlen")]
///         fn string_length(s: &str) -> u64 {
///             s.len() as u64
///         }
///     }
///
///     impl ObjectImpl for MyWidget {}
///     impl WidgetImpl for MyWidget {}
///     impl BoxImpl for MyWidget {}
/// }
///
/// glib::wrapper! {
///     pub struct MyWidget(ObjectSubclass<imp::MyWidget>) @extends gtk::Widget, gtk::Box;
/// }
///
/// #[gtk::template_callbacks]
/// impl MyWidget {
///     pub fn new() -> Self {
///         glib::Object::new()
///     }
///     #[template_callback]
///     pub fn print_both_labels(&self) {
///         let imp = self.imp();
///         println!("{} {}", imp.label.label(), imp.button.label().unwrap().as_str());
///     }
/// }
///
/// pub struct Utility {}
///
/// #[gtk::template_callbacks(functions)]
/// impl Utility {
///     #[template_callback]
///     fn concat_strs(#[rest] values: &[glib::Value]) -> String {
///         let mut res = String::new();
///         for (index, value) in values.iter().enumerate() {
///             res.push_str(value.get::<&str>().unwrap_or_else(|e| {
///                 panic!("Expected string value for argument {}: {}", index, e);
///             }));
///         }
///         res
///     }
///     #[template_callback(function = false)]
///     fn reset_label(label: &gtk::Label) {
///         label.set_label("");
///     }
/// }
/// ```
#[proc_macro_attribute]
#[proc_macro_error]
pub fn template_callbacks(attr: TokenStream, item: TokenStream) -> TokenStream {
    use proc_macro_error::abort_call_site;
    let args = parse_macro_input!(attr as template_callbacks_attribute::Args);
    match syn::parse::<syn::ItemImpl>(item) {
        Ok(input) => template_callbacks_attribute::impl_template_callbacks(input, args).into(),
        Err(_) => abort_call_site!(template_callbacks_attribute::WRONG_PLACE_MSG),
    }
}

/// Attribute macro for declaring GTK tests.
///
/// Wraps the standard Rust [`test`] attribute with setup logic for GTK. All tests that call
/// into GTK must use this attribute. This attribute can also be used on asynchronous functions;
/// the asynchronous test will be run on the main thread context.
///
/// # Technical Details
///
/// GTK is a single-threaded library, so Rust's normal multi-threaded test behavior cannot be used.
/// The `#[gtk::test]` attribute creates a main thread for GTK and runs all tests on that thread.
/// This has the side effect of making all tests run serially, not in parallel.
///
/// [`test`]: <https://doc.rust-lang.org/std/prelude/v1/macro.test.html>
///
/// # Example
///
/// ```no_run
/// use gtk::prelude::*;
///
/// #[gtk::test]
/// fn test_button() {
///     let button = gtk::Button::new();
///     button.activate();
/// }
/// ```
#[proc_macro_attribute]
#[proc_macro_error]
pub fn test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    use proc_macro_error::abort_call_site;
    use quote::quote;

    match syn::parse::<syn::ItemFn>(item) {
        Ok(mut input) => {
            let crate_ident = util::crate_ident_new();
            let block = &input.block;
            let block = if input.sig.asyncness.is_some() {
                quote! {
                    #crate_ident::glib::MainContext::default().block_on(async move {
                        #block
                    })
                }
            } else {
                quote! { #block }
            };
            input.sig.asyncness.take();

            let attrs = &input.attrs;
            let vis = &input.vis;
            let sig = &input.sig;
            let test = quote! {
                #(#attrs)*
                #[::std::prelude::v1::test]
                #vis #sig {
                    #crate_ident::test_synced(move || {
                        #block
                    })
                }
            };
            test.into()
        }
        Err(_) => abort_call_site!("This macro should be used on a function definition"),
    }
}
