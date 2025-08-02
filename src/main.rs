use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tilepad_manifest::plugin::PluginId;

mod bundle;
mod bundle_icon_pack;
mod link;
mod server;
mod unlink;
mod zip;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Override the default server port when using a custom port within the tilepad desktop app
    #[arg(short, long)]
    pub port: Option<u16>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Scaffold out a new tilepad plugin
    Create,

    /// Restart a specific plugin
    Restart {
        /// ID of the plugin to restart
        plugin_id: PluginId,
    },

    /// Stop a specific plugin
    Stop {
        /// ID of the plugin to stop
        plugin_id: PluginId,
    },

    /// Link the current plugin to tilepad
    ///
    /// Creates a symlink so that changes in the .tilepadPlugin folder
    /// will be accessible in the app
    Link,

    /// Remove the link from the current plugin
    Unlink,

    /// Tell TilePad to reload the currently loaded plugins and load
    /// any new plugins that were added
    ReloadPlugins,

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
        /// Optional path to the directory containing the iconpack manifest
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

    let port = args.port.unwrap_or(8532);

    match command {
        Commands::Bundle { path, output, name } => bundle::bundle(path, output, name),
        Commands::BundleIconPack { path, output, name } => {
            bundle_icon_pack::bundle_icon_pack(path, output, name)
        }
        Commands::Link => link::link(port),
        Commands::Unlink => unlink::unlink(port),
        Commands::ReloadPlugins => server::try_reload_plugins(port),

        Commands::Create => todo!(),
        Commands::Restart { plugin_id } => server::restart_plugin(port, plugin_id),
        Commands::Stop { plugin_id } => server::stop_plugin(port, plugin_id),
    }
}
