use tuple_space::query_tuple::QueryTuple;
use tuple_space::tuple::Tuple;

#[derive(Debug)]
pub(crate) enum Command {
    Size,
    Write(Tuple),
    Read(QueryTuple),
    Take(QueryTuple),
}
