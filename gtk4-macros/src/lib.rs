// Take a look at the license at the top of the repository in the LICENSE file.

mod attribute_parser;
mod composite_template_derive;
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
/// we need to have programmatic access to.
///
/// # Example
///
/// Specify that `MyWidget` is using a composite template and load the
/// template file the `composite_template.ui` file.
///
/// ```
/// use gtk::prelude::*;
/// use gtk::CompositeTemplate;
///
/// #[derive(Debug, CompositeTemplate)]
/// #[template(file = "composite_template.ui")]
/// struct MyWidget {
///     #[template_child]
///     pub label: TemplateChild<gtk::Label>,
/// }
/// ```
///
/// Then, in the `ObjectSubclass` implementation you will need to call
/// `bind_template` in the `class_init` function, and `init_template` in
/// `instance_init` function.
///
///
/// ```compile_fail
/// impl ObjectSubclass for MyWidget {
///        const NAME: &'static str = "MyWidget";
///        type Type = super::MyWidget;
///        type ParentType = gtk::Widget;
///        type Instance = subclass::simple::InstanceStruct<Self>;
///        type Class = subclass::simple::ClassStruct<Self>;
///
///        glib::object_subclass!();
///
///        fn new() -> Self {
///            Self {
///                label: TemplateChild::default(),
///            }
///        }
///
///        fn class_init(klass: &mut Self::Class) {
///            Self::bind_template(klass);
///        }
///
///        fn instance_init(obj: &glib::subclass::InitializingObject<Self::Type>) {
///            obj.init_template();
///        }
///    }
/// ```
#[proc_macro_derive(CompositeTemplate, attributes(template, template_child))]
#[proc_macro_error]
pub fn composite_template_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let gen = composite_template_derive::impl_composite_template(&input);
    gen.into()
}
