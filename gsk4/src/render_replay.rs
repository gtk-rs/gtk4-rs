// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ffi, RenderNode};
use glib::{prelude::*, translate::*};
use std::boxed::Box as Box_;

#[repr(C)]
#[doc(alias = "GskRenderReplay")]
pub struct RenderReplay(std::ptr::NonNull<ffi::GskRenderReplay>);

impl Drop for RenderReplay {
    fn drop(&mut self) {
        unsafe {
            ffi::gsk_render_replay_free(self.0.as_ptr());
        }
    }
}

impl RenderReplay {
    #[doc(alias = "gsk_render_replay_default")]
    #[allow(clippy::should_implement_trait)]
    pub fn default(&self, node: impl AsRef<RenderNode>) -> Option<RenderNode> {
        unsafe {
            from_glib_full(ffi::gsk_render_replay_default(
                self.0.as_ptr(),
                node.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_render_replay_filter_font")]
    pub fn filter_font(&self, font: &impl IsA<pango::Font>) -> pango::Font {
        unsafe {
            from_glib_full(ffi::gsk_render_replay_filter_font(
                self.0.as_ptr(),
                font.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_render_replay_filter_node")]
    pub fn filter_node(&self, node: impl AsRef<RenderNode>) -> Option<RenderNode> {
        unsafe {
            from_glib_full(ffi::gsk_render_replay_filter_node(
                self.0.as_ptr(),
                node.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_render_replay_filter_texture")]
    pub fn filter_texture(&self, texture: &impl IsA<gdk::Texture>) -> gdk::Texture {
        unsafe {
            from_glib_full(ffi::gsk_render_replay_filter_texture(
                self.0.as_ptr(),
                texture.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_render_replay_foreach_node")]
    pub fn foreach_node(&self, node: impl AsRef<RenderNode>) {
        unsafe {
            ffi::gsk_render_replay_foreach_node(self.0.as_ptr(), node.as_ref().to_glib_none().0);
        }
    }

    #[allow(clippy::type_complexity)]
    #[doc(alias = "gsk_render_replay_set_font_filter")]
    pub fn set_font_filter(
        &mut self,
        filter: Option<Box_<dyn Fn(&RenderReplay, &pango::Font) -> pango::Font + 'static>>,
    ) {
        let filter_data: Box_<
            Option<Box_<dyn Fn(&RenderReplay, &pango::Font) -> pango::Font + 'static>>,
        > = Box_::new(filter);
        unsafe extern "C" fn filter_func(
            replay: *mut ffi::GskRenderReplay,
            font: *mut pango::ffi::PangoFont,
            user_data: glib::ffi::gpointer,
        ) -> *mut pango::ffi::PangoFont {
            let replay = RenderReplay(std::ptr::NonNull::new_unchecked(replay));
            let font = from_glib_borrow(font);
            let callback = &*(user_data
                as *mut Option<Box_<dyn Fn(&RenderReplay, &pango::Font) -> pango::Font + 'static>>);
            if let Some(ref callback) = *callback {
                callback(&replay, &font)
            } else {
                panic!("cannot get closure...")
            }
            .into_glib_ptr()
        }
        let filter = if filter_data.is_some() {
            Some(filter_func as _)
        } else {
            None
        };
        unsafe extern "C" fn user_destroy_func(data: glib::ffi::gpointer) {
            let _callback = Box_::from_raw(
                data as *mut Option<
                    Box_<dyn Fn(&RenderReplay, &pango::Font) -> pango::Font + 'static>,
                >,
            );
        }
        let destroy_call3 = Some(user_destroy_func as _);
        let super_callback0: Box_<
            Option<Box_<dyn Fn(&RenderReplay, &pango::Font) -> pango::Font + 'static>>,
        > = filter_data;
        unsafe {
            ffi::gsk_render_replay_set_font_filter(
                self.0.as_mut(),
                filter,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    #[doc(alias = "gsk_render_replay_set_node_filter")]
    pub fn set_node_filter<P: Fn(&RenderReplay, &RenderNode) -> Option<RenderNode> + 'static>(
        &mut self,
        filter: P,
    ) {
        let filter_data: Box_<P> = Box_::new(filter);
        unsafe extern "C" fn filter_func<
            P: Fn(&RenderReplay, &RenderNode) -> Option<RenderNode> + 'static,
        >(
            replay: *mut ffi::GskRenderReplay,
            node: *mut ffi::GskRenderNode,
            user_data: glib::ffi::gpointer,
        ) -> *mut ffi::GskRenderNode {
            let replay = RenderReplay(std::ptr::NonNull::new_unchecked(replay));
            let node = from_glib_borrow(node);
            let callback = &*(user_data as *mut P);
            (*callback)(&replay, &node).into_glib_ptr()
        }
        let filter = Some(filter_func::<P> as _);
        unsafe extern "C" fn user_destroy_func<
            P: Fn(&RenderReplay, &RenderNode) -> Option<RenderNode> + 'static,
        >(
            data: glib::ffi::gpointer,
        ) {
            let _callback = Box_::from_raw(data as *mut P);
        }
        let destroy_call3 = Some(user_destroy_func::<P> as _);
        let super_callback0: Box_<P> = filter_data;
        unsafe {
            ffi::gsk_render_replay_set_node_filter(
                self.0.as_mut(),
                filter,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    #[doc(alias = "gsk_render_replay_set_node_foreach")]
    pub fn set_node_foreach<P: Fn(&RenderReplay, &RenderNode) -> glib::ControlFlow + 'static>(
        &mut self,
        foreach: P,
    ) {
        let foreach_data: Box_<P> = Box_::new(foreach);
        unsafe extern "C" fn foreach_func<
            P: Fn(&RenderReplay, &RenderNode) -> glib::ControlFlow + 'static,
        >(
            replay: *mut ffi::GskRenderReplay,
            node: *mut ffi::GskRenderNode,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let replay = RenderReplay(std::ptr::NonNull::new_unchecked(replay));
            let node = from_glib_borrow(node);
            let callback = &*(user_data as *mut P);
            (*callback)(&replay, &node).into_glib()
        }
        let foreach = Some(foreach_func::<P> as _);
        unsafe extern "C" fn user_destroy_func<
            P: Fn(&RenderReplay, &RenderNode) -> glib::ControlFlow + 'static,
        >(
            data: glib::ffi::gpointer,
        ) {
            let _callback = Box_::from_raw(data as *mut P);
        }
        let destroy_call3 = Some(user_destroy_func::<P> as _);
        let super_callback0: Box_<P> = foreach_data;
        unsafe {
            ffi::gsk_render_replay_set_node_foreach(
                self.0.as_mut(),
                foreach,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    #[allow(clippy::type_complexity)]
    #[doc(alias = "gsk_render_replay_set_texture_filter")]
    pub fn set_texture_filter(
        &mut self,
        filter: Option<Box_<dyn Fn(&RenderReplay, &gdk::Texture) -> gdk::Texture + 'static>>,
    ) {
        let filter_data: Box_<
            Option<Box_<dyn Fn(&RenderReplay, &gdk::Texture) -> gdk::Texture + 'static>>,
        > = Box_::new(filter);
        unsafe extern "C" fn filter_func(
            replay: *mut ffi::GskRenderReplay,
            texture: *mut gdk::ffi::GdkTexture,
            user_data: glib::ffi::gpointer,
        ) -> *mut gdk::ffi::GdkTexture {
            let replay = RenderReplay(std::ptr::NonNull::new_unchecked(replay));
            let texture = from_glib_borrow(texture);
            let callback = &*(user_data
                as *mut Option<
                    Box_<dyn Fn(&RenderReplay, &gdk::Texture) -> gdk::Texture + 'static>,
                >);
            if let Some(ref callback) = *callback {
                callback(&replay, &texture)
            } else {
                panic!("cannot get closure...")
            }
            .into_glib_ptr()
        }
        let filter = if filter_data.is_some() {
            Some(filter_func as _)
        } else {
            None
        };
        unsafe extern "C" fn user_destroy_func(data: glib::ffi::gpointer) {
            let _callback = Box_::from_raw(
                data as *mut Option<
                    Box_<dyn Fn(&RenderReplay, &gdk::Texture) -> gdk::Texture + 'static>,
                >,
            );
        }
        let destroy_call3 = Some(user_destroy_func as _);
        let super_callback0: Box_<
            Option<Box_<dyn Fn(&RenderReplay, &gdk::Texture) -> gdk::Texture + 'static>>,
        > = filter_data;
        unsafe {
            ffi::gsk_render_replay_set_texture_filter(
                self.0.as_mut(),
                filter,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    #[doc(alias = "gsk_render_replay_new")]
    pub fn new() -> RenderReplay {
        assert_initialized_main_thread!();
        unsafe {
            RenderReplay(std::ptr::NonNull::new_unchecked(
                ffi::gsk_render_replay_new(),
            ))
        }
    }
}

#[cfg(feature = "v4_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_22")))]
impl Default for RenderReplay {
    fn default() -> Self {
        Self::new()
    }
}
