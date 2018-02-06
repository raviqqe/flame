use futures::Future;

use super::arguments::Arguments;
use super::error::Error;
use super::value::Value;

pub type Function = Box<Callable>;

pub trait Callable: Send + Sync {
    fn call(self, a: Arguments) -> Box<Future<Item = Value, Error = Error>>;
}
