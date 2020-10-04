// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use cairo;
use color_stop::ColorStop;
use gdk;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use graphene;
use pango;
use BlendMode;
use RenderNode;
use RenderNodeType;
use RoundedRect;

use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::ops::Deref;

// RenderNode subtypes

macro_rules! subtype(
    ($subtype: ident) => (
        #[derive(Debug, Clone)]
        pub struct $subtype(RenderNode);

        impl Deref for $subtype {
            type Target = RenderNode;

            fn deref(&self) -> &RenderNode {
                &self.0
            }
        }

        impl fmt::Display for $subtype {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, stringify!($subtype))
            }
        }

        impl TryFrom<RenderNode> for $subtype {
            type Error = RenderNode;

            fn try_from(value: RenderNode) -> Result<Self, RenderNode> {
                assert_initialized_main_thread!();
                if value.get_node_type() == RenderNodeType::$subtype {
                    Ok($subtype(value))
                }
                else {
                    Err(value)
                }
            }
        }
    );
);

subtype!(BlendNode);
subtype!(BlurNode);
subtype!(BorderNode);
subtype!(CairoNode);
subtype!(ClipNode);
subtype!(ColorMatrixNode);
subtype!(ColorNode);
subtype!(ContainerNode);
subtype!(CrossFadeNode);
subtype!(DebugNode);
subtype!(InsetShadowNode);
subtype!(LinearGradientNode);
subtype!(OpacityNode);
subtype!(OutsetShadowNode);
subtype!(RepeatNode);
subtype!(RoundedClipNode);
subtype!(TextNode);
subtype!(TextureNode);

