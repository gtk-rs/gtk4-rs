// Take a look at the license at the top of the repository in the LICENSE file.

//! # GTK 4 Macros
//!
//! The crate aims to provide useful macros to use with the GTK 4 Rust bindings.

mod attribute_parser;
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
/// The `callbacks` attribute specifies this template has bound callbacks.
/// See [`macro@template_callbacks`] for a description of how to use them.
///
/// The `template_child` attribute is used to mark all internal widgets
/// we need to have programmatic access to.
///
/// The `template_child` attribute can take two parameters:
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
/// [`ObjectSubclass`]: https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/subclass/types/trait.ObjectSubclass.html
/// [`bind_template`]: https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/subclass/widget/trait.CompositeTemplate.html#tymethod.bind_template
/// [`class_init`]: https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/subclass/types/trait.ObjectSubclass.html#method.class_init
/// [`init_template`]: https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/prelude/trait.InitializingWidgetExt.html#tymethod.init_template
/// [`instance_init`]: https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/subclass/types/trait.ObjectSubclass.html#method.instance_init
///
/// ```no_run
/// # fn main() {}
/// use gtk::prelude::*;
/// use gtk::glib;
/// use gtk::CompositeTemplate;
/// use gtk::subclass::prelude::*;
///
/// mod imp {
///     use super::*;
///
///     #[derive(Debug, Default, CompositeTemplate)]
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
///             Self::bind_template(klass);
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
///         glib::Object::new(&[]).expect("Failed to create an instance of MyWidget")
///     }
/// }
/// ```
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
/// [`ObjectSubclass`] implementation you will need to call [`bind_template_callbacks`] in the
/// [`class_init`] function.
///
/// Template callbacks can be specified on both a widget's public `impl` or on its private `impl`.
/// Template callbacks from other types can also be used. Care must be taken to call
/// `bind_template_callbacks` on each type in order to use its callbacks.
///
/// These callbacks can be bound using the `<signal>` or `<closure>` tags in the template file.
/// Note that the arguments and return type will only be checked at run time when the method is
/// invoked.
///
/// Methods can optionally take `&self` as a first parameter. In this case, the attribute
/// `swapped="true"` will usually have to be set on the `<signal>` or `<closure>` tag in order to
/// invoke the function correctly.
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
/// must be used on an argument of type `&[glib::Value]` and will pass in the remaining arguments.
/// The first and last values will be omitted from the slice if this callback is a `function`.
///
/// Arguments and return types in template callbacks have some special restrictions, similar to the
/// restrictions on [`glib::closure`]. Each argument's type must implement [`glib::ToValue`]. The
/// last argument can also be `&[glib::Value]` annotated with `#[rest]` as described above. The
/// return type of a callback, if present, must implement [`glib::FromValue`]. Type-checking of
/// inputs and outputs is done at run-time; if the argument types or return type do not match the
/// type of the signal or closure then the callback will panic. To implement your own type checking
/// or to use dynamic typing, an argument's type can be left as a [`&glib::Value`].
/// This can also be used if you need custom unboxing, such as if the target type does not
/// implement `FromValue`.
///
/// [`glib::closure`]: https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/macro.closure.html
/// [`glib::wrapper`]: https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/macro.wrapper.html
/// [`ObjectSubclass`]: https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/subclass/types/trait.ObjectSubclass.html
/// [`class_init`]: https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/subclass/types/trait.ObjectSubclass.html#method.class_init
/// [`bind_template_callbacks`]: https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/subclass/widget/trait.CompositeTemplateCallbacks.html#tymethod.bind_template_callbacks
/// [`glib::FromValue`]: https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/value/trait.FromValue.html
/// [`glib::ToValue`]: https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/value/trait.ToValue.html
/// [`&glib::Value`]: https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/value/struct.Value.html
///
/// # Example
///
/// ```no_run
/// # fn main() {}
/// use gtk::prelude::*;
/// use gtk::glib;
/// use gtk::CompositeTemplate;
/// use gtk::subclass::prelude::*;
///
/// mod imp {
///     use super::*;
///
///     #[derive(Debug, Default, CompositeTemplate)]
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
///             Self::bind_template(klass);
///             // Bind the private callbacks
///             Self::bind_template_callbacks(klass);
///             // Bind the public callbacks
///             Self::Type::bind_template_callbacks(klass);
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
///         glib::Object::new(&[]).expect("Failed to create an instance of MyWidget")
///     }
///     #[template_callback]
///     pub fn print_both_labels(&self) {
///         let self_ = imp::MyWidget::from_instance(self);
///         println!("{} {}", self_.label.label(), self_.button.label().unwrap().as_str());
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
