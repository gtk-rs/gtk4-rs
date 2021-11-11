// Take a look at the license at the top of the repository in the LICENSE file.

use crate::SymbolicPaintable;
use glib::translate::*;
use glib::IsA;

pub trait SymbolicPaintableExtManual: 'static {
    #[doc(alias = "gtk_symbolic_paintable_snapshot_symbolic")]
    fn snapshot_symbolic(
        &self,
        snapshot: &gdk::Snapshot,
        width: f64,
        height: f64,
        colors: &[gdk::RGBA],
    );
}

impl<O: IsA<SymbolicPaintable>> SymbolicPaintableExtManual for O {
    fn snapshot_symbolic(
        &self,
        snapshot: &gdk::Snapshot,
        width: f64,
        height: f64,
        colors: &[gdk::RGBA],
    ) {
        let n_colors = colors.len() as usize;
        unsafe {
            ffi::gtk_symbolic_paintable_snapshot_symbolic(
                self.as_ref().to_glib_none().0,
                snapshot.to_glib_none().0,
                width,
                height,
                colors.to_glib_none().0,
                n_colors,
            );
        }
    }
}
