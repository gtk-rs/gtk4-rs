// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(clippy::derive_hash_xor_eq)]
#![allow(clippy::too_many_arguments)]

pub use cairo;
pub use ffi;
pub use gdk;
pub use glib;
pub use graphene;
pub use pango;

// GSK 4 has no runtime to initialize
macro_rules! assert_initialized_main_thread {
    () => {};
}

// No-op
macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(clippy::upper_case_acronyms)]
#[allow(unused_imports)]
mod auto;

pub mod prelude;

pub use auto::*;

#[macro_use]
mod render_node;
mod renderer;
mod rounded_rect;
mod shadow;

// Render node types
mod blend_node;
mod blur_node;
mod border_node;
mod cairo_node;
mod clip_node;
mod color_matrix_node;
mod color_node;
mod color_stop;
mod conic_gradient_node;
mod container_node;
mod cross_fade_node;
mod debug_node;
mod gl_shader_node;
mod inset_shadow_node;
mod linear_gradient_node;
mod opacity_node;
mod outset_shadow_node;
mod parse_location;
mod radial_gradient_node;
mod repeat_node;
mod repeating_linear_gradient_node;
mod repeating_radial_gradient_node;
mod rounded_clip_node;
mod shadow_node;
mod text_node;
mod texture_node;
mod transform_node;

pub use blend_node::BlendNode;
pub use blur_node::BlurNode;
pub use border_node::BorderNode;
pub use cairo_node::CairoNode;
pub use clip_node::ClipNode;
pub use color_matrix_node::ColorMatrixNode;
pub use color_node::ColorNode;
pub use color_stop::ColorStop;
pub use conic_gradient_node::ConicGradientNode;
pub use container_node::ContainerNode;
pub use cross_fade_node::CrossFadeNode;
pub use debug_node::DebugNode;
pub use gl_shader_node::GLShaderNode;
pub use inset_shadow_node::InsetShadowNode;
pub use linear_gradient_node::LinearGradientNode;
pub use opacity_node::OpacityNode;
pub use outset_shadow_node::OutsetShadowNode;
pub use parse_location::ParseLocation;
pub use radial_gradient_node::RadialGradientNode;
pub use render_node::{IsRenderNode, RenderNode, NONE_RENDER_NODE};
pub use repeat_node::RepeatNode;
pub use repeating_linear_gradient_node::RepeatingLinearGradientNode;
pub use repeating_radial_gradient_node::RepeatingRadialGradientNode;
pub use rounded_clip_node::RoundedClipNode;
pub use rounded_rect::RoundedRect;
pub use shadow::Shadow;
pub use shadow_node::ShadowNode;
pub use text_node::TextNode;
pub use texture_node::TextureNode;
pub use transform_node::TransformNode;