impl BlendNode {
    pub fn get_blend_mode(&self) -> BlendMode {
        unsafe {
            from_glib(gsk_sys::gsk_blend_node_get_blend_mode(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_bottom_child(&self) -> RenderNode {
        unsafe {
            from_glib_full(gsk_sys::gsk_blend_node_get_bottom_child(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_top_child(&self) -> RenderNode {
        unsafe { from_glib_full(gsk_sys::gsk_blend_node_get_top_child(self.to_glib_none().0)) }
    }

    pub fn new(bottom: &RenderNode, top: &RenderNode, blend_mode: BlendMode) -> BlendNode {
        assert_initialized_main_thread!();
        let node: RenderNode = unsafe {
            from_glib_full(gsk_sys::gsk_blend_node_new(
                bottom.to_glib_none().0,
                top.to_glib_none().0,
                blend_mode.to_glib(),
            ))
        };
        node.try_into().unwrap()
    }
}

impl BlurNode {
    pub fn get_child(&self) -> RenderNode {
        unsafe { from_glib_full(gsk_sys::gsk_blur_node_get_child(self.to_glib_none().0)) }
    }

    pub fn get_radius(&self) -> f64 {
        unsafe { gsk_sys::gsk_blur_node_get_radius(self.to_glib_none().0) }
    }

    pub fn new(child: &RenderNode, radius: f64) -> BlurNode {
        assert_initialized_main_thread!();
        let node: RenderNode =
            unsafe { from_glib_full(gsk_sys::gsk_blur_node_new(child.to_glib_none().0, radius)) };
        node.try_into().unwrap()
    }
}

impl BorderNode {
    pub fn new(
        outline: &RoundedRect,
        border_width: &[f32; 4],
        border_color: &[gdk::RGBA; 4],
    ) -> BorderNode {
        assert_initialized_main_thread!();
        let node: RenderNode = unsafe {
            from_glib_full(gsk_sys::gsk_border_node_new(
                outline.to_glib_none().0,
                border_width.to_glib_none().0 as *const _,
                border_color.as_ptr() as *const _,
            ))
        };
        node.try_into().unwrap()
    }

    pub fn peek_colors(&self) -> gdk::RGBA {
        unsafe { from_glib_none(gsk_sys::gsk_border_node_peek_colors(self.to_glib_none().0)) }
    }

    pub fn peek_outline(&self) -> RoundedRect {
        unsafe { from_glib_none(gsk_sys::gsk_border_node_peek_outline(self.to_glib_none().0)) }
    }

    pub fn peek_widths(&self) -> [f32; 4] {
        unsafe {
            let widths = gsk_sys::gsk_border_node_peek_widths(self.to_glib_none().0);
            [*widths, *widths.add(1), *widths.add(2), *widths.add(3)]
        }
    }
}

impl CairoNode {
    pub fn get_draw_context(&self) -> cairo::Context {
        unsafe {
            from_glib_full(gsk_sys::gsk_cairo_node_get_draw_context(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn new(bounds: &graphene::Rect) -> CairoNode {
        assert_initialized_main_thread!();
        let node: RenderNode =
            unsafe { from_glib_full(gsk_sys::gsk_cairo_node_new(bounds.to_glib_none().0)) };
        node.try_into().unwrap()
    }

    /* TODO `const cairo_surface_t *` isn't supported
    pub fn peek_surface(&self) -> cairo::Surface {
        unsafe {
            from_glib_none(gsk_sys::gsk_cairo_node_peek_surface(self.to_glib_none().0))
        }
    }*/
}

impl ClipNode {
    pub fn get_child(&self) -> RenderNode {
        unsafe { from_glib_none(gsk_sys::gsk_clip_node_get_child(self.to_glib_none().0)) }
    }

    pub fn new(child: &RenderNode, clip: &graphene::Rect) -> ClipNode {
        assert_initialized_main_thread!();
        let node: RenderNode = unsafe {
            from_glib_full(gsk_sys::gsk_clip_node_new(
                child.to_glib_none().0,
                clip.to_glib_none().0,
            ))
        };
        node.try_into().unwrap()
    }

    pub fn peek_clip(&self) -> graphene::Rect {
        unsafe { from_glib_none(gsk_sys::gsk_clip_node_peek_clip(self.to_glib_none().0)) }
    }
}

impl ColorMatrixNode {
    pub fn get_child(&self) -> RenderNode {
        unsafe {
            from_glib_none(gsk_sys::gsk_color_matrix_node_get_child(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn new(
        child: &RenderNode,
        color_matrix: &graphene::Matrix,
        color_offset: &graphene::Vec4,
    ) -> ColorMatrixNode {
        assert_initialized_main_thread!();
        let node: RenderNode = unsafe {
            from_glib_full(gsk_sys::gsk_color_matrix_node_new(
                child.to_glib_none().0,
                color_matrix.to_glib_none().0,
                color_offset.to_glib_none().0,
            ))
        };
        node.try_into().unwrap()
    }

    pub fn peek_color_matrix(&self) -> graphene::Matrix {
        unsafe {
            from_glib_none(gsk_sys::gsk_color_matrix_node_peek_color_matrix(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn peek_color_offset(&self) -> graphene::Vec4 {
        unsafe {
            from_glib_none(gsk_sys::gsk_color_matrix_node_peek_color_offset(
                self.to_glib_none().0,
            ))
        }
    }
}

impl ColorNode {
    pub fn new(rgba: &gdk::RGBA, bounds: &graphene::Rect) -> ColorNode {
        assert_initialized_main_thread!();
        let node: RenderNode = unsafe {
            from_glib_full(gsk_sys::gsk_color_node_new(
                rgba.to_glib_none().0,
                bounds.to_glib_none().0,
            ))
        };
        node.try_into().unwrap()
    }

    pub fn peek_color(&self) -> gdk::RGBA {
        unsafe { from_glib_none(gsk_sys::gsk_color_node_peek_color(self.to_glib_none().0)) }
    }
}

impl ContainerNode {
    pub fn get_child(&self, idx: u32) -> RenderNode {
        unsafe {
            from_glib_full(gsk_sys::gsk_container_node_get_child(
                self.to_glib_none().0,
                idx,
            ))
        }
    }

    pub fn get_n_children(&self) -> u32 {
        unsafe { gsk_sys::gsk_container_node_get_n_children(self.to_glib_none().0) }
    }

    pub fn new(children: &[RenderNode]) -> ContainerNode {
        assert_initialized_main_thread!();
        let n_children = children.len() as u32;
        let node: RenderNode = unsafe {
            from_glib_full(gsk_sys::gsk_container_node_new(
                children.to_glib_none().0,
                n_children,
            ))
        };
        node.try_into().unwrap()
    }
}

impl CrossFadeNode {
    pub fn get_end_child(&self) -> RenderNode {
        unsafe {
            from_glib_full(gsk_sys::gsk_cross_fade_node_get_end_child(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_progress(&self) -> f64 {
        unsafe { gsk_sys::gsk_cross_fade_node_get_progress(self.to_glib_none().0) }
    }

    pub fn get_start_child(&self) -> RenderNode {
        unsafe {
            from_glib_full(gsk_sys::gsk_cross_fade_node_get_start_child(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn new(start: &RenderNode, end: &RenderNode, progress: f64) -> CrossFadeNode {
        assert_initialized_main_thread!();
        let node: RenderNode = unsafe {
            from_glib_full(gsk_sys::gsk_cross_fade_node_new(
                start.to_glib_none().0,
                end.to_glib_none().0,
                progress,
            ))
        };
        node.try_into().unwrap()
    }
}

impl DebugNode {
    pub fn get_child(&self) -> RenderNode {
        unsafe { from_glib_none(gsk_sys::gsk_debug_node_get_child(self.to_glib_none().0)) }
    }

    pub fn get_message(&self) -> GString {
        unsafe { from_glib_none(gsk_sys::gsk_debug_node_get_message(self.to_glib_none().0)) }
    }

    pub fn new(child: &RenderNode, message: &str) -> DebugNode {
        assert_initialized_main_thread!();
        let node: RenderNode = unsafe {
            from_glib_full(gsk_sys::gsk_debug_node_new(
                child.to_glib_none().0,
                message.to_glib_full(),
            ))
        };
        node.try_into().unwrap()
    }
}

impl InsetShadowNode {
    pub fn get_blur_radius(&self) -> f32 {
        unsafe { gsk_sys::gsk_inset_shadow_node_get_blur_radius(self.to_glib_none().0) }
    }

    pub fn get_dx(&self) -> f32 {
        unsafe { gsk_sys::gsk_inset_shadow_node_get_dx(self.to_glib_none().0) }
    }

    pub fn get_dy(&self) -> f32 {
        unsafe { gsk_sys::gsk_inset_shadow_node_get_dy(self.to_glib_none().0) }
    }

    pub fn get_spread(&self) -> f32 {
        unsafe { gsk_sys::gsk_inset_shadow_node_get_spread(self.to_glib_none().0) }
    }

    pub fn new(
        outline: &RoundedRect,
        color: &gdk::RGBA,
        dx: f32,
        dy: f32,
        spread: f32,
        blur_radius: f32,
    ) -> InsetShadowNode {
        assert_initialized_main_thread!();
        let node: RenderNode = unsafe {
            from_glib_full(gsk_sys::gsk_inset_shadow_node_new(
                outline.to_glib_none().0,
                color.to_glib_none().0,
                dx,
                dy,
                spread,
                blur_radius,
            ))
        };
        node.try_into().unwrap()
    }

    pub fn peek_color(&self) -> gdk::RGBA {
        unsafe {
            from_glib_none(gsk_sys::gsk_inset_shadow_node_peek_color(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn peek_outline(&self) -> RoundedRect {
        unsafe {
            from_glib_none(gsk_sys::gsk_inset_shadow_node_peek_outline(
                self.to_glib_none().0,
            ))
        }
    }
}

impl LinearGradientNode {
    pub fn get_n_color_stops(&self) -> usize {
        unsafe { gsk_sys::gsk_linear_gradient_node_get_n_color_stops(self.to_glib_none().0) }
    }

    pub fn new(
        bounds: &graphene::Rect,
        start: &graphene::Point,
        end: &graphene::Point,
        color_stops: &[ColorStop],
    ) -> LinearGradientNode {
        assert_initialized_main_thread!();
        let n_color_stops = color_stops.len() as usize;
        let node: RenderNode = unsafe {
            from_glib_full(gsk_sys::gsk_linear_gradient_node_new(
                bounds.to_glib_none().0,
                start.to_glib_none().0,
                end.to_glib_none().0,
                color_stops.to_glib_none().0,
                n_color_stops,
            ))
        };
        node.try_into().unwrap()
    }

    pub fn new_repeating(
        bounds: &graphene::Rect,
        start: &graphene::Point,
        end: &graphene::Point,
        color_stops: &[ColorStop],
    ) -> LinearGradientNode {
        assert_initialized_main_thread!();
        let n_color_stops = color_stops.len() as usize;
        let node: RenderNode = unsafe {
            from_glib_full(gsk_sys::gsk_repeating_linear_gradient_node_new(
                bounds.to_glib_none().0,
                start.to_glib_none().0,
                end.to_glib_none().0,
                color_stops.to_glib_none().0,
                n_color_stops,
            ))
        };
        node.try_into().unwrap()
    }

    pub fn peek_color_stops(&self) -> Vec<ColorStop> {
        let n = self.get_n_color_stops();
        unsafe {
            FromGlibContainerAsVec::from_glib_none_num_as_vec(
                gsk_sys::gsk_linear_gradient_node_peek_color_stops(self.to_glib_none().0),
                n,
            )
        }
    }

    pub fn peek_end(&self) -> graphene::Point {
        unsafe {
            from_glib_none(gsk_sys::gsk_linear_gradient_node_peek_end(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn peek_start(&self) -> graphene::Point {
        unsafe {
            from_glib_none(gsk_sys::gsk_linear_gradient_node_peek_start(
                self.to_glib_none().0,
            ))
        }
    }
}

impl OpacityNode {
    pub fn get_child(&self) -> RenderNode {
        unsafe { from_glib_none(gsk_sys::gsk_opacity_node_get_child(self.to_glib_none().0)) }
    }

    pub fn get_opacity(&self) -> f64 {
        unsafe { gsk_sys::gsk_opacity_node_get_opacity(self.to_glib_none().0) }
    }

    pub fn new(child: &RenderNode, opacity: f64) -> OpacityNode {
        assert_initialized_main_thread!();
        let node: RenderNode = unsafe {
            from_glib_full(gsk_sys::gsk_opacity_node_new(
                child.to_glib_none().0,
                opacity,
            ))
        };
        node.try_into().unwrap()
    }
}

impl OutsetShadowNode {
    pub fn get_blur_radius(&self) -> f32 {
        unsafe { gsk_sys::gsk_outset_shadow_node_get_blur_radius(self.to_glib_none().0) }
    }

    pub fn get_dx(&self) -> f32 {
        unsafe { gsk_sys::gsk_outset_shadow_node_get_dx(self.to_glib_none().0) }
    }

    pub fn get_dy(&self) -> f32 {
        unsafe { gsk_sys::gsk_outset_shadow_node_get_dy(self.to_glib_none().0) }
    }

    pub fn get_spread(&self) -> f32 {
        unsafe { gsk_sys::gsk_outset_shadow_node_get_spread(self.to_glib_none().0) }
    }

    pub fn new(
        outline: &RoundedRect,
        color: &gdk::RGBA,
        dx: f32,
        dy: f32,
        spread: f32,
        blur_radius: f32,
    ) -> OutsetShadowNode {
        assert_initialized_main_thread!();
        let node: RenderNode = unsafe {
            from_glib_full(gsk_sys::gsk_outset_shadow_node_new(
                outline.to_glib_none().0,
                color.to_glib_none().0,
                dx,
                dy,
                spread,
                blur_radius,
            ))
        };
        node.try_into().unwrap()
    }

    pub fn peek_color(&self) -> gdk::RGBA {
        unsafe {
            from_glib_none(gsk_sys::gsk_outset_shadow_node_peek_color(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn peek_outline(&self) -> RoundedRect {
        unsafe {
            from_glib_none(gsk_sys::gsk_outset_shadow_node_peek_outline(
                self.to_glib_none().0,
            ))
        }
    }
}

impl RepeatNode {
    pub fn get_child(&self) -> RenderNode {
        unsafe { from_glib_full(gsk_sys::gsk_repeat_node_get_child(self.to_glib_none().0)) }
    }

    pub fn new(
        bounds: &graphene::Rect,
        child: &RenderNode,
        child_bounds: Option<&graphene::Rect>,
    ) -> RepeatNode {
        assert_initialized_main_thread!();
        let node: RenderNode = unsafe {
            from_glib_full(gsk_sys::gsk_repeat_node_new(
                bounds.to_glib_none().0,
                child.to_glib_none().0,
                child_bounds.to_glib_none().0,
            ))
        };
        node.try_into().unwrap()
    }

    pub fn peek_child_bounds(&self) -> graphene::Rect {
        unsafe {
            from_glib_none(gsk_sys::gsk_repeat_node_peek_child_bounds(
                self.to_glib_none().0,
            ))
        }
    }
}

impl RoundedClipNode {
    pub fn get_child(&self) -> RenderNode {
        unsafe {
            from_glib_none(gsk_sys::gsk_rounded_clip_node_get_child(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn new(child: &RenderNode, clip: &RoundedRect) -> RoundedClipNode {
        assert_initialized_main_thread!();
        let node: RenderNode = unsafe {
            from_glib_full(gsk_sys::gsk_rounded_clip_node_new(
                child.to_glib_none().0,
                clip.to_glib_none().0,
            ))
        };
        node.try_into().unwrap()
    }

    pub fn peek_clip(&self) -> RoundedRect {
        unsafe {
            from_glib_none(gsk_sys::gsk_rounded_clip_node_peek_clip(
                self.to_glib_none().0,
            ))
        }
    }
}

impl TextNode {
    pub fn get_num_glyphs(&self) -> u32 {
        unsafe { gsk_sys::gsk_text_node_get_num_glyphs(self.to_glib_none().0) }
    }

    pub fn get_x(&self) -> f32 {
        unsafe { gsk_sys::gsk_text_node_get_x(self.to_glib_none().0) }
    }

    pub fn get_y(&self) -> f32 {
        unsafe { gsk_sys::gsk_text_node_get_y(self.to_glib_none().0) }
    }

    pub fn new<P: IsA<pango::Font>>(
        font: &P,
        glyphs: &mut pango::GlyphString,
        color: &gdk::RGBA,
        x: f32,
        y: f32,
    ) -> Option<TextNode> {
        assert_initialized_main_thread!();
        let node: Option<RenderNode> = unsafe {
            from_glib_full(gsk_sys::gsk_text_node_new(
                font.as_ref().to_glib_none().0,
                glyphs.to_glib_none_mut().0,
                color.to_glib_none().0,
                x,
                y,
            ))
        };
        node.map(|node| node.try_into().unwrap())
    }

    pub fn peek_color(&self) -> gdk::RGBA {
        unsafe { from_glib_none(gsk_sys::gsk_text_node_peek_color(self.to_glib_none().0)) }
    }

    pub fn peek_font(&self) -> pango::Font {
        unsafe { from_glib_none(gsk_sys::gsk_text_node_peek_font(self.to_glib_none().0)) }
    }
}

impl TextureNode {
    pub fn get_texture(&self) -> gdk::Texture {
        unsafe { from_glib_none(gsk_sys::gsk_texture_node_get_texture(self.to_glib_none().0)) }
    }

    pub fn new<P: IsA<gdk::Texture>>(texture: &P, bounds: &graphene::Rect) -> RenderNode {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_texture_node_new(
                texture.as_ref().to_glib_none().0,
                bounds.to_glib_none().0,
            ))
        }
    }
}
