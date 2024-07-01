use std::collections::{BTreeMap, BTreeSet};

use actix_web::{get, post, web::{Data, Json}, Responder};
use openraft::{error::Infallible, BasicNode, RaftMetrics};

use system::Logger;

use crate::{app::App, NodeId, TypeConfig};

// --- Cluster management

/// Add a node as **Learner**.
///
/// A Learner receives log replication from the leader but does not vote.
/// This should be done before adding a node as a member into the cluster
/// (by calling `change-membership`)
#[post("/add-learner")]
pub async fn add_learner(app: Data<App>, req: Json<(NodeId, String)>) -> actix_web::Result<impl Responder> {
    Logger::info("add-learner request", true);
    let node_id = req.0 .0;
    let node = BasicNode { addr: req.0 .1.clone() };
    let res = app.raft.add_learner(node_id, node, true).await;
    Logger::info(format!("add-learner response: {:?}", res), true);
    Ok(Json(res))
}

/// Changes specified learners to members, or remove members.
#[post("/change-membership")]
pub async fn change_membership(app: Data<App>, req: Json<BTreeSet<NodeId>>) -> actix_web::Result<impl Responder> {
    Logger::info("change-membership request", true);
    let res = app.raft.change_membership(req.0, false).await;
    Logger::info(format!("change-membership response: {:?}", res), true);
    Ok(Json(res))
}

/// Initialize a single-node cluster if the `req` is empty vec.
/// Otherwise initialize a cluster with the `req` specified vec of node-id and node-address
#[post("/init")]
pub async fn init(app: Data<App>, req: Json<Vec<(NodeId, String)>>) -> actix_web::Result<impl Responder> {
    Logger::info("init request", true);
    let mut nodes = BTreeMap::new();
    if req.0.is_empty() {
        nodes.insert(app.id, BasicNode { addr: app.addr.clone() });
    } else {
        for (id, addr) in req.0.into_iter() {
            nodes.insert(id, BasicNode { addr });
        }
    };
    let res = app.raft.initialize(nodes).await;
    Logger::info(format!("init response: {:?}", res), true);
    Ok(Json(res))
}

/// Get the latest metrics of the cluster
#[get("/metrics")]
pub async fn metrics(app: Data<App>) -> actix_web::Result<impl Responder> {
    Logger::info("metrics request", true);
    let metrics = app.raft.metrics().borrow().clone();
    let res: Result<RaftMetrics<TypeConfig>, Infallible> = Ok(metrics);
    Logger::info(format!("metrics response: {:?}", res), true);
    Ok(Json(res))
}
