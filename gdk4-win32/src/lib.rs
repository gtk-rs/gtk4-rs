// Take a look at the license at the top of the repository in the LICENSE file.

#![doc = include_str!("../README.md")]
#![allow(deprecated)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(not(all(feature = "win32", windows)))]
use std::ffi::c_void;

pub use gdk;
pub use gdk4_win32_sys as ffi;
pub use gio;
pub use glib;
#[cfg(feature = "win32")]
#[cfg_attr(docsrs, doc(cfg(feature = "win32")))]
pub use windows;

#[macro_use]
mod rt;

mod enums;

#[allow(clippy::upper_case_acronyms)]
#[allow(unused_imports)]
mod auto;

pub mod prelude;

pub use auto::*;

mod win32_display;
pub use win32_display::Win32DisplayFilterHandle;

mod win32_hcursor;
mod win32_surface;

#[cfg(not(all(feature = "win32", windows)))]
pub struct HANDLE(pub *mut c_void);
#[cfg(not(all(feature = "win32", windows)))]
pub struct HCURSOR(pub *mut c_void);
#[cfg(not(all(feature = "win32", windows)))]
pub struct HICON(pub *mut c_void);
#[cfg(not(all(feature = "win32", windows)))]
pub struct HWND(pub *mut c_void);

#[cfg(not(all(feature = "win32", windows)))]
#[repr(transparent)]
pub struct WPARAM(pub usize);
#[cfg(not(all(feature = "win32", windows)))]
#[repr(transparent)]
pub struct LPARAM(pub isize);

#[cfg(not(all(feature = "win32", windows)))]
#[repr(C)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}

#[allow(non_snake_case)]
#[cfg(not(all(feature = "win32", windows)))]
#[repr(C)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: u32,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: u32,
    pub pt: POINT,
}

#[cfg(all(feature = "win32", windows))]
pub use windows::Win32::Foundation::{HANDLE, HWND};
#[cfg(all(feature = "win32", windows))]
pub use windows::Win32::UI::WindowsAndMessaging::{HCURSOR, HICON, MSG};
