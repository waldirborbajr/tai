pub fn show_version() -> String {
    let pkg_name: &str = env!("CARGO_PKG_NAME");
    let pkg_version: &str = env!("CARGO_PKG_VERSION");
    format!("{} via 🦀 v{}/2023", pkg_name, pkg_version)
}
