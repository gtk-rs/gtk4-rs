// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Bitset;
use glib::translate::*;
use std::marker::PhantomData;

#[derive(Copy, Clone)]
#[doc(alias = "GtkBitsetIter")]
#[repr(transparent)]
pub struct BitsetIter<'a>(ffi::GtkBitsetIter, PhantomData<&'a Bitset>);

impl<'a> BitsetIter<'a> {
    #[doc(alias = "gtk_bitset_iter_init_at")]
    pub fn init_at(set: &'a Bitset, target: u32) -> Option<(Self, u32)> {
        assert_initialized_main_thread!();
        unsafe {
            let mut iter = std::mem::MaybeUninit::uninit();
            let mut value = std::mem::MaybeUninit::uninit();
            let found: bool = from_glib(ffi::gtk_bitset_iter_init_at(
                iter.as_mut_ptr(),
                set.to_glib_none().0,
                target,
                value.as_mut_ptr(),
            ));

            if found {
                Some((
                    BitsetIter(iter.assume_init(), PhantomData),
                    value.assume_init(),
                ))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_bitset_iter_init_first")]
    pub fn init_first(set: &'a Bitset) -> Option<(Self, u32)> {
        assert_initialized_main_thread!();
        unsafe {
            let mut iter = std::mem::MaybeUninit::uninit();
            let mut value = std::mem::MaybeUninit::uninit();
            let found: bool = from_glib(ffi::gtk_bitset_iter_init_first(
                iter.as_mut_ptr(),
                set.to_glib_none().0,
                value.as_mut_ptr(),
            ));
            if found {
                Some((
                    BitsetIter(iter.assume_init(), PhantomData),
                    value.assume_init(),
                ))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_bitset_iter_init_last")]
    pub fn init_last(set: &'a Bitset) -> Option<(Self, u32)> {
        assert_initialized_main_thread!();
        unsafe {
            let mut iter = std::mem::MaybeUninit::uninit();
            let mut value = std::mem::MaybeUninit::uninit();
            let found: bool = from_glib(ffi::gtk_bitset_iter_init_last(
                iter.as_mut_ptr(),
                set.to_glib_none().0,
                value.as_mut_ptr(),
            ));
            if found {
                Some((
                    BitsetIter(iter.assume_init(), PhantomData),
                    value.assume_init(),
                ))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_bitset_iter_get_value")]
    pub fn value(&self) -> u32 {
        unsafe { ffi::gtk_bitset_iter_get_value(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_bitset_iter_is_valid")]
    pub fn is_valid(&self) -> bool {
        unsafe { from_glib(ffi::gtk_bitset_iter_is_valid(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_bitset_iter_previous")]
    pub fn previous(&mut self) -> Option<u32> {
        unsafe {
            let mut value = std::mem::MaybeUninit::uninit();

            let found: bool = from_glib(ffi::gtk_bitset_iter_previous(
                &mut self.0 as *mut _,
                value.as_mut_ptr(),
            ));
            if found {
                Some(value.assume_init())
            } else {
                None
            }
        }
    }
}

impl<'a> Iterator for BitsetIter<'a> {
    type Item = u32;

    #[doc(alias = "gtk_bitset_iter_next")]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut value = std::mem::MaybeUninit::uninit();

            let found: bool = from_glib(ffi::gtk_bitset_iter_next(
                &mut self.0 as *mut _,
                value.as_mut_ptr(),
            ));
            if found {
                Some(value.assume_init())
            } else {
                None
            }
        }
    }
}

impl<'a> std::iter::FusedIterator for BitsetIter<'a> {}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GtkBitsetIter> for BitsetIter<'a> {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<*const ffi::GtkBitsetIter, Self> {
        Stash(&self.0 as *const ffi::GtkBitsetIter, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as gtk4;

    #[test]
    fn test_bitset_iter() {
        let set = Bitset::new_range(0, 20);
        let (mut iter, init_value) = BitsetIter::init_first(&set).unwrap();
        assert_eq!(init_value, 0);
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.previous(), Some(0));

        let set2 = Bitset::new_range(0, 3);
        let (mut iter, init_val) = BitsetIter::init_last(&set2).unwrap();
        assert_eq!(init_val, 2);
        assert_eq!(iter.previous(), Some(1));
        assert_eq!(iter.previous(), Some(0));
        assert_eq!(iter.previous(), None);
        assert!(!iter.is_valid());
        assert_eq!(iter.next(), Some(1));
        assert!(iter.is_valid());
    }
}
