use gdk;
use gdk_sys;
use glib::translate::*;

pub fn targets_include_image(targets: &[&gdk::Atom], writable: bool) -> bool {
    assert_initialized_main_thread!();
    let n_targets = targets.len() as i32;
    let targets: Vec<_> = targets.iter().map(|t| ToGlibPtr::<gdk_sys::GdkAtom>::to_glib_none(t)).collect();
    let mut targets: Vec<gdk_sys::GdkAtom> = targets.iter().map(|t| t.0).collect();
    unsafe {
        from_glib(gtk_sys::gtk_targets_include_image(targets.as_mut_ptr(), n_targets, writable.to_glib()))
    }
}

pub fn targets_include_text(targets: &[&gdk::Atom]) -> bool {
    assert_initialized_main_thread!();
    let n_targets = targets.len() as i32;
    let targets: Vec<_> = targets.iter().map(|t| ToGlibPtr::<gdk_sys::GdkAtom>::to_glib_none(t)).collect();
    let mut targets: Vec<gdk_sys::GdkAtom> = targets.iter().map(|t| t.0).collect();
    unsafe {
        from_glib(gtk_sys::gtk_targets_include_text(targets.as_mut_ptr(), n_targets))
    }
}

pub fn targets_include_uri(targets: &[&gdk::Atom]) -> bool {
    assert_initialized_main_thread!();
    let n_targets = targets.len() as i32;
    let targets: Vec<_> = targets.iter().map(|t| ToGlibPtr::<gdk_sys::GdkAtom>::to_glib_none(t)).collect();
    let mut targets: Vec<gdk_sys::GdkAtom> = targets.iter().map(|t| t.0).collect();
    unsafe {
        from_glib(gtk_sys::gtk_targets_include_uri(targets.as_mut_ptr(), n_targets))
    }
}
