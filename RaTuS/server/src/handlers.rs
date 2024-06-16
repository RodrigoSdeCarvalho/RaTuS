use crate::command::Command;
use crate::command_result::CommandResult;
use crate::types::{CommandReceive, CommandSend};
use log::{debug, error, info};
use std::convert::Infallible;
use tokio::sync::oneshot;
use tuple_space::query_tuple::QueryTuple;
use tuple_space::store::Store;
use tuple_space::tuple::Tuple;
use tuple_space::vec_store::VecStore;
use warp::http::StatusCode;

pub(crate) fn spawn_tuple_space_handler(
    mut command_rx: CommandReceive,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut tuple_store = VecStore::default();

        while let Some((command, response)) = command_rx.recv().await {
            debug!("Command {:?} received", command);
            let command_result = match command {
                Command::Size => match tuple_store.size() {
                    Ok(size) => CommandResult::Size(size),
                    Err(error) => CommandResult::Error(error.into()),
                },
                Command::Write(tuple) => match tuple_store.write(&tuple) {
                    Ok(()) => CommandResult::Write,
                    Err(error) => CommandResult::Error(error.into()),
                },
                Command::Read(query_tuple) => match tuple_store.read(&query_tuple) {
                    Ok(tuple_option) => CommandResult::Read(tuple_option),
                    Err(error) => CommandResult::Error(error.into()),
                },
                Command::Take(query_tuple) => match tuple_store.take(&query_tuple) {
                    Ok(tuple_option) => CommandResult::Take(tuple_option),
                    Err(error) => CommandResult::Error(error.into()),
                },
            };
            debug!("CommandResult {:?}", command_result);
            match response.send(command_result) {
                Ok(()) => debug!("CommandResult sent"),
                Err(command_result) => error!("Could not send CommandResult {:?}", command_result),
            }
        }
    })
}

pub(crate) async fn size(
    command_tx: CommandSend,
) -> std::result::Result<Box<dyn warp::Reply>, Infallible> {
    info!("Size");
    let (response_tx, response_rx) = oneshot::channel();

    match command_tx.send((Command::Size, response_tx)).await {
        Ok(_) => (),
        Err(error) => {
            error!("Tuple Space error {:?}", error);
            return Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR));
        }
    }

    match response_rx.await {
        Ok(CommandResult::Size(size)) => {
            info!("Size success");
            Ok(Box::new(warp::reply::json(&size)))
        }
        Err(error) => {
            error!("Error: {:?}", error);
            Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
        }
        unexpected => {
            error!("Unexpected response: {:?}", unexpected);
            Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}

pub(crate) async fn write(
    tuple: Tuple,
    command_tx: CommandSend,
) -> std::result::Result<impl warp::Reply, Infallible> {
    info!("Write {}", tuple);
    let (response_tx, response_rx) = oneshot::channel();
    match command_tx.send((Command::Write(tuple), response_tx)).await {
        Ok(_) => (),
        Err(error) => {
            error!("Tuple Space error {:?}", error);
            return Ok(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    match response_rx.await {
        Ok(CommandResult::Write) => {
            info!("Write success");
            Ok(StatusCode::CREATED)
        }
        Err(error) => {
            error!("Error: {:?}", error);
            Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }
        unexpected => {
            error!("Unexpected response: {:?}", unexpected);
            Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub(crate) async fn read(
    query_tuple: QueryTuple,
    command_tx: CommandSend,
) -> std::result::Result<Box<dyn warp::Reply>, Infallible> {
    let (response_tx, response_rx) = oneshot::channel();
    match command_tx
        .send((Command::Read(query_tuple), response_tx))
        .await
    {
        Ok(_) => (),
        Err(error) => {
            error!("Tuple Space error {:?}", error);
            return Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR));
        }
    }
    match response_rx.await {
        Ok(CommandResult::Read(Some(tuple))) => {
            info!("Tuple found");
            Ok(Box::new(warp::reply::json(&tuple)))
        }
        Ok(CommandResult::Read(None)) => {
            info!("Tuple not found");
            Ok(Box::new(StatusCode::NOT_FOUND))
        }
        Err(error) => {
            error!("Error: {:?}", error);
            Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
        }
        unexpected => {
            error!("Unexpected response: {:?}", unexpected);
            Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}

pub(crate) async fn take(
    query_tuple: QueryTuple,
    command_tx: CommandSend,
) -> std::result::Result<Box<dyn warp::Reply>, Infallible> {
    let (response_tx, response_rx) = oneshot::channel();
    match command_tx
        .send((Command::Take(query_tuple), response_tx))
        .await
    {
        Ok(_) => (),
        Err(error) => {
            error!("Tuple Space error {:?}", error);
            return Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR));
        }
    }
    match response_rx.await {
        Ok(CommandResult::Take(Some(tuple))) => {
            info!("Tuple found");
            Ok(Box::new(warp::reply::json(&tuple)))
        }
        Ok(CommandResult::Take(None)) => {
            info!("Tuple not found");
            Ok(Box::new(StatusCode::NOT_FOUND))
        }
        Err(error) => {
            error!("Error: {:?}", error);
            Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
        }
        unexpected => {
            error!("Unexpected response: {:?}", unexpected);
            Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}
