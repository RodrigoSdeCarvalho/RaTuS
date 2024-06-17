use ts_core::query_tuple::QueryTuple;
use ts_core::tuple::Tuple;

#[derive(Debug)]
pub(crate) enum Command {
    Size,
    Write(Tuple),
    Read(QueryTuple),
    Get(QueryTuple),
}
