// Take a look at the license at the top of the repository in the LICENSE file.

#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;
pub use gdk;
pub use gio;
pub use glib;
#[cfg(any(feature = "win32", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "win32")))]
pub use windows;

#[macro_use]
mod rt;

#[allow(clippy::upper_case_acronyms)]
#[allow(unused_imports)]
mod auto;

pub mod builders;

pub use auto::*;

mod win32_display;
mod win32_hcursor;
mod win32_surface;

#[cfg(not(feature = "win32"))]
pub struct HANDLE(pub isize);
#[cfg(not(feature = "win32"))]
pub struct HCURSOR(pub isize);
#[cfg(not(feature = "win32"))]
pub struct HICON(pub isize);
#[cfg(not(feature = "win32"))]
pub struct HWND(pub isize);

#[cfg(feature = "win32")]
pub use windows::Win32::Foundation::{HANDLE, HWND};
#[cfg(feature = "win32")]
pub use windows::Win32::UI::WindowsAndMessaging::{HCURSOR, HICON};
