use dirs::data_dir;
use eyre::{Context, OptionExt};
use std::path::PathBuf;
use symlink::remove_symlink_dir;
use tilepad_manifest::plugin::PluginManifest;

use crate::server::try_reload_plugins;

pub fn unlink(port: u16) -> eyre::Result<()> {
    let path = PathBuf::from(".");
    let plugin_path = path.join(".tilepadPlugin");
    eyre::ensure!(
        plugin_path.exists(),
        ".tilepadPlugin directory does not exist"
    );

    eyre::ensure!(plugin_path.is_dir(), ".tilepadPlugin is not a directory");

    let manifest_path = plugin_path.join("manifest.json");
    eyre::ensure!(
        manifest_path.exists(),
        ".tilepadPlugin/manifest.json manifest file does not exist"
    );

    let manifest =
        std::fs::read_to_string(manifest_path).wrap_err("failed to read manifest file")?;
    let manifest = PluginManifest::parse(&manifest).wrap_err("failed to parse manifest")?;

    let data_path = data_dir().ok_or_eyre("failed to get app data directory")?;

    let tilepad_path = data_path.join("com.jacobtread.tilepad.desktop");
    eyre::ensure!(
        tilepad_path.exists(),
        "tilepad directory does not exist, do you have it installed?"
    );

    let plugins_path = tilepad_path.join("plugins");
    let plugin_out_path: PathBuf = plugins_path.join(&manifest.plugin.id.0);

    if plugin_out_path.is_symlink() {
        remove_symlink_dir(&plugin_out_path)
            .wrap_err("failed to create missing plugin directory")?;
        println!("removed link");
    } else {
        println!("link not found")
    }

    try_reload_plugins(port)?;

    Ok(())
}
