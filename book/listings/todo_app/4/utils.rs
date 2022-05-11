use std::path::PathBuf;

use crate::APP_ID;
use gtk::glib;

// ANCHOR: data_path
pub fn data_path() -> PathBuf {
    let mut path = glib::user_data_dir();
    path.push(APP_ID);
    std::fs::create_dir_all(&path).expect("Could not create directory.");
    path.push("data.json");
    path
}
// ANCHOR_END: data_path
