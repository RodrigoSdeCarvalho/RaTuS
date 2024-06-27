use serde::{Deserialize, Serialize};

use crate::query_tuple::QueryTuple;
use crate::result::Result;
use crate::store::Store;
use crate::tuple::Tuple;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VecStore {
    inner: Vec<Option<Tuple>>,
    tuple_count: usize,
    compact_margin: f64,
}

pub const DEFAULT_COMPACT_MARGIN: f64 = 0.9;

impl VecStore {
    pub fn tuple_count(&self) -> usize {
        self.tuple_count
    }

    pub fn compact_margin(&self) -> f64 {
        self.compact_margin
    }

    fn compact(&mut self) {
        let current_compact_margin = self.tuple_count as f64 / self.inner.len() as f64;
        if current_compact_margin < self.compact_margin {
            self.inner.retain(|t| t.is_some())
        }
    }

    pub fn builder() -> VecStoreBuilder {
        VecStoreBuilder::default()
    }

    fn index_of(&self, query_tuple: &QueryTuple) -> Option<usize> {
        self.inner.iter().position(|vec_element| {
            if let Some(tuple) = vec_element {
                query_tuple == tuple
            } else {
                false
            }
        })
    }
}

impl Default for VecStore {
    fn default() -> Self {
        Self {
            inner: Vec::new(),
            tuple_count: 0,
            compact_margin: DEFAULT_COMPACT_MARGIN,
        }
    }
}

impl Store for VecStore {
    fn size(&self) -> Result<usize> {
        Ok(self.tuple_count)
    }

    fn read(&self, query_tuple: &QueryTuple) -> Result<Option<Tuple>> {
        match self.index_of(query_tuple) {
            Some(index) => Ok(self.inner[index].clone()),
            None => Ok(None),
        }
    }

    fn write(&mut self, tuple: &Tuple) -> Result<()> {
        self.compact();
        self.inner.push(Some(tuple.clone()));
        self.tuple_count += 1;
        Ok(())
    }

    fn get(&mut self, query_tuple: &QueryTuple) -> Result<Option<Tuple>> {
        match self.index_of(query_tuple) {
            Some(index) => {
                let tuple = self.inner[index].take();
                self.tuple_count -= 1;
                Ok(tuple)
            }
            None => Ok(None),
        }
    }
}

pub struct VecStoreBuilder {
    compact_margin: f64,
}

impl VecStoreBuilder {
    pub fn compact_margin(mut self, compact_margin: f64) -> Self {
        self.compact_margin = compact_margin;
        self
    }

    pub fn build(self) -> VecStore {
        let VecStoreBuilder { compact_margin } = self;
        VecStore {
            compact_margin,
            ..Default::default()
        }
    }
}

impl Default for VecStoreBuilder {
    fn default() -> Self {
        Self {
            compact_margin: DEFAULT_COMPACT_MARGIN,
        }
    }
}

#[test]
fn test_vec_store() -> Result<()> {
    let mut tuple_store = VecStore::builder().compact_margin(0.85).build();

    tuple_store.write(&Tuple::builder().integer(5).build())?;
    tuple_store.write(&Tuple::builder().integer(2).build())?;

    assert_eq!(2, tuple_store.size()?);
    assert_eq!(2, tuple_store.tuple_count());

    match tuple_store.read(&QueryTuple::builder().integer(2).build())? {
        Some(_tuple) => (),
        None => panic!("No tuple found"),
    }

    assert_eq!(2, tuple_store.size()?);
    assert_eq!(2, tuple_store.tuple_count());

    match tuple_store.get(&QueryTuple::builder().integer(5).build())? {
        Some(_tuple) => (),
        None => panic!("No tuple found"),
    }

    assert_eq!(1, tuple_store.size()?);
    assert_eq!(1, tuple_store.tuple_count());

    match tuple_store.get(&QueryTuple::builder().any_integer().build())? {
        Some(_tuple) => (),
        None => panic!("No tuple found"),
    }

    assert_eq!(0, tuple_store.size()?);
    assert_eq!(0, tuple_store.tuple_count());

    match tuple_store.get(&QueryTuple::builder().any_integer().build())? {
        Some(_tuple) => panic!("Tuple found"),
        None => (),
    }
    assert_eq!(0, tuple_store.size()?);
    assert_eq!(0, tuple_store.tuple_count());

    Ok(())
}
