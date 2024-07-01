use actix_web::{post, web::{Data, Json}, Responder};
use openraft::raft::{AppendEntriesRequest, InstallSnapshotRequest, VoteRequest};

use system::Logger;

use crate::{app::App, TypeConfig};

// --- Raft communication

#[post("/raft-vote")]
pub async fn vote(app: Data<App>, req: Json<VoteRequest<TypeConfig>>) -> actix_web::Result<impl Responder> {
    Logger::info(format!("vote request: {:?}", req.0), true);
    let res = app.raft.vote(req.0).await;
    Logger::info(format!("vote response: {:?}", res), true);
    Ok(Json(res))
}

#[post("/raft-append")]
pub async fn append(app: Data<App>, req: Json<AppendEntriesRequest<TypeConfig>>) -> actix_web::Result<impl Responder> {
    let res = app.raft.append_entries(req.0).await;
    Ok(Json(res))
}

#[post("/raft-snapshot")]
pub async fn snapshot(
    app: Data<App>,
    req: Json<InstallSnapshotRequest<TypeConfig>>,
) -> actix_web::Result<impl Responder> {
    let res = app.raft.install_snapshot(req.0).await;
    Ok(Json(res))
}
