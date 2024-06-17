use crate::tuple::Tuple;
use crate::types::QueryTypes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryTuple {
    query_tuple: Vec<QueryTypes>,
}

impl QueryTuple {
    pub fn len(&self) -> usize {
        self.query_tuple.len()
    }

    pub fn is_empty(&self) -> bool {
        self.query_tuple.is_empty()
    }

    pub fn builder() -> QueryTupleBuilder {
        QueryTupleBuilder::default()
    }
}

impl PartialEq<Tuple> for QueryTuple {
    fn eq(&self, rhs: &Tuple) -> bool {
        if self.len() != rhs.len() {
            return false;
        }
        for i in 0..self.len() {
            if self[i] != rhs[i] {
                return false;
            }
        }
        true
    }
}

impl std::ops::Index<usize> for QueryTuple {
    type Output = QueryTypes;

    fn index(&self, index: usize) -> &Self::Output {
        &self.query_tuple[index]
    }
}

#[derive(Default)]
pub struct QueryTupleBuilder {
    query_tuple: Vec<QueryTypes>,
}

impl QueryTupleBuilder {
    pub fn build(self) -> QueryTuple {
        let QueryTupleBuilder { query_tuple } = self;
        QueryTuple { query_tuple }
    }

    pub fn any(mut self) -> Self {
        self.query_tuple.push(QueryTypes::Any);
        self
    }

    pub fn any_integer(mut self) -> Self {
        self.query_tuple.push(QueryTypes::AnyInteger);
        self
    }

    pub fn integer(mut self, integer: i64) -> Self {
        self.query_tuple.push(QueryTypes::ExactInteger(integer));
        self
    }

    pub fn any_float(mut self) -> Self {
        self.query_tuple.push(QueryTypes::AnyFloat);
        self
    }

    pub fn float(mut self, float: f64) -> Self {
        self.query_tuple.push(QueryTypes::ExactFloat(float));
        self
    }

    pub fn any_boolean(mut self) -> Self {
        self.query_tuple.push(QueryTypes::AnyBoolean);
        self
    }

    pub fn boolean(mut self, boolean: bool) -> Self {
        self.query_tuple.push(QueryTypes::ExactBoolean(boolean));
        self
    }

    pub fn any_string(mut self) -> Self {
        self.query_tuple.push(QueryTypes::AnyString);
        self
    }

    pub fn string(mut self, string: &str) -> Self {
        self.query_tuple
            .push(QueryTypes::ExactString(String::from(string)));
        self
    }
}
