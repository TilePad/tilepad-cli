use crate::zip::zip_directory;
use eyre::Context;
use std::{fs::File, path::PathBuf};
use tilepad_manifest::icons::IconsManifest;

pub fn bundle_icon_pack(
    path: Option<PathBuf>,
    output: Option<PathBuf>,
    output_name: Option<String>,
) -> eyre::Result<()> {
    let path = path.unwrap_or_else(|| PathBuf::from("."));
    let output_path = output.unwrap_or_else(|| PathBuf::from("."));

    eyre::ensure!(path.exists(), ".tilepadPlugin directory does not exist");
    eyre::ensure!(path.is_dir(), "target is not a directory");

    let manifest_path = path.join("manifest.json");
    eyre::ensure!(
        manifest_path.exists(),
        "manifest.json manifest file does not exist"
    );

    let manifest =
        std::fs::read_to_string(manifest_path).wrap_err("failed to read manifest file")?;
    let manifest = IconsManifest::parse(&manifest).wrap_err("failed to parse manifest")?;

    let output_file_name = output_name.unwrap_or_else(|| manifest.icons.id.0.clone());

    let output_plugin_file = output_path.join(format!("{output_file_name}.tilepadIcons"));

    let file = File::create(output_plugin_file)?;

    zip_directory(&path, file)?;

    Ok(())
}
