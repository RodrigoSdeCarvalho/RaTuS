use crate::query_tuple::QueryTupleBuilder;
use crate::types::Types;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tuple {
    tuple: Vec<Types>,
}

impl Tuple {
    pub fn builder() -> TupleBuilder {
        TupleBuilder::default()
    }

    pub fn query() -> QueryTupleBuilder {
        QueryTupleBuilder::default()
    }

    pub fn len(&self) -> usize {
        self.tuple.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tuple.is_empty()
    }
}

impl std::fmt::Display for Tuple {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(formatter, "(")?;
        write!(
            formatter,
            "{}",
            self.tuple
                .iter()
                .map(|t| format!("{}", t))
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        write!(formatter, ")")?;
        Ok(())
    }
}

#[derive(Default)]
pub struct TupleBuilder {
    tuple: Vec<Types>,
}

impl TupleBuilder {
    pub fn build(self) -> Tuple {
        let TupleBuilder { tuple } = self;

        Tuple { tuple }
    }

    pub fn integer(mut self, integer: i64) -> Self {
        self.tuple.push(Types::Integer(integer));
        self
    }

    pub fn float(mut self, float: f64) -> Self {
        self.tuple.push(Types::Float(float));
        self
    }

    pub fn boolean(mut self, boolean: bool) -> Self {
        self.tuple.push(Types::Boolean(boolean));
        self
    }

    pub fn string(mut self, string: &str) -> Self {
        self.tuple.push(Types::String(String::from(string)));
        self
    }
}

impl PartialEq for Tuple {
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

impl std::ops::Index<usize> for Tuple {
    type Output = Types;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tuple[index]
    }
}

#[test]
fn test_builder() {
    let tuple = Tuple::builder().integer(5).build();

    assert_eq!(1, tuple.len());

    let tuple = Tuple::builder()
        .integer(1)
        .float(2.0)
        .boolean(true)
        .string("String")
        .build();
    assert_eq!(4, tuple.len());

    let query_tuple = Tuple::query()
        .integer(1)
        .float(2.0)
        .boolean(true)
        .string("String")
        .build();
    assert_eq!(query_tuple, tuple);

    let query_tuple = Tuple::query()
        .any_integer()
        .any_float()
        .any_boolean()
        .any_string()
        .build();
    assert_eq!(query_tuple, tuple)
}
