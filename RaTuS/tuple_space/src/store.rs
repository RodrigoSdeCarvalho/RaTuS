use crate::query_tuple::QueryTuple;
use crate::result::Result;
use crate::tuple::Tuple;

pub trait Store: Default {
    fn size(&self) -> Result<usize>;
    fn write(&mut self, tuple: &Tuple) -> Result<()>;
    fn read(&self, query_tuple: &QueryTuple) -> Result<Option<Tuple>>;
    fn take(&mut self, query_tuple: &QueryTuple) -> Result<Option<Tuple>>;
}
