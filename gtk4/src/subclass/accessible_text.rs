// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`AccessibleText`] interface.

use crate::{
    ffi, subclass::prelude::*, AccessibleText, AccessibleTextGranularity, AccessibleTextRange,
};
use glib::object::Cast;
use glib::{translate::*, GString};

pub trait AccessibleTextImpl: WidgetImpl {
    #[doc(alias = "get_attributes")]
    fn attributes(&self, offset: u32) -> Vec<(AccessibleTextRange, GString, GString)> {
        self.parent_attributes(offset)
    }

    #[doc(alias = "get_caret_position")]
    fn caret_position(&self) -> u32 {
        self.parent_caret_position()
    }

    #[doc(alias = "get_contents")]
    fn contents(&self, start: u32, end: u32) -> Option<glib::Bytes> {
        self.parent_contents(start, end)
    }

    #[doc(alias = "get_contents_at")]
    fn contents_at(
        &self,
        offset: u32,
        granularity: crate::AccessibleTextGranularity,
    ) -> Option<(u32, u32, glib::Bytes)> {
        self.parent_contents_at(offset, granularity)
    }

    #[doc(alias = "get_default_attributes")]
    fn default_attributes(&self) -> Vec<(GString, GString)> {
        self.parent_default_attributes()
    }

    #[cfg(feature = "v4_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_16")))]
    #[doc(alias = "get_extents")]
    fn extents(&self, start: u32, end: u32) -> Option<graphene::Rect> {
        self.parent_extents(start, end)
    }

    #[cfg(feature = "v4_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_16")))]
    #[doc(alias = "get_offset")]
    fn offset(&self, point: &graphene::Point) -> Option<u32> {
        self.parent_offset(point)
    }

    #[doc(alias = "get_selection")]
    fn selection(&self) -> Vec<AccessibleTextRange> {
        self.parent_selection()
    }
}

pub trait AccessibleTextImplExt: AccessibleTextImpl {
    fn parent_attributes(&self, offset: u32) -> Vec<(AccessibleTextRange, GString, GString)> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<AccessibleText>()
                as *const ffi::GtkAccessibleTextInterface;

            let func = (*parent_iface)
                .get_attributes
                .expect("no parent \"get_attributes\" implementation");

            let mut n_ranges = std::mem::MaybeUninit::uninit();
            let mut ranges = std::ptr::null_mut();
            let mut attribute_names = std::ptr::null_mut();
            let mut attribute_values = std::ptr::null_mut();

            let is_set: bool = from_glib(func(
                self.obj()
                    .unsafe_cast_ref::<AccessibleText>()
                    .to_glib_none()
                    .0,
                offset,
                n_ranges.as_mut_ptr(),
                &mut ranges,
                &mut attribute_names,
                &mut attribute_values,
            ));

