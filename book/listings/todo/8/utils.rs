use std::path::PathBuf;

use gtk::glib;

use crate::APP_ID;

// ANCHOR: data_path
pub fn data_path() -> PathBuf {
    let mut path = glib::user_data_dir();
    path.push(APP_ID);
    std::fs::create_dir_all(&path).expect("Could not create directory.");
    path.push("data.json");
    path
}
// ANCHOR_END: data_path
