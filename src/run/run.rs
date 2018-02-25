use futures::prelude::*;
use futures_cpupool::CpuPool;

use super::super::compile::Effect;

use super::error::RuntimeError;

lazy_static! {
    static ref POOL: CpuPool = CpuPool::new_num_cpus();
}

#[async]
pub fn run(es: Vec<Effect>) -> Result<(), RuntimeError> {
    for e in es {
        if e.expanded {
            unimplemented!()
        } else {
            POOL.spawn(e.value.impure());
        }
    }

    Ok(())
}