            if !is_set
                || n_ranges.assume_init() == 0
                || ranges.is_null()
                || attribute_names.is_null()
                || attribute_values.is_null()
            {
                Vec::new()
            } else {
                let mut names = glib::StrV::from_glib_full(attribute_names).into_iter();
                let mut values = glib::StrV::from_glib_full(attribute_values).into_iter();

                glib::Slice::from_glib_container_num(ranges, n_ranges.assume_init())
                    .into_iter()
                    .flat_map(|range| {
                        if let (Some(name), Some(value)) = (names.next(), values.next()) {
                            Some((range, name, value))
                        } else {
                            None
                        }
                    })
                    .collect()
            }
        }
    }

    fn parent_caret_position(&self) -> u32 {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<AccessibleText>()
                as *const ffi::GtkAccessibleTextInterface;

            let func = (*parent_iface)
                .get_caret_position
                .expect("no parent \"get_caret_position\" implementation");

            func(
                self.obj()
                    .unsafe_cast_ref::<AccessibleText>()
                    .to_glib_none()
                    .0,
            )
        }
    }

    fn parent_contents(&self, start: u32, end: u32) -> Option<glib::Bytes> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<AccessibleText>()
                as *const ffi::GtkAccessibleTextInterface;

            let func = (*parent_iface).get_contents?;

            from_glib_full(func(
                self.obj()
                    .unsafe_cast_ref::<AccessibleText>()
                    .to_glib_none()
                    .0,
                start,
                end,
            ))
        }
    }

    fn parent_contents_at(
        &self,
        offset: u32,
        granularity: crate::AccessibleTextGranularity,
    ) -> Option<(u32, u32, glib::Bytes)> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<AccessibleText>()
                as *const ffi::GtkAccessibleTextInterface;

            let func = (*parent_iface).get_contents_at?;

            let mut start = std::mem::MaybeUninit::uninit();
            let mut end = std::mem::MaybeUninit::uninit();

            let bytes = func(
                self.obj()
                    .unsafe_cast_ref::<AccessibleText>()
                    .to_glib_none()
                    .0,
                offset,
                granularity.into_glib(),
                start.as_mut_ptr(),
                end.as_mut_ptr(),
            );

            if !bytes.is_null() {
                Some((
                    start.assume_init(),
                    end.assume_init(),
                    from_glib_full(bytes),
                ))
            } else {
                None
            }
        }
    }

    fn parent_default_attributes(&self) -> Vec<(GString, GString)> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<AccessibleText>()
                as *const ffi::GtkAccessibleTextInterface;

            let func = (*parent_iface)
                .get_default_attributes
                .expect("no parent \"get_default_attributes\" implementation");

            let mut attribute_names = std::ptr::null_mut();
            let mut attribute_values = std::ptr::null_mut();

            func(
                self.obj()
                    .unsafe_cast_ref::<AccessibleText>()
                    .to_glib_none()
                    .0,
                &mut attribute_names,
                &mut attribute_values,
            );

            if attribute_names.is_null() || attribute_values.is_null() {
                Vec::new()
            } else {
                glib::StrV::from_glib_full(attribute_names)
                    .into_iter()
                    .zip(glib::StrV::from_glib_full(attribute_values))
                    .collect()
            }
        }
    }

    #[cfg(feature = "v4_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_16")))]
    fn parent_extents(&self, start: u32, end: u32) -> Option<graphene::Rect> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<AccessibleText>()
                as *const ffi::GtkAccessibleTextInterface;

            let func = (*parent_iface)
                .get_extents
                .expect("no parent \"get_extents\" implementation");

            let mut extents = std::mem::MaybeUninit::uninit();

            let filled = from_glib(func(
                self.obj()
                    .unsafe_cast_ref::<AccessibleText>()
                    .to_glib_none()
                    .0,
                start,
                end,
                extents.as_mut_ptr(),
            ));

            if filled {
                Some(graphene::Rect::unsafe_from(extents.assume_init()))
            } else {
                None
            }
        }
    }

    #[cfg(feature = "v4_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_16")))]
    fn parent_offset(&self, point: &graphene::Point) -> Option<u32> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<AccessibleText>()
                as *const ffi::GtkAccessibleTextInterface;

            let func = (*parent_iface)
                .get_offset
                .expect("no parent \"get_offset\" implementation");

            let mut offset = std::mem::MaybeUninit::uninit();

            let offset_set = from_glib(func(
                self.obj()
                    .unsafe_cast_ref::<AccessibleText>()
                    .to_glib_none()
                    .0,
                point.to_glib_none().0,
                offset.as_mut_ptr(),
            ));

            if offset_set {
                Some(offset.assume_init())
            } else {
                None
            }
        }
    }

    fn parent_selection(&self) -> Vec<AccessibleTextRange> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<AccessibleText>()
                as *const ffi::GtkAccessibleTextInterface;

            let func = (*parent_iface)
                .get_selection
                .expect("no parent \"get_selection\" implementation");

            let mut n_ranges = std::mem::MaybeUninit::uninit();
            let mut ranges = std::ptr::null_mut();

            let valid = from_glib(func(
                self.obj()
                    .unsafe_cast_ref::<AccessibleText>()
                    .to_glib_none()
                    .0,
                n_ranges.as_mut_ptr(),
                &mut ranges,
            ));

            if valid {
                let n = n_ranges.assume_init();
                AccessibleTextRange::from_glib_container_num_as_vec(ranges, n)
            } else {
                Vec::new()
            }
        }
    }
}

