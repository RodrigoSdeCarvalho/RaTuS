use ts_core::tuple::Tuple;
use ts_client::client::Client;

use system::{Logger, set_process_name};

#[tokio::main]
async fn main() {
    set_process_name("RaTuS Client Example");

    let client = Client::builder().build("http://localhost:8000").unwrap();
    let tuple = Tuple::builder().string("Number").integer(5).build();

    client.write(&tuple).await.unwrap();
    Logger::info(&format!("Wrote: {}", tuple), true);

    Logger::info(&format!("Size: {}", client.size().await.unwrap()), true);

    let query_tuple = Tuple::query().string("Number").any_integer().build();

    let read_tuple = client.read(&query_tuple).await.unwrap().unwrap();
    Logger::info(&format!("Read: {}", read_tuple), true);

    Logger::info(&format!("Size: {}", client.size().await.unwrap()), true);

    let take_tuple = client.take(&query_tuple).await.unwrap().unwrap();
    Logger::info(&format!("Take: {}", take_tuple), true);

    Logger::info(&format!("Size: {}", client.size().await.unwrap()), true);

    let no_tuple = client.take(&query_tuple).await.unwrap();
    Logger::info(&format!("Take: {:?}", no_tuple), true);
}
