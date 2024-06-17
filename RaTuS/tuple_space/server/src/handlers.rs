use std::convert::Infallible;
use tokio::sync::oneshot;
use warp::http::StatusCode;

use crate::command::Command;
use crate::command_result::CommandResult;
use crate::types::{CommandReceive, CommandSend};

use ts_core::{
    query_tuple::QueryTuple,
    store::Store,
    tuple::Tuple,
    vec_store::VecStore,
    mutex_store::MutexStore,
};

use system::Logger;

pub(crate) fn spawn_tuple_space_handler(
    mut command_rx: CommandReceive,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut mutex_store = MutexStore::<VecStore>::default();

        while let Some((command, response)) = command_rx.recv().await {
            Logger::info(&format!("Command {:?} received", command), true);
            let command_result = match command {
                Command::Size => match mutex_store.size() {
                    Ok(size) => CommandResult::Size(size),
                    Err(error) => CommandResult::Error(error.into()),
                },
                Command::Write(tuple) => match mutex_store.write(&tuple) {
                    Ok(()) => CommandResult::Write,
                    Err(error) => CommandResult::Error(error.into()),
                },
                Command::Read(query_tuple) => match mutex_store.read(&query_tuple) {
                    Ok(tuple_option) => CommandResult::Read(tuple_option),
                    Err(error) => CommandResult::Error(error.into()),
                },
                Command::Get(query_tuple) => match mutex_store.get(&query_tuple) {
                    Ok(tuple_option) => CommandResult::Get(tuple_option),
                    Err(error) => CommandResult::Error(error.into()),
                },
            };
            Logger::info(&format!("CommandResult {:?}", command_result), true);
            match response.send(command_result) {
                Ok(()) => Logger::info("CommandResult sent", true),
                Err(command_result) => Logger::error(&format!("Could not send CommandResult {:?}", command_result), true),
            }
        }
    })
}

pub(crate) async fn size(
    command_tx: CommandSend,
) -> std::result::Result<Box<dyn warp::Reply>, Infallible> {
    Logger::info("Size", true);
    let (response_tx, response_rx) = oneshot::channel();

    match command_tx.send((Command::Size, response_tx)).await {
        Ok(_) => (),
        Err(error) => {
            Logger::error(&format!("Tuple Space error {:?}", error), true);
            return Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR));
        }
    }

    match response_rx.await {
        Ok(CommandResult::Size(size)) => {
            Logger::info(&format!("Size: {}", size), true);
            Ok(Box::new(warp::reply::json(&size)))
        }
        Err(error) => {
            Logger::error(&format!("Error: {:?}", error), true);
            Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
        }
        unexpected => {
            Logger::error(&format!("Unexpected response: {:?}", unexpected), true);
            Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}

pub(crate) async fn write(
    tuple: Tuple,
    command_tx: CommandSend,
) -> std::result::Result<impl warp::Reply, Infallible> {
    Logger::info(&format!("Write {:?}", tuple), true);
    let (response_tx, response_rx) = oneshot::channel();
    match command_tx.send((Command::Write(tuple), response_tx)).await {
        Ok(_) => (),
        Err(error) => {
            Logger::error(&format!("Tuple Space error {:?}", error), true);
            return Ok(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    match response_rx.await {
        Ok(CommandResult::Write) => {
            Logger::info("Write success", true);
            Ok(StatusCode::CREATED)
        }
        Err(error) => {
            Logger::error(&format!("Error: {:?}", error), true);
            Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }
        unexpected => {
            Logger::error(&format!("Unexpected response: {:?}", unexpected), true);
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
            Logger::error(&format!("Tuple Space error {:?}", error), true);
            return Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR));
        }
    }
    match response_rx.await {
        Ok(CommandResult::Read(Some(tuple))) => {
            Logger::info(&format!("Tuple found {:?}", tuple), true);
            Ok(Box::new(warp::reply::json(&tuple)))
        }
        Ok(CommandResult::Read(None)) => {
            Logger::info("Tuple not found", true);
            Ok(Box::new(StatusCode::NOT_FOUND))
        }
        Err(error) => {
            Logger::error(&format!("Error: {:?}", error), true);
            Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
        }
        unexpected => {
            Logger::error(&format!("Unexpected response: {:?}", unexpected), true);
            Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}

pub(crate) async fn get(
    query_tuple: QueryTuple,
    command_tx: CommandSend,
) -> std::result::Result<Box<dyn warp::Reply>, Infallible> {
    let (response_tx, response_rx) = oneshot::channel();
    match command_tx
        .send((Command::Get(query_tuple), response_tx))
        .await
    {
        Ok(_) => (),
        Err(error) => {
            Logger::error(&format!("Tuple Space error {:?}", error), true);
            return Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR));
        }
    }
    match response_rx.await {
        Ok(CommandResult::Get(Some(tuple))) => {
            Logger::info(&format!("Tuple found {:?}", tuple), true);
            Ok(Box::new(warp::reply::json(&tuple)))
        }
        Ok(CommandResult::Get(None)) => {
            Logger::info("Tuple not found", true);
            Ok(Box::new(StatusCode::NOT_FOUND))
        }
        Err(error) => {
            Logger::error(&format!("Error: {:?}", error), true);
            Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
        }
        unexpected => {
            Logger::error(&format!("Unexpected response: {:?}", unexpected), true);
            Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}
