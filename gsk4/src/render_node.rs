// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ParseLocation, RenderNode, RenderNodeType};
use glib::translate::*;
use glib::StaticType;

impl RenderNode {
    #[inline]
    pub fn is<T: IsRenderNode>(&self) -> bool {
        T::NODE_TYPE == self.node_type()
    }

    #[inline]
    pub fn type_(&self) -> glib::Type {
        unsafe {
            let ptr = self.as_ptr();
            from_glib((*(*(ptr as *mut glib::gobject_ffi::GTypeInstance)).g_class).g_type)
        }
    }

    #[doc(alias = "gsk_render_node_deserialize")]
    pub fn deserialize(bytes: &glib::Bytes) -> Option<Self> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gsk_render_node_deserialize(
                bytes.to_glib_none().0,
                None,
                std::ptr::null_mut(),
            ))
        }
    }

    #[doc(alias = "gsk_render_node_deserialize")]
    pub fn deserialize_with_error_func<P: FnMut(&ParseLocation, &ParseLocation, &glib::Error)>(
        bytes: &glib::Bytes,
        error_func: P,
    ) -> Option<Self> {
        assert_initialized_main_thread!();
        let error_func_data: P = error_func;
        unsafe extern "C" fn error_func_func<
            P: FnMut(&ParseLocation, &ParseLocation, &glib::Error),
        >(
            start: *const ffi::GskParseLocation,
            end: *const ffi::GskParseLocation,
            error: *const glib::ffi::GError,
            user_data: glib::ffi::gpointer,
        ) {
            let start = from_glib_borrow(start);
            let end = from_glib_borrow(end);
            let error = from_glib_borrow(error);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            (*callback)(&start, &end, &error);
        }
        let error_func = Some(error_func_func::<P> as _);
        let super_callback0: &P = &error_func_data;
        unsafe {
            from_glib_full(ffi::gsk_render_node_deserialize(
                bytes.to_glib_none().0,
                error_func,
                super_callback0 as *const _ as usize as *mut _,
            ))
        }
    }

    #[inline]
    pub fn downcast<T: IsRenderNode>(self) -> Result<T, Self> {
        unsafe {
            if self.is::<T>() {
                Ok(from_glib_full(self.into_glib_ptr()))
            } else {
                Err(self)
            }
        }
    }

    #[inline]
    pub fn downcast_ref<T: IsRenderNode>(&self) -> Option<&T> {
        unsafe {
            if self.is::<T>() {
                Some(&*(self as *const RenderNode as *const T))
            } else {
                None
            }
        }
    }
}

impl std::fmt::Debug for RenderNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RenderNode")
            .field("bounds", &self.bounds())
            .field("node_type", &self.node_type())
            .finish()
    }
}

// rustdoc-stripper-ignore-next
/// A common trait implemented by the various [`RenderNode`](crate::RenderNode) types.
///
/// # Safety
///
/// The user is not supposed to implement this trait.
pub unsafe trait IsRenderNode:
    StaticType
    + FromGlibPtrFull<*mut ffi::GskRenderNode>
    + std::convert::AsRef<crate::RenderNode>
    + 'static
{
    const NODE_TYPE: RenderNodeType;
    fn upcast(self) -> RenderNode;
    fn upcast_ref(&self) -> &RenderNode;
}

#[doc(hidden)]
impl AsRef<RenderNode> for RenderNode {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

macro_rules! define_render_node {
    ($rust_type:ident, $ffi_type:path, $node_type:path) => {
        impl std::convert::AsRef<crate::RenderNode> for $rust_type {
            #[inline]
            fn as_ref(&self) -> &crate::RenderNode {
                self
            }
        }

        impl std::ops::Deref for $rust_type {
            type Target = crate::RenderNode;

            #[inline]
            fn deref(&self) -> &Self::Target {
                unsafe { &*(self as *const $rust_type as *const crate::RenderNode) }
            }
        }

        unsafe impl crate::render_node::IsRenderNode for $rust_type {
            const NODE_TYPE: RenderNodeType = $node_type;

            #[inline]
            fn upcast(self) -> crate::RenderNode {
                unsafe {
                    glib::translate::from_glib_full(
                        glib::translate::IntoGlibPtr::<*mut $ffi_type>::into_glib_ptr(self)
                            as *mut ffi::GskRenderNode,
                    )
                }
            }

            #[inline]
            fn upcast_ref(&self) -> &crate::RenderNode {
                self
            }
        }

        #[doc(hidden)]
        impl glib::translate::FromGlibPtrFull<*mut ffi::GskRenderNode> for $rust_type {
            #[inline]
            unsafe fn from_glib_full(ptr: *mut ffi::GskRenderNode) -> Self {
                glib::translate::from_glib_full(ptr as *mut $ffi_type)
            }
        }

        #[cfg(any(feature = "v4_6", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
        impl glib::value::ValueType for $rust_type {
            type Type = Self;
        }

        #[cfg(any(feature = "v4_6", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
        unsafe impl<'a> glib::value::FromValue<'a> for $rust_type {
            type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

            #[inline]
            unsafe fn from_value(value: &'a glib::Value) -> Self {
                skip_assert_initialized!();
                glib::translate::from_glib_full(ffi::gsk_value_dup_render_node(
                    glib::translate::ToGlibPtr::to_glib_none(value).0,
                ))
            }
        }

        #[cfg(any(feature = "v4_6", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
        impl glib::value::ToValue for $rust_type {
            #[inline]
            fn to_value(&self) -> glib::Value {
                let mut value = glib::Value::for_value_type::<Self>();
                unsafe {
                    ffi::gsk_value_set_render_node(
                        glib::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        self.as_ptr() as *mut _,
                    )
                }
                value
            }

            #[inline]
            fn value_type(&self) -> glib::Type {
                use glib::StaticType;
                Self::static_type()
            }
        }

        #[cfg(any(feature = "v4_6", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
        impl glib::value::ToValueOptional for $rust_type {
            #[inline]
            fn to_value_optional(s: Option<&Self>) -> glib::Value {
                skip_assert_initialized!();
                let mut value = glib::Value::for_value_type::<Self>();
                unsafe {
                    ffi::gsk_value_set_render_node(
                        glib::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        s.map(|s| s.as_ptr()).unwrap_or(std::ptr::null_mut()) as *mut _,
                    )
                }
                value
            }
        }
    };
}