impl<T: AccessibleTextImpl> AccessibleTextImplExt for T {}

unsafe impl<T: AccessibleTextImpl> IsImplementable<T> for AccessibleText {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.get_contents = Some(accessible_text_get_contents::<T>);
        iface.get_contents_at = Some(accessible_text_get_contents_at::<T>);
        iface.get_caret_position = Some(accessible_text_get_caret_position::<T>);
        iface.get_selection = Some(accessible_text_get_selection::<T>);
        iface.get_attributes = Some(accessible_text_get_attributes::<T>);
        iface.get_default_attributes = Some(accessible_text_get_default_attributes::<T>);

        #[cfg(feature = "v4_16")]
        {
            iface.get_extents = Some(accessible_text_get_extents::<T>);
            iface.get_offset = Some(accessible_text_get_offset::<T>);
        }
    }
}

unsafe extern "C" fn accessible_text_get_contents<T: AccessibleTextImpl>(
    accessible_text: *mut ffi::GtkAccessibleText,
    start: u32,
    end: u32,
) -> *mut glib::ffi::GBytes {
    let instance = &*(accessible_text as *mut T::Instance);
    let imp = instance.imp();

    let contents = imp.contents(start, end);
    contents.into_glib_ptr()
}

unsafe extern "C" fn accessible_text_get_contents_at<T: AccessibleTextImpl>(
    accessible_text: *mut ffi::GtkAccessibleText,
    offset: libc::c_uint,
    granularity: ffi::GtkAccessibleTextGranularity,
    start: *mut libc::c_uint,
    end: *mut libc::c_uint,
) -> *mut glib::ffi::GBytes {
    let instance = &*(accessible_text as *mut T::Instance);
    let imp = instance.imp();

    if let Some((r_start, r_end, bytes)) =
        imp.contents_at(offset, AccessibleTextGranularity::from_glib(granularity))
    {
        if !start.is_null() {
            *start = r_start;
        }
        if !end.is_null() {
            *end = r_end;
        }

        bytes.into_glib_ptr()
    } else {
        std::ptr::null_mut()
    }
}

unsafe extern "C" fn accessible_text_get_caret_position<T: AccessibleTextImpl>(
    accessible_text: *mut ffi::GtkAccessibleText,
) -> u32 {
    let instance = &*(accessible_text as *mut T::Instance);
    let imp = instance.imp();

    imp.caret_position()
}

unsafe extern "C" fn accessible_text_get_selection<T: AccessibleTextImpl>(
    accessible_text: *mut ffi::GtkAccessibleText,
    n_ranges: *mut libc::size_t,
    ranges: *mut *mut ffi::GtkAccessibleTextRange,
) -> glib::ffi::gboolean {
    let instance = &*(accessible_text as *mut T::Instance);
    let imp = instance.imp();

    let r_ranges = imp.selection();
    let n: usize = r_ranges.len();
    *n_ranges = n;

    if n == 0 {
        false
    } else {
        *ranges = r_ranges.to_glib_container().0;

        true
    }
    .into_glib()
}

