use tuple_space::tuple::Tuple;
use ts_client::client::Client;

use system::{Logger, set_process_name};

#[tokio::main]
async fn main() {
    set_process_name("RaTuS Client Example");

    let client = Client::builder().build("http://localhost:8000").unwrap();

    let tuple = Tuple::builder().string("Number").integer(5).build();

    client.write(&tuple).await.unwrap();
    println!("Wrote: {}", tuple);

    println!("Size: {}", client.size().await.unwrap());

    let query_tuple = Tuple::query().string("Number").any_integer().build();

    let read_tuple = client.read(&query_tuple).await.unwrap().unwrap();
    println!("Read {}", read_tuple);

    println!("Size: {}", client.size().await.unwrap());

    let take_tuple = client.take(&query_tuple).await.unwrap().unwrap();
    println!("Take {}", take_tuple);

    println!("Size: {}", client.size().await.unwrap());

    let no_tuple = client.take(&query_tuple).await.unwrap();
    println!("Take {:?}", no_tuple);
}
