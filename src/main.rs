use std::{
    fs::{File, remove_dir_all},
    io::{Read, Seek, Write},
    os::windows::fs::symlink_dir,
    path::{Path, PathBuf, absolute},
};

use clap::{Parser, Subcommand};
use dirs::data_dir;
use eyre::{Context, OptionExt};
use tilepad_manifest::plugin::Manifest as PluginManifest;
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Scaffold out a new tilepad plugin
    Create,

    /// Restart a specific plugin
    Restart {
        /// ID of the plugin to restart
        plugin_id: String,
    },

    /// Stop a specific plugin
    Stop {
        /// ID of the plugin to stop
        plugin_id: String,
    },

    /// Link the current plugin to tilepad
    ///
    /// Creates a symlink so that changes in the .tilepadPlugin folder
    /// will be accessible in the app
    Link {},

    /// Remove the link from the current plugin
    Unlink,

    /// Bundles the .tilepadPlugin directory into a .tilepadPlugin
    /// archive ready to be installed by Tilepad
    Bundle {
        /// Optional path to the directory containing the .tilepadPlugin directory
        /// if not specified the current directory will be used
        #[arg(short, long)]
        path: Option<PathBuf>,

        /// Optional name for the bundle defaults to the plugin ID from the plugin
        /// manifest file
        #[arg(short, long)]
        name: Option<String>,

        /// Optional output directory to save the .tilepadPlugin archive
        /// defaults to the current directory
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    BundleIconPack {
        /// Optional path to the directory containing the .tilepadIcons directory
        /// if not specified the current directory will be used
        #[arg(short, long)]
        path: Option<PathBuf>,

        /// Optional name for the bundle defaults to the icon pack ID from the
        /// manifest file
        #[arg(short, long)]
        name: Option<String>,

        /// Optional output directory to save the .tilepadIcons archive
        /// defaults to the current directory
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let command = match args.command {
        Some(command) => command,
        None => {
            return Err(eyre::eyre!("please specify a command"));
        }
    };

    match command {
        Commands::Bundle { path, output, name } => bundle(path, output, name),
        Commands::Link {} => link(),
        Commands::Unlink {} => unlink(),
        _ => todo!("not implemented"),
    }
}

fn link() -> eyre::Result<()> {
    let path = PathBuf::from(".");
    let plugin_path = path.join(".tilepadPlugin");
    let plugin_path = absolute(plugin_path).wrap_err("failed to make absolute path")?;

    eyre::ensure!(
        plugin_path.exists(),
        ".tilepadPlugin directory does not exist"
    );

    eyre::ensure!(plugin_path.is_dir(), ".tilepadPlugin is not a directory");

    let manifest_path = plugin_path.join("manifest.toml");
    eyre::ensure!(
        manifest_path.exists(),
        ".tilepadPlugin/manifest.toml manifest file does not exist"
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
    let plugin_out_path = plugins_path.join(&manifest.plugin.id.0);
    let plugin_out_path = absolute(plugin_out_path).wrap_err("failed to make absolute path")?;

    if plugin_out_path.exists() {
        remove_dir_all(&plugin_out_path).wrap_err("failed to create missing plugin directory")?;
    }

    println!(
        "linking {} to {}",
        plugin_out_path.display(),
        plugin_path.display()
    );

    symlink_dir(plugin_path, plugin_out_path).wrap_err("failed to create link")?;

    println!("created link");

    Ok(())
}

fn unlink() -> eyre::Result<()> {
    let path = PathBuf::from(".");
    let plugin_path = path.join(".tilepadPlugin");
    eyre::ensure!(
        plugin_path.exists(),
        ".tilepadPlugin directory does not exist"
    );

    eyre::ensure!(plugin_path.is_dir(), ".tilepadPlugin is not a directory");

    let manifest_path = plugin_path.join("manifest.toml");
    eyre::ensure!(
        manifest_path.exists(),
        ".tilepadPlugin/manifest.toml manifest file does not exist"
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
        remove_dir_all(&plugin_out_path).wrap_err("failed to create missing plugin directory")?;
        println!("removed link");
    } else {
        println!("link not found")
    }

    Ok(())
}

fn bundle(
    path: Option<PathBuf>,
    output: Option<PathBuf>,
    output_name: Option<String>,
) -> eyre::Result<()> {
    let path = path.unwrap_or_else(|| PathBuf::from("."));
    let output_path = output.unwrap_or_else(|| PathBuf::from("."));

    let plugin_path = path.join(".tilepadPlugin");

    eyre::ensure!(
        plugin_path.exists(),
        ".tilepadPlugin directory does not exist"
    );

    eyre::ensure!(plugin_path.is_dir(), ".tilepadPlugin is not a directory");

    let manifest_path = plugin_path.join("manifest.toml");
    eyre::ensure!(
        manifest_path.exists(),
        ".tilepadPlugin/manifest.toml manifest file does not exist"
    );

    let manifest =
        std::fs::read_to_string(manifest_path).wrap_err("failed to read manifest file")?;
    let manifest = PluginManifest::parse(&manifest).wrap_err("failed to parse manifest")?;

    let output_file_name = output_name.unwrap_or_else(|| manifest.plugin.id.0.clone());

    let output_plugin_file = output_path.join(format!("{output_file_name}.tilepadPlugin"));

    let file = File::create(output_plugin_file)?;

    zip(&plugin_path, file)?;

    Ok(())
}

fn zip<T>(input: &Path, writer: T) -> eyre::Result<()>
where
    T: Write + Seek,
{
    let walkdir = WalkDir::new(input);
    let it = walkdir.into_iter();
    let mut zip = zip::ZipWriter::new(writer);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let mut buffer = Vec::new();
    for entry in it {
        let entry = match entry {
            Ok(value) => value,
            Err(_) => continue,
        };

        let path = entry.path();
        let relative_path = path.strip_prefix(input)?;

        if path.is_file() {
            zip.start_file_from_path(relative_path, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !relative_path.as_os_str().is_empty() {
            zip.add_directory_from_path(relative_path, options)?;
        }
    }
    zip.finish()?;
    Ok(())
}
