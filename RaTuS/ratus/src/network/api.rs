use actix_web::post;
use actix_web::web;
use actix_web::web::Data;
use actix_web::Responder;
use openraft::error::CheckIsLeaderError;
use openraft::error::RaftError;
use serde::Deserialize;
use serde::Serialize;
use ts_core::query_tuple::QueryTuple;
use ts_core::store::Store;
use ts_core::tuple::Tuple;
use web::Json;

use crate::app::App;
use crate::store::Request;
use crate::typ;
use crate::TypeConfig;

#[post("/write")]
pub async fn write(app: Data<App>, req: Json<Request>) -> actix_web::Result<impl Responder> {
    let response = app.raft.client_write(req.0).await;
    Ok(Json(response))
}

#[post("/get")]
pub async fn get(app: Data<App>, req: Json<Request>) -> actix_web::Result<impl Responder> {
    let response = app.raft.client_write(req.0).await;
    Ok(Json(response))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReadRequest {
    pub query: QueryTuple,
}

#[post("/read")]
pub async fn read(app: Data<App>, req: Json<ReadRequest>) -> actix_web::Result<impl Responder> {
    let state_machine = app.state_machine_store.state_machine.read().await;
    let query = req.0.query;
    let value = state_machine.data.read(&query);

    let res: Option<Tuple> = match value {
        Ok(v) => v,
        Err(_) => None,
    };

    let total_res: Result<Option<Tuple>, typ::RPCError> = Ok(res);
    Ok(Json(total_res))
}

// #[post("/consistent_read")]
// pub async fn consistent_read(app: Data<App>, req: Json<ReadRequest>) -> actix_web::Result<impl Responder> {
//     let ret = app.raft.ensure_linearizable().await;

//     match ret {
//         Ok(_) => {
//             let state_machine = app.state_machine_store.state_machine.read().await;
//             let query = req.0.query;
//             let value = state_machine.data.read(&query);

//             let res: Result<Option<Tuple>, RaftError<TypeConfig, CheckIsLeaderError<TypeConfig>>> =
//                 Ok(value.unwrap_or_default());
//             Ok(Json(res))
//         }
//         Err(e) => Ok(Json(Err(e))),
//     }
// }
