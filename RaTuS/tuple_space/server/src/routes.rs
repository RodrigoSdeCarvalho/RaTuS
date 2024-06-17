use crate::handlers;
use crate::types::CommandSend;
use std::convert::Infallible;
use warp::Filter;

const SIZE_PATH: &str = "size";
const WRITE_PATH: &str = "write";
const READ_PATH: &str = "read";
const TAKE_PATH: &str = "take";

fn with_command_tx(
    command_tx: CommandSend,
) -> impl Filter<Extract = (CommandSend,), Error = Infallible> + Clone {
    warp::any().map(move || command_tx.clone())
}

fn size(
    command_tx: CommandSend,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(SIZE_PATH)
        .and(warp::get())
        .and(with_command_tx(command_tx))
        .and_then(handlers::size)
}

fn write(
    command_tx: CommandSend,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(WRITE_PATH)
        .and(warp::post())
        .and(warp::body::json())
        .and(with_command_tx(command_tx))
        .and_then(handlers::write)
}

fn read(
    command_tx: CommandSend,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(READ_PATH)
        .and(warp::post())
        .and(warp::body::json())
        .and(with_command_tx(command_tx))
        .and_then(handlers::read)
}

fn take(
    command_tx: CommandSend,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(TAKE_PATH)
        .and(warp::post())
        .and(warp::body::json())
        .and(with_command_tx(command_tx))
        .and_then(handlers::take)
}

pub(crate) fn tuple_routes(
    command_tx: CommandSend,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    size(command_tx.clone())
        .or(write(command_tx.clone()))
        .or(read(command_tx.clone()))
        .or(take(command_tx))
}
