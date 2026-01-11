use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Root directory for assets.
///
/// For now:
/// - Use the process's current working directory + "assets".
/// - Later make this configurable or use env vars.
/// e. g., a config file specifying where assets live.
pub fn assets_roots() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("assets")
}

/// Resolve a path relative to the assets root.
pub fn asset_path(relative: impl AsRef<Path>) -> PathBuf {
    assets_roots().join(relative)
}

/// Read a whole file as bytes from the assets directory.
pub fn read_asset_binary(relative: impl AsRef<Path>) -> io::Result<Vec<u8>> {
    let full_path = asset_path(relative);
    fs::read(full_path)
}

/// Read a whole file as a UTF-8 string from the assets directory.
pub fn read_asset_string(relative: impl AsRef<Path>) -> io::Result<String> {
    let full_path = asset_path(relative);
    fs::read_to_string(full_path)
}
