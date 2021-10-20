use std::path::PathBuf;

use gtk::glib;

// ANCHOR: data_path
pub fn data_path() -> PathBuf {
    let mut path = glib::user_config_dir();
    path.push("org.gtk-rs.Todo");
    std::fs::create_dir_all(&path).expect("Could not create directory.");
    path.push("data.json");
    path
}
// ANCHOR_END: data_path
