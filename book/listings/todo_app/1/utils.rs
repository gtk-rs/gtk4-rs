use std::path::PathBuf;

use gtk::glib;

pub fn data_path() -> PathBuf {
    let mut path = glib::user_config_dir();
    path.push("MyGtkApp");
    std::fs::create_dir_all(&path).expect("Could not create directory.");
    path.push("data.json");
    path
}
