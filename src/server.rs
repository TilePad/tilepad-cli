use std::time::Duration;

use serde::Deserialize;
use tilepad_manifest::plugin::PluginId;

const IDENTIFIER: &str = "TILEPAD_CONTROLLER_SERVER";

#[derive(Deserialize)]
pub struct ServerDetails {
    pub identifier: String,
}

/// Try accessing a running TilePad dev instance to trigger a restart
pub fn try_reload_plugins(port: u16) -> eyre::Result<()> {
    if get_local_server(port).is_err() {
        println!("TilePad does not appear to be running, not attempting reload");
        return Ok(());
    }

    println!("TilePad appears to be running, attempting to reload plugins");
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;
    let response = client
        .post(format!("http://127.0.0.1:{port}/dev/reload_plugins"))
        .send()?;

    response.error_for_status()?;

    Ok(())
}

/// Try accessing a running TilePad dev instance to stop a plugin
pub fn stop_plugin(port: u16, plugin_id: PluginId) -> eyre::Result<()> {
    if get_local_server(port).is_err() {
        println!("TilePad does not appear to be running, not attempting stop");
        return Ok(());
    }

    println!("TilePad appears to be running, attempting to stop plugin");
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;
    let response = client
        .post(format!(
            "http://127.0.0.1:{port}/dev/plugin/{plugin_id}/stop"
        ))
        .send()?;

    response.error_for_status()?;

    Ok(())
}

/// Try accessing a running TilePad dev instance to restart a plugin
pub fn restart_plugin(port: u16, plugin_id: PluginId) -> eyre::Result<()> {
    if get_local_server(port).is_err() {
        println!("TilePad does not appear to be running, not attempting restart");
        return Ok(());
    }

    println!("TilePad appears to be running, attempting to restart plugin");
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;
    let response = client
        .post(format!(
            "http://127.0.0.1:{port}/dev/plugin/{plugin_id}/restart"
        ))
        .send()?;

    response.error_for_status()?;

    Ok(())
}

/// Try accessing a running TilePad dev instance to trigger a restart
pub fn get_local_server(port: u16) -> eyre::Result<ServerDetails> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;
    let response = client
        .get(format!("http://127.0.0.1:{port}/server/details"))
        .send()?;
    let response = response.error_for_status()?;
    let body: ServerDetails = response.json()?;

    if body.identifier != IDENTIFIER {
        eyre::bail!("invalid server identifier");
    }

    Ok(body)
}
