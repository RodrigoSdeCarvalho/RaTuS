use crate::error::Error;
use ts_core::tuple::Tuple;

#[derive(Debug)]
pub(crate) enum CommandResult {
    Size(usize),
    Write,
    Read(Option<Tuple>),
    Take(Option<Tuple>),
    Error(Error),
}
