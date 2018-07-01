use std::task::Executor;

use futures::executor::ThreadPool;

use compile::Effect;
use core::Value;

use super::error::RuntimeError;

pub async fn evaluate(v: Value) {
    await!(v.impure()).unwrap();
}

pub async fn run(es: Vec<Effect>) -> Result<(), RuntimeError> {
    let mut p = ThreadPool::new()?;

    for e in es {
        if e.expanded {
            unimplemented!()
        } else {
            p.spawn_obj(Box::new(evaluate(e.value)).into()).unwrap();
        }
    }

    Ok(())
}
