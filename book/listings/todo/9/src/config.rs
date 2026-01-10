pub const APP_ID: Option<&str> = option_env!("APP_ID");
pub const RESOURCES_FILE: Option<&str> = option_env!("RESOURCES_FILE");

pub fn app_id() -> &'static str {
    APP_ID.expect("APP_ID env var not set at compile time")
}

pub fn resources_file() -> &'static str {
    RESOURCES_FILE.expect("RESOURCES_FILE env var not set at compile time")
}
