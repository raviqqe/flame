use futures::prelude::*;

use super::error::Error;
use super::value::Value;

pub const MERGE: Value = unimplemented!();

pub trait Collection
where
    Self: Sized,
{
    fn merge<F: Future<Item = Self, Error = Error>>(self, c: Self) -> F;
}
