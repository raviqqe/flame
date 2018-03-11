use futures::executor::ThreadPool;
use futures::prelude::*;

use compile::Effect;
use core::Value;

use super::error::RuntimeError;

#[async_move(boxed_send)]
pub fn evaluate(v: Value) -> Result<(), Never> {
    await!(v.impure()).unwrap();
    Ok(())
}

#[async_move]
pub fn run(es: Vec<Effect>) -> Result<(), RuntimeError> {
    let mut p = ThreadPool::new();

    for e in es {
        if e.expanded {
            unimplemented!()
        } else {
            p.spawn(evaluate(e.value)).unwrap();
        }
    }

    Ok(())
}
