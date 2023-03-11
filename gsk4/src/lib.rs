// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![doc = include_str!("../README.md")]

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

#[allow(clippy::derived_hash_with_manual_eq)]
mod auto;

pub mod builders;
pub mod prelude;
pub use auto::*;

#[macro_use]
mod render_node;
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
mod gl_shader;
mod gl_shader_node;
mod inset_shadow_node;
mod linear_gradient_node;
#[cfg(any(feature = "v4_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
mod mask_node;
#[cfg(any(feature = "v4_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v4_2")))]
mod ngl_renderer;
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
#[cfg(any(feature = "v4_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
mod texture_scale_node;
mod transform;
mod transform_node;

pub use color_stop::ColorStop;
#[cfg(any(feature = "v4_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v4_2")))]
#[cfg_attr(feature = "v4_4", deprecated = "Since 4.4")]
pub use ngl_renderer::NglRenderer;
pub use parse_location::ParseLocation;
pub use rounded_rect::RoundedRect;
pub use shadow::Shadow;
