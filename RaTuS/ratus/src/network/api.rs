use actix_web::{post, web::{self, Data}, Responder};
use serde::{Deserialize, Serialize};
use ts_core::{query_tuple::QueryTuple, store::Store, tuple::Tuple};
use web::Json;

use system::Logger;

use crate::{app::App, store::Request, typ};

#[post("/write")]
pub async fn write(app: Data<App>, req: Json<Request>) -> actix_web::Result<impl Responder> {
    Logger::info(format!("write request: {:?}", req.0), true);
    let response = app.raft.client_write(req.0).await;
    Logger::info(format!("write response: {:?}", response), true);
    Ok(Json(response))
}

#[post("/get")]
pub async fn get(app: Data<App>, req: Json<Request>) -> actix_web::Result<impl Responder> {
    Logger::info(format!("get request: {:?}", req.0), true);
    let response = app.raft.client_write(req.0).await;
    Logger::info(format!("get response: {:?}", response), true);
    Ok(Json(response))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReadRequest {
    pub query: QueryTuple,
}

#[post("/read")]
pub async fn read(app: Data<App>, req: Json<ReadRequest>) -> actix_web::Result<impl Responder> {
    Logger::info(format!("read request: {:?}", req.0), true);
    let state_machine = app.state_machine_store.state_machine.read().await;
    let query = req.0.query;
    let value = state_machine.data.read(&query);

    let res: Option<Tuple> = match value {
        Ok(v) => v,
        Err(_) => None,
    };

    let total_res: Result<Option<Tuple>, typ::RPCError> = Ok(res);
    Logger::info(format!("read response: {:?}", total_res), true);
    Ok(Json(total_res))
}
