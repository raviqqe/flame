use futures::prelude::*;
use futures::executor::ThreadPool;

use compile::Effect;
use core::Value;

use super::error::RuntimeError;

#[async_move(boxed_send)]
fn evaluate(v: Value) -> Result<(), Never> {
    await!(v.impure());
    Ok(())
}

#[async_move]
pub fn run(es: Vec<Effect>) -> Result<(), RuntimeError> {
    let mut p = ThreadPool::new();

    for e in es {
        if e.expanded {
            unimplemented!()
        } else {
            p.spawn(evaluate(e.value));
        }
    }

    Ok(())
}
