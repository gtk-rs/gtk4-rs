// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(deprecated)]
#![allow(clippy::manual_c_str_literals)]
#![doc = include_str!("../README.md")]

pub use cairo;
pub use gdk;
pub use glib;
pub use graphene;
pub use gsk4_sys as ffi;
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
#[allow(clippy::too_many_arguments)]
#[allow(unused_imports)]
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
#[cfg(feature = "v4_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
mod fill_node;
mod gl_shader;
mod gl_shader_node;
mod inset_shadow_node;
mod linear_gradient_node;
#[cfg(feature = "v4_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
mod mask_node;
#[cfg(feature = "v4_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_2")))]
mod ngl_renderer;
mod opacity_node;
mod outset_shadow_node;
mod parse_location;
#[cfg(feature = "v4_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
mod path;
#[cfg(feature = "v4_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
mod path_builder;
#[cfg(feature = "v4_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
mod path_point;
mod radial_gradient_node;
mod repeat_node;
mod repeating_linear_gradient_node;
mod repeating_radial_gradient_node;
mod rounded_clip_node;
mod shadow_node;
#[cfg(feature = "v4_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
mod stroke;
#[cfg(feature = "v4_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
mod stroke_node;
mod text_node;
mod texture_node;
#[cfg(feature = "v4_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
mod texture_scale_node;
mod transform;
mod transform_node;

pub use color_stop::ColorStop;
#[cfg(feature = "v4_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_2")))]
#[cfg_attr(feature = "v4_4", deprecated = "Since 4.4")]
pub use ngl_renderer::NglRenderer;
pub use parse_location::ParseLocation;
pub use rounded_rect::RoundedRect;
pub use shadow::Shadow;
