// Take a look at the license at the top of the repository in the LICENSE file.

use crate::BuilderScope;

glib::wrapper! {
    #[doc(alias = "GtkBuilderCScope")]
    pub struct BuilderCScope(Object<ffi::GtkBuilderCScope, ffi::GtkBuilderCScopeClass>) @implements BuilderScope;

    match fn {
        type_ => || ffi::gtk_builder_cscope_get_type(),
    }
}
