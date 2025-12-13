use std::{env, net::SocketAddr};

use anyhow::Context;
use lexe_sidecar::{
    cli::SidecarArgs,
    client::SidecarClient,
    def::UserSidecarApi,
    lexe::{
        self, lexe_tokio::notify_once::NotifyOnce,
        node_client::credentials::ClientCredentials, sdk_core::def::SdkApi,
    },
    run::Sidecar,
    serde_json,
    tracing::info,
};
use tokio::task::JoinHandle;

fn main() -> anyhow::Result<()> {
    // (Optional) Load env vars from .env.
    let _ = dotenvy::dotenv();

    // (Optional) Set up Lexe's `tracing` logger.
    lexe::logger::init_with_default("info");
    info!("Initializing program.");

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("Failed to build Tokio runtime")?;

    rt.block_on(run())
}

#[tracing::instrument(skip_all, name = "(my-app)")]
async fn run() -> anyhow::Result<()> {
    // Initialize the sidecar server.
    let (sidecar_task, sidecar_url, sidecar_shutdown) =
        init_server().context("Failed to init sidecar")?;

    // A sidecar client is initialized from a URL, e.g. "http://127.0.0.1:5393".
    let client = SidecarClient::new(sidecar_url);

    // Make a request to the health check endpoint.
    let status = client
        .health_check()
        .await
        .context("Failed to get health check")?
        .status;
    info!(%status, "Health check result: ");

    // Make a request to the node_info endpoint.
    let node_info = client
        .node_info()
        .await
        .context("Failed to get node info")?;
    let node_info_json = serde_json::to_string_pretty(&node_info)
        .context("Failed to serialize node info")?;
    info!("Node info:\n{node_info_json}");

    // Tell the sidecar server to shut down.
    sidecar_shutdown.send();

    // Wait for the server task to finish.
    sidecar_task
        .await
        .context("Join error")?
        .context("Sidecar error")?;

    Ok(())
}

/// Initialize the sidecar server.
/// Returns the sidecar task, the server URL, and the server's shutdown channel.
fn init_server(
) -> anyhow::Result<(JoinHandle<anyhow::Result<()>>, String, NotifyOnce)> {
    // Create our args and populate any options from env if available.
    let mut args = SidecarArgs::default();
    args.or_env_mut()?;

    // (Optional) Use port 0 so the OS assigns a port to us. This lets us avoid
    // port conflicts if we need to run multiple sidecars, each with different
    // credentials. The final port is returned to us via `sidecar_url` below.
    let addr = SocketAddr::from(([127, 0, 0, 1], 0)); // 127.0.0.1:0
    args.listen_addr = Some(addr);

    // Already handled by `args.or_env_mut()`, but demonstrates how to construct
    // `ClientCredentials` yourself
    if let Ok(credentials_str) = env::var("LEXE_CLIENT_CREDENTIALS") {
        let credentials =
            ClientCredentials::try_from_base64_blob(&credentials_str)
                .with_context(|| credentials_str)
                .context("Failed to parse credentials")?;
        args.client_credentials = Some(credentials);
    }

    // Init the `Sidecar` struct.
    let sidecar = Sidecar::init(args)?;
    let sidecar_url = sidecar.url();
    let sidecar_shutdown = sidecar.shutdown_channel();

    // Optionally spawn a Ctrl+C handler.
    // It sends a signal over `sidecar_shutdown` when Ctrl+C is pressed.
    let spawn_ctrlc_handler = true;

    // Spawn the sidecar off into a task. You could also just `.await` on it:
    // `sidecar.run(spawn_ctrlc_handler).await`.
    #[allow(clippy::disallowed_methods)] // LxTask is internal to Lexe
    let sidecar_task = tokio::task::spawn(sidecar.run(spawn_ctrlc_handler));

    info!("Sidecar server initialized; running at {sidecar_url}");

    Ok((sidecar_task, sidecar_url, sidecar_shutdown))
}
