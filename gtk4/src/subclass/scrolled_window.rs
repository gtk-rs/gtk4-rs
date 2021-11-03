// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

pub trait ScrolledWindowImpl: ScrolledWindowImplExt {}

pub trait ScrolledWindowImplExt: ObjectSubclass {}
