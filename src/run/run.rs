#![feature(proc_macro, conservative_impl_trait, generators)]

use futures::prelude::*;
use futures_cpupool::*;

use super::super::compile::Effect;

lazy_static! {
    static ref POOL: CpuPool = CpuPool::new_num_cpus();
}

#[async]
pub fn run(es: Vec<Effect>) -> Result<(), ()> {
    Ok(())
}
