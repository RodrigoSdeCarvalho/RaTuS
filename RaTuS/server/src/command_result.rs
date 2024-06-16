use crate::error::Error;
use tuple_space::tuple::Tuple;

#[derive(Debug)]
pub(crate) enum CommandResult {
    Size(usize),
    Write,
    Read(Option<Tuple>),
    Take(Option<Tuple>),
    Error(Error),
}
