use std::time::Duration;

use ts_core::tuple::Tuple;

use system::{ Logger, set_process_name };

use ratus::{
    client::{ RaftClusterClient, Node }, 
    network::api::ReadRequest, 
    store::Request
};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Run start_node main function in a separate terminal for each of the following nodes:
    // cargo run --bin start_node -- --id 1 --http-addr 127.0.0.1:21001
    // cargo run --bin start_node -- --id 2 --http-addr 127.0.0.1:21002
    // cargo run --bin start_node -- --id 3 --http-addr 127.0.0.1:21003
    // Then run this test_app main function in a separate terminal:
    // cargo run --bin test_app
    let node1 = Node::new(1, "127.0.0.1:21001".to_string());
    let node2 = Node::new(2, "127.0.0.1:21002".to_string());
    let node3 = Node::new(3, "127.0.0.1:21003".to_string());
    
    set_process_name("ratus-test-app");

    // For simplicity, the node that the cluster makes the requests to is changed explicitly.
    // But an actual application could do change the target based on request error or timeout.
    // The election of a new leader is done automatically by the Raft protocol in case of leader failure.

    // Create the client of the Raft cluster.
    let mut cluster = RaftClusterClient::new(&node1);
    Logger::info(format!("Raft cluster client created for node 1 at {}", node1.addr), true);
    // Initialize the cluster.
    cluster.init().await?;
    Logger::info("Raft cluster client initialized", true);
    // Add two nodes to the cluster.
    let _ = cluster.add_nodes(
        vec![
            &node2,
            &node3,
        ]
    ).await?;
    // Check the metrics of the cluster to see the leader and the followers.
    let metrics = cluster.metrics().await?;
    Logger::info(format!("Cluster metrics: {:?}", metrics), true);
    Logger::info(format!("Leader: {:?}", metrics.current_leader), true);

    // Write a tuple to the cluster and check the replication.
    let tuple_to_write = Tuple::builder().
        string("Number").
        integer(5).
        boolean(true).
        build();
    Logger::trace(format!("Writing tuple to cluster: {:?}", tuple_to_write), true);
    let _ = cluster
        .write(&Request::Set {
            tuple: tuple_to_write,
        })
        .await?;

    Logger::info("Tuple written to cluster", true);
    Logger::trace("Sleeping for 1 second to wait for replication", true);
    tokio::time::sleep(Duration::from_millis(1_000)).await;

    // Reading the tuple from each node in the cluster.
    let query = Tuple::query().
        string("Number").
        any_integer().
        any_boolean().
        build();

    cluster.change_target(&node2);
    let x = cluster.read(&ReadRequest {
        query: query.clone(),
    }).await?;
    Logger::info(format!("Read tuple from node 2: {:?}", x), true);

    cluster.change_target(&node3);
    let x = cluster.read(&ReadRequest {
        query: query,
    }).await?;
    Logger::info(format!("Read tuple from node 3: {:?}", x), true);

    // A write (or a get, which changes the state-machine's data too) to non-leader will be automatically forwarded to a known leader 
    // Check the node logs on their terminals to see the forwarding. Or check the logs in the log folder.
    // If the save flag is set to true, in the configs.json file, the logs will be saved in a file.
    let tuple_to_write = Tuple::builder().
        string("Float").
        integer(6).
        float(3.14).
        boolean(true).
        build();
    
    Logger::trace(format!("Writing tuple to cluster: {:?}", tuple_to_write), true);
    let _ = cluster
        .write(&Request::Set {
            tuple: tuple_to_write,
        })
        .await?;
    Logger::info("Tuple written to cluster", true);

    // Reading the tuple from each node in the cluster.
    Logger::trace("Sleeping for 1 second to wait for replication", true);
    tokio::time::sleep(Duration::from_millis(1_000)).await;

    cluster.change_target(&node2);
    let x = cluster.read(&ReadRequest {
        query: Tuple::query().
            string("Float").
            integer(6).
            any_float().
            any_boolean().
            build(),
    }).await?;
    Logger::info(format!("Read tuple from node 2: {:?}", x), true);

    cluster.change_target(&node1);
    let x = cluster.read(&ReadRequest {
        query: Tuple::query().
            string("Float").
            integer(6).
            any_float().
            any_boolean().
            build(),
    }).await?;
    Logger::info(format!("Read tuple from node 1: {:?}", x), true);

    // Removing tuples from the cluster. 
    // Will get and remove the first tuple that matches the query (Not all tuples that match the query)
    // So if two tuples match the query, only one will be removed.
    // Or if there are replicas of the tuple, only one will be removed per query.
    let tuple_to_remove = Tuple::query().
        string("Number").
        integer(5).
        any_boolean().
        build();
    Logger::trace(format!("Removing tuple from cluster: {:?}", tuple_to_remove), true);
    let x = cluster
        .get(&Request::Get {
            query: tuple_to_remove,
        })
        .await;
    Logger::info(format!("Removed tuple from cluster: {:?}", x), true);

    // Check if the tuple was removed from the cluster.
    Logger::trace("Sleeping for 1 second to wait for replication", true);
    tokio::time::sleep(Duration::from_millis(1_000)).await;

    cluster.change_target(&node2);
    let x = cluster.read(&ReadRequest {
        query: Tuple::query().
            string("Number").
            integer(5).
            any_boolean().
            build(),
    }).await?;
    Logger::info(format!("Read tuple from node 2: {:?}", x), true);

    cluster.change_target(&node3);
    let x = cluster.read(&ReadRequest {
        query: Tuple::query().
            string("Number").
            integer(5).
            any_boolean().
            build(),
    }).await?;
    Logger::info(format!("Read tuple from node 3: {:?}", x), true);

    // Fault tolerance test
    // When the following log is shown in the test terminal, kill the leader node.
    cluster.change_target(&node1);

    Logger::info("Kill the leader node to test fault tolerance", true);
    Logger::info("Press Enter to continue", true);
    let _ = std::io::stdin().read_line(&mut String::new());

    let should_err = cluster.metrics().await; 
    if let Err(_) = should_err {
        Logger::info("Node 1 is down as expected", true);
    } else {
        Logger::warn("Error: Expected an error but got none make sure you killed the leader node before pressing Enter", true);
    }

    // Change the target to a working node.
    // Note that the node 1 won't automatically be removed from the cluster, but it won't be the leader anymore.
    // Thus, if it were to recover, it would be a part of the cluster as a follower. And would receve the replicated data.
    cluster.change_target(&node2);

    // Sleep for a while to let the new leader be elected. Could be less than 5 seconds, but 5 seconds is a safe bet.
    Logger::trace("Sleeping for 5 seconds to wait for leader election (Could be less than 5 seconds, but 5 seconds is a safe bet.)", true);
    tokio::time::sleep(Duration::from_millis(5_000)).await;

    // Check the metrics of the cluster to see the new leader and the followers.
    let metrics = cluster.metrics().await?;
    Logger::info(format!("Cluster metrics: {:?}", metrics), true);
    Logger::info(format!("New Elected Leader: {:?}", metrics.current_leader), true);

    // Test all the operations again to assure the fault tolerance.
    Logger::info("Testing all operations again to assure the fault tolerance", true);
    let tuple_to_write = Tuple::builder().
        string("Number").
        integer(7).
        boolean(true).
        float(3.14).
        build();

    Logger::trace(format!("Writing tuple to cluster after new election: {:?}", tuple_to_write), true);    
    let _ = cluster
        .write(&Request::Set {
            tuple: tuple_to_write,
        })    
        .await?;    
    Logger::info("Tuple written to cluster", true);


    Logger::trace("Sleeping for 1 second to wait for replication", true);
    tokio::time::sleep(Duration::from_millis(1_000)).await;

    let query = Tuple::query().
        string("Number").
        any_integer().
        any_boolean().
        any_float().
        build();

    let x = cluster.read(&ReadRequest {
        query: query.clone(),
    }).await?;
    Logger::info(format!("Read tuple from node 2: {:?}", x), true);

    cluster.change_target(&node3);
    let x = cluster.read(&ReadRequest {
        query: query,
    }).await?;
    Logger::info(format!("Read tuple from node 3: {:?}", x), true);

    let tuple_to_remove = Tuple::query().
        string("Number").
        integer(7).
        any_boolean().
        any_float().
        build();
    Logger::trace(format!("Removing tuple from cluster: {:?}", tuple_to_remove), true);

    let x = cluster
        .get(&Request::Get {
            query: tuple_to_remove,
        })
        .await;
    Logger::info(format!("Removed tuple from cluster: {:?}", x), true);

    Logger::trace("Sleeping for 1 second to wait for replication", true);
    tokio::time::sleep(Duration::from_millis(1_000)).await;

    cluster.change_target(&node2);
    let x = cluster.read(&ReadRequest {
        query: Tuple::query().
            string("Number").
            integer(7).
            any_boolean().
            any_float().
            build(),
    }).await?;
    Logger::info(format!("Read tuple from node 2: {:?}", x), true);

    Ok(())
}
