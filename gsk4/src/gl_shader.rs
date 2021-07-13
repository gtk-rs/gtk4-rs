use crate::GLShader;
use glib::translate::*;

impl GLShader {
    #[doc(alias = "gsk_gl_shader_get_arg_vec2")]
    #[doc(alias = "get_arg_vec2")]
    pub fn arg_vec2(&self, args: &glib::Bytes, idx: i32) -> graphene::Vec2 {
        unsafe {
            let out_value = std::ptr::null_mut();
            ffi::gsk_gl_shader_get_arg_vec2(
                self.to_glib_none().0,
                args.to_glib_none().0,
                idx,
                out_value,
            );
            from_glib_none(out_value)
        }
    }

    #[doc(alias = "gsk_gl_shader_get_arg_vec3")]
    #[doc(alias = "get_arg_vec3")]
    pub fn arg_vec3(&self, args: &glib::Bytes, idx: i32) -> graphene::Vec3 {
        unsafe {
            let out_value = std::ptr::null_mut();
            ffi::gsk_gl_shader_get_arg_vec3(
                self.to_glib_none().0,
                args.to_glib_none().0,
                idx,
                out_value,
            );
            from_glib_none(out_value)
        }
    }

    #[doc(alias = "gsk_gl_shader_get_arg_vec4")]
    #[doc(alias = "get_arg_vec4")]
    pub fn arg_vec4(&self, args: &glib::Bytes, idx: i32) -> graphene::Vec4 {
        unsafe {
            let out_value = std::ptr::null_mut();
            ffi::gsk_gl_shader_get_arg_vec4(
                self.to_glib_none().0,
                args.to_glib_none().0,
                idx,
                out_value,
            );
            from_glib_none(out_value)
        }
    }
}
