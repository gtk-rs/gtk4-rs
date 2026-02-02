pub const APP_ID: Option<&str> = option_env!("APP_ID");
pub const GETTEXT_PACKAGE: Option<&str> = option_env!("GETTEXT_PACKAGE");
pub const LOCALEDIR: Option<&str> = option_env!("LOCALEDIR");
pub const RESOURCES_FILE: Option<&str> = option_env!("RESOURCES_FILE");

pub fn app_id() -> &'static str {
    APP_ID.expect("APP_ID env var not set at compile time")
}

pub fn gettext_package() -> &'static str {
    GETTEXT_PACKAGE.expect("GETTEXT_PACKAGE env var not set at compile time")
}

pub fn localedir() -> &'static str {
    LOCALEDIR.expect("LOCALEDIR env var not set at compile time")
}

pub fn resources_file() -> &'static str {
    RESOURCES_FILE.expect("RESOURCES_FILE env var not set at compile time")
}
