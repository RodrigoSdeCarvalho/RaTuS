use crate::error::Error;
use ts_core::tuple::Tuple;

#[derive(Debug)]
pub(crate) enum CommandResult {
    Size(usize),
    Write,
    Read(Option<Tuple>),
    Get(Option<Tuple>),
    Error(Error),
}
