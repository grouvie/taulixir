use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use serde::Deserialize;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::ErlRpc;
#[cfg(mobile)]
use mobile::ErlRpc;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the erl-rpc APIs.
pub trait ErlRpcExt<R: Runtime> {
    fn erl_rpc(&self) -> &ErlRpc<R>;
}

impl<R: Runtime, T: Manager<R>> crate::ErlRpcExt<R> for T {
    fn erl_rpc(&self) -> &ErlRpc<R> {
        self.state::<ErlRpc<R>>().inner()
    }
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub server_node_name: String,
    pub cookie: String,
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R, Config> {
    let builder = Builder::<R, Config>::new("erl-rpc").invoke_handler(tauri::generate_handler![
        commands::ping,
        commands::invoke_erl_rpc
    ]);
    builder
        .setup(|app, api: tauri::plugin::PluginApi<R, Config>| {
            let config = api.config().clone();
            #[cfg(desktop)]
            let erl_rcp_state = desktop::init(app, api)?;
            #[cfg(mobile)]
            let erl_rcp_state = mobile::init(app, api)?;
            app.manage(erl_rcp_state);
            let app_handle_into = app.clone();
            tokio::spawn(async move {
                if let Err(e) = async_setup(app_handle_into, &config).await {
                    eprintln!("Error during async setup: {}", e);
                }
            });
            Ok(())
        })
        .build()
}

async fn async_setup<R: Runtime>(app_handle: tauri::AppHandle<R>, config: &Config) -> Result<()> {
    // Define the retry parameters.
    let retry_interval = Duration::from_secs(5);
    let overall_timeout = Duration::from_secs(30);
    let start_time = Instant::now();

    sleep(retry_interval);

    let rpc_client = loop {
        match erl_rpc::RpcClient::connect(&config.server_node_name, &config.cookie).await {
            Ok(client) => break client,
            Err(err) => {
                if Instant::now().duration_since(start_time) >= overall_timeout {
                    panic!("Connecting erl RpcClient failed after 30 seconds: {err:?}");
                }
                eprintln!("Failed to connect: {err:?}. Retrying in 5 seconds...");
                sleep(retry_interval);
            }
        }
    };
    println!("RpcClient connected");

    let rpc_handle = rpc_client.handle();
    app_handle.manage(rpc_handle);

    // Run the RPC client as a background task
    tokio::spawn(async move {
        if let Err(error) = rpc_client.run().await {
            eprintln!("RpcClient Error: {error}");
        }
    });

    Ok(())
}
