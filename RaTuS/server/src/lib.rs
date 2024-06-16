mod command;
mod command_result;
mod config;
mod error;
mod handlers;
mod routes;
mod types;

pub use crate::config::Config;
use crate::types::CommandPayload;
use tokio::sync::mpsc;

pub async fn start(config: Config) {
    let (command_tx, command_rx) = mpsc::channel::<CommandPayload>(config.queue_size);
    handlers::spawn_tuple_space_handler(command_rx);
    let tuple_routes = routes::tuple_routes(command_tx);
    warp::serve(tuple_routes)
        .run((config.ip_address, config.port))
        .await;
}
