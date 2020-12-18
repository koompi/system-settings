pub fn ROOT_PATH() -> String {
    if cfg!(debug_assertions) {
        format!(
            "{}/root/usr/share/system_settings",
            env!("CARGO_MANIFEST_DIR")
        )
    } else {
        format!("/usr/share/system_settings")
    }
}