unsafe extern "C" fn accessible_text_get_attributes<T: AccessibleTextImpl>(
    accessible_text: *mut ffi::GtkAccessibleText,
    offset: u32,
    n_ranges: *mut libc::size_t,
    ranges: *mut *mut ffi::GtkAccessibleTextRange,
    attribute_names: *mut *mut *mut libc::c_char,
    attribute_values: *mut *mut *mut libc::c_char,
) -> glib::ffi::gboolean {
    let instance = &*(accessible_text as *mut T::Instance);
    let imp = instance.imp();

    let attrs = imp.attributes(offset);
    let n: usize = attrs.len();
    *n_ranges = n;

    if n == 0 {
        *attribute_names = std::ptr::null_mut();
        *attribute_values = std::ptr::null_mut();

        false
    } else {
        let mut c_ranges = glib::Slice::with_capacity(attrs.len());
        let mut c_names = glib::StrV::with_capacity(attrs.len());
        let mut c_values = glib::StrV::with_capacity(attrs.len());

        for (range, name, value) in attrs {
            c_ranges.push(range);
            c_names.push(name);
            c_values.push(value);
        }

        *ranges = c_ranges.to_glib_container().0;
        *attribute_names = c_names.into_glib_ptr();
        *attribute_values = c_values.into_glib_ptr();

        true
    }
    .into_glib()
}

unsafe extern "C" fn accessible_text_get_default_attributes<T: AccessibleTextImpl>(
    accessible_text: *mut ffi::GtkAccessibleText,
    attribute_names: *mut *mut *mut libc::c_char,
    attribute_values: *mut *mut *mut libc::c_char,
) {
    let instance = &*(accessible_text as *mut T::Instance);
    let imp = instance.imp();

    let attrs = imp.default_attributes();

    if attrs.is_empty() {
        *attribute_names = std::ptr::null_mut();
        *attribute_values = std::ptr::null_mut();
    } else {
        let mut c_names = glib::StrV::with_capacity(attrs.len());
        let mut c_values = glib::StrV::with_capacity(attrs.len());

        for (name, value) in attrs {
            c_names.push(name);
            c_values.push(value);
        }

        *attribute_names = c_names.into_glib_ptr();
        *attribute_values = c_values.into_glib_ptr();
    }
}

#[cfg(feature = "v4_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_16")))]
unsafe extern "C" fn accessible_text_get_extents<T: AccessibleTextImpl>(
    accessible_text: *mut ffi::GtkAccessibleText,
    start: u32,
    end: u32,
    extents: *mut graphene::ffi::graphene_rect_t,
) -> glib::ffi::gboolean {
    let instance = &*(accessible_text as *mut T::Instance);
    let imp = instance.imp();

    let rect = imp.extents(start, end);

    if let Some(rect) = rect {
        *extents = *rect.as_ptr();

        true
    } else {
        false
    }
    .into_glib()
}

#[cfg(feature = "v4_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_16")))]
unsafe extern "C" fn accessible_text_get_offset<T: AccessibleTextImpl>(
    accessible_text: *mut ffi::GtkAccessibleText,
    point: *const graphene::ffi::graphene_point_t,
    offset: *mut libc::c_uint,
) -> glib::ffi::gboolean {
    let instance = &*(accessible_text as *mut T::Instance);
    let imp = instance.imp();

    let pos = imp.offset(&from_glib_borrow(point));

    if let Some(pos) = pos {
        if !offset.is_null() {
            *offset = pos;
        }
        true
    } else {
        false
    }
    .into_glib()
}

#[cfg(test)]
mod test {
    use crate as gtk4;
    use crate::prelude::*;
    use crate::subclass::prelude::*;

    mod imp {
        use super::*;

        #[derive(Default)]
        pub struct TestTextView {}

        #[glib::object_subclass]
        impl ObjectSubclass for TestTextView {
            const NAME: &'static str = "TestTextView";
            type Type = super::TestTextView;
            type ParentType = crate::TextView;
            type Interfaces = (crate::AccessibleText,);
        }

        impl ObjectImpl for TestTextView {}
        impl WidgetImpl for TestTextView {}
        impl AccessibleTextImpl for TestTextView {
            fn attributes(
                &self,
                offset: u32,
            ) -> Vec<(
                crate::accessible_text_range::AccessibleTextRange,
                glib::GString,
                glib::GString,
            )> {
                self.parent_attributes(offset)
            }

