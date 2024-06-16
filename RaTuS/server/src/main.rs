mod cli;
mod command;
mod command_result;
mod config;
mod error;
mod handlers;
mod result;
mod routes;
mod types;

use crate::config::Config;
use crate::types::CommandPayload;
use log::{debug, info};
use tokio::sync::mpsc;

#[tokio::main]
async fn run(config: Config) {
    let (command_tx, command_rx) = mpsc::channel::<CommandPayload>(config.queue_size);
    debug!(
        "Command channels initialised with queue size {}",
        config.queue_size
    );
    handlers::spawn_tuple_space_handler(command_rx);
    debug!("Tuple space handler spawned");
    let tuple_routes = routes::tuple_routes(command_tx);
    debug!("Warp server starting");
    warp::serve(tuple_routes)
        .run((config.ip_address, config.port))
        .await;
}

fn main() -> crate::result::Result<()> {
    env_logger::init();
    info!("Starting..");

    let cli_args = crate::cli::parse_args();
    let config_file_arg = cli_args.value_of(crate::cli::CONFIG_FILE);

    let config = Config::load_configuration(config_file_arg)?;
    run(config);
    Ok(())
}
