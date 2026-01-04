use std::path::PathBuf;

use gtk::glib;

use crate::config;

pub fn data_path() -> PathBuf {
    let mut path = glib::user_data_dir();
    path.push(config::APP_ID);
    std::fs::create_dir_all(&path).expect("Could not create directory.");
    path.push("data.json");
    path
}
