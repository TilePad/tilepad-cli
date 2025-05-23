use std::time::Duration;

use serde::Deserialize;

const IDENTIFIER: &str = "TILEPAD_CONTROLLER_SERVER";

#[derive(Deserialize)]
pub struct ServerDetails {
    pub identifier: String,
}

/// Try accessing a running TilePad dev instance to trigger a restart
pub fn try_reload_plugins() -> eyre::Result<()> {
    if get_local_server().is_err() {
        println!("TilePad does not appear to be running, not attempting reload");
        return Ok(());
    }

    println!("TilePad appears to be runnnig, attempting to reload plugins");
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;
    let response = client
        .post("http://127.0.0.1:59371/dev/reload_plugins")
        .send()?;

    response.error_for_status()?;

    Ok(())
}

/// Try accessing a running TilePad dev instance to trigger a restart
pub fn get_local_server() -> eyre::Result<ServerDetails> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;
    let response = client.get("http://127.0.0.1:59371/server/details").send()?;
    let response = response.error_for_status()?;
    let body: ServerDetails = response.json()?;

    if body.identifier != IDENTIFIER {
        eyre::bail!("invalid server identifier");
    }

    Ok(body)
}