            fn caret_position(&self) -> u32 {
                self.parent_caret_position()
            }

            fn contents(&self, start: u32, end: u32) -> Option<glib::Bytes> {
                self.parent_contents(start, end)
            }

            fn contents_at(
                &self,
                offset: u32,
                granularity: crate::AccessibleTextGranularity,
            ) -> Option<(u32, u32, glib::Bytes)> {
                self.parent_contents_at(offset, granularity)
            }

            fn default_attributes(&self) -> Vec<(glib::GString, glib::GString)> {
                self.parent_default_attributes()
            }

            fn selection(&self) -> Vec<crate::accessible_text_range::AccessibleTextRange> {
                self.parent_selection()
            }

            #[cfg(feature = "v4_16")]
            fn extents(&self, start: u32, end: u32) -> Option<graphene::Rect> {
                self.parent_extents(start, end)
            }

            #[cfg(feature = "v4_16")]
            fn offset(&self, point: &graphene::Point) -> Option<u32> {
                self.parent_offset(point)
            }
        }

        impl TextViewImpl for TestTextView {}
        impl TestTextView {}
    }

    glib::wrapper! {
        pub struct TestTextView(ObjectSubclass<imp::TestTextView>)
        @extends crate::Widget, crate::TextView,
        @implements crate::Accessible, crate::AccessibleText, crate::Buildable, crate::ConstraintTarget, crate::Scrollable;
    }

    impl TestTextView {}

    #[crate::test]
    fn test_accessible_text_iface() {
        let text: TestTextView = glib::Object::new();
        let mut iter = text.buffer().iter_at_offset(0);
        text.buffer()
            .insert_markup(&mut iter, "<b>Lorem Ipsum</b> dolor <i>sit.</i> amnet");

        let (range, _, value) = text
            .imp()
            .attributes(0)
            .into_iter()
            .find(|(_, name, _)| name == "weight")
            .unwrap();

        assert_eq!(range.start(), 0);
        assert_eq!(range.length(), "Lorem Ipsum".len());
        assert_eq!(value, "700");

        assert_eq!(
            text.imp().caret_position(),
            "Lorem Ipsum dolor sit. amnet".len() as u32
        );
        let pos = "Lorem Ipsum ".len();
        let iter = text.buffer().iter_at_offset(pos as i32);
        text.buffer().place_cursor(&iter);
        assert_eq!(text.imp().caret_position(), pos as u32);

        assert_eq!(
            std::str::from_utf8(
                &text
                    .imp()
                    .contents_at(pos as u32, crate::AccessibleTextGranularity::Character)
                    .unwrap()
                    .2
            )
            .unwrap(),
            "d"
        );
        assert_eq!(
            std::str::from_utf8(
                &text
                    .imp()
                    .contents_at(pos as u32, crate::AccessibleTextGranularity::Word)
                    .unwrap()
                    .2
            )
            .unwrap(),
            "dolor "
        );
        assert_eq!(
            std::str::from_utf8(
                &text
                    .imp()
                    .contents_at(pos as u32, crate::AccessibleTextGranularity::Line)
                    .unwrap()
                    .2
            )
            .unwrap(),
            "Lorem Ipsum dolor sit. amnet"
        );

        assert_eq!(
            "Lorem Ipsum\0",
            std::str::from_utf8(&text.imp().contents(0, 11).unwrap()).unwrap()
        );

        assert!(text
            .imp()
            .default_attributes()
            .iter()
            .any(|(name, value)| name == "editable" && value == "true"));
        text.buffer().select_range(
            &text.buffer().iter_at_offset(0),
            &text.buffer().iter_at_offset(10),
        );
        let selected_range = text.imp().selection()[0];
        assert_eq!(selected_range.start(), 0);
        assert_eq!(selected_range.length(), 10);

        #[cfg(feature = "v4_16")]
        {
            let _extents = text.imp().extents(0, 20);
            let _offset = text.imp().offset(&graphene::Point::new(10.0, 10.0));
        }
    }
}
