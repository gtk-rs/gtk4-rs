// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Calendar;
#[cfg(not(feature = "v4_14"))]
use glib::prelude::*;

impl Calendar {
    #[cfg(not(feature = "v4_14"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "v4_14"))))]
    pub fn day(&self) -> i32 {
        ObjectExt::property(self, "day")
    }

    #[cfg(not(feature = "v4_14"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "v4_14"))))]
    pub fn set_day(&self, day: i32) {
        ObjectExt::set_property(self, "day", day)
    }

    #[cfg(not(feature = "v4_14"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "v4_14"))))]
    pub fn month(&self) -> i32 {
        ObjectExt::property(self, "month")
    }

    #[cfg(not(feature = "v4_14"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "v4_14"))))]
    pub fn set_month(&self, month: i32) {
        ObjectExt::set_property(self, "month", month)
    }

    #[cfg(not(feature = "v4_14"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "v4_14"))))]
    pub fn year(&self) -> i32 {
        ObjectExt::property(self, "year")
    }

    #[cfg(not(feature = "v4_14"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "v4_14"))))]
    pub fn set_year(&self, year: i32) {
        ObjectExt::set_property(self, "year", year)
    }
}
