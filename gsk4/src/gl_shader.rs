use glib::translate::*;

impl GLShader {
    #[doc(alias = "gsk_gl_shader_get_arg_vec2")]
    #[doc(alias = "get_arg_vec2")]
    pub fn arg_vec2(&self, args: &glib::Bytes, idx: i32) -> Option<graphene::Vec2> {
        unsafe {
            let mut out_value = std::ptr::null_mut();
            let ret = ffi::gsk_gl_shader_get_arg_vec2(
                self.to_glib_none().0,
                args.to_glib_none().0,
                idx,
                out_value.to_glib_none_mut().0,
            );
            if from_glib(ret) {
                Some(out_value)
            } else {
                None
            }
        }
    }

    #[doc(alias = "gsk_gl_shader_get_arg_vec3")]
    #[doc(alias = "get_arg_vec3")]
    pub fn arg_vec3(&self, args: &glib::Bytes, idx: i32) -> Option<graphene::Vec3> {
        unsafe {
            let mut out_value = std::ptr::null_mut();
            let ret = ffi::gsk_gl_shader_get_arg_vec3(
                self.to_glib_none().0,
                args.to_glib_none().0,
                idx,
                out_value.to_glib_none_mut().0,
            );
            if from_glib(ret) {
                Some(out_value)
            } else {
                None
            }
        }
    }

    #[doc(alias = "gsk_gl_shader_get_arg_vec4")]
    #[doc(alias = "get_arg_vec4")]
    pub fn arg_vec4(&self, args: &glib::Bytes, idx: i32) -> Option<graphene::Vec4> {
        unsafe {
            let mut out_value = std::ptr::null_mut();
            let ret = ffi::gsk_gl_shader_get_arg_vec4(
                self.to_glib_none().0,
                args.to_glib_none().0,
                idx,
                out_value.to_glib_none_mut().0,
            );
            if from_glib(ret) {
                Some(out_value)
            } else {
                None
            }
        }
    }

}