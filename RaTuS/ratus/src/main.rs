use ts_server::{start, Config};

#[tokio::main]
async fn main() {
    let config = Config::new("0.0.0.0", 8000, 1024);
    start(config).await;
}
