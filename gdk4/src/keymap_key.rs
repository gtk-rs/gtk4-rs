// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

glib::wrapper! {
    #[doc(alias = "GdkKeymapKey")]
    pub struct KeymapKey(BoxedInline<ffi::GdkKeymapKey>);
}

impl KeymapKey {
    pub fn new(keycode: u32, group: i32, level: i32) -> Self {
        assert_initialized_main_thread!();
        KeymapKey(ffi::GdkKeymapKey {
            group,
            keycode,
            level,
        })
    }

    pub fn keycode(&self) -> u32 {
        self.0.keycode
    }

    pub fn group(&self) -> i32 {
        self.0.group
    }

    pub fn level(&self) -> i32 {
        self.0.level
    }
}

impl fmt::Debug for KeymapKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("KeymapKey")
            .field("keycode", &self.keycode())
            .field("group", &self.group())
            .field("level", &self.level())
            .finish()
    }
}
