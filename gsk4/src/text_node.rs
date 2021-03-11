// Take a look at the license at the top of the repository in the LICENSE file.

use crate::RenderNodeType;
use glib::object::IsA;
use glib::translate::*;
use std::mem;

define_render_node!(
    TextNode,
    ffi::GskTextNode,
    ffi::gsk_text_node_get_type,
    RenderNodeType::TextNode
);

impl TextNode {
    #[doc(alias = "gsk_text_node_new")]
    pub fn new<P: IsA<pango::Font>>(
        font: &P,
        glyphs: &mut pango::GlyphString,
        color: &gdk::RGBA,
        offset: &graphene::Point,
    ) -> Option<TextNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gsk_text_node_new(
                font.as_ref().to_glib_none().0,
                glyphs.to_glib_none_mut().0,
                color.to_glib_none().0,
                offset.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_text_node_get_color")]
    pub fn get_color(&self) -> Option<gdk::RGBA> {
        unsafe { from_glib_none(ffi::gsk_text_node_get_color(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_text_node_get_font")]
    pub fn get_font(&self) -> Option<pango::Font> {
        unsafe { from_glib_none(ffi::gsk_text_node_get_font(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_text_node_get_glyphs")]
    pub fn get_glyphs(&self) -> Vec<pango::GlyphInfo> {
        unsafe {
            let mut n_glyphs = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_none_num(
                ffi::gsk_text_node_get_glyphs(self.to_glib_none().0, n_glyphs.as_mut_ptr()),
                n_glyphs.assume_init() as usize,
            );
            ret
        }
    }

    #[doc(alias = "gsk_text_node_get_num_glyphs")]
    pub fn get_num_glyphs(&self) -> u32 {
        unsafe { ffi::gsk_text_node_get_num_glyphs(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_text_node_get_offset")]
    pub fn get_offset(&self) -> Option<graphene::Point> {
        unsafe { from_glib_none(ffi::gsk_text_node_get_offset(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_text_node_has_color_glyphs")]
    pub fn has_color_glyphs(&self) -> bool {
        unsafe { from_glib(ffi::gsk_text_node_has_color_glyphs(self.to_glib_none().0)) }
    }
}
