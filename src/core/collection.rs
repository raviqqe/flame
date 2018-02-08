use futures::prelude::*;

use super::error::Error;
use super::function::Function;
use super::normal::Normal;
use super::result::Result;
use super::signature::Signature;
use super::value::Value;

lazy_static! {
    pub static ref INSERT: Value = Value::from(Function::new(Default::default(), insert));
}

#[async(boxed_send)]
fn insert(vs: Vec<Value>) -> Result<Value> {
    unimplemented!()
}

pure_function!(
    MERGE,
    Signature::new(
        vec![],
        vec![],
        "".to_string(),
        vec![],
        vec![],
        "".to_string()
    ),
    merge
);

#[async(boxed_send)]
fn merge(vs: Vec<Value>) -> Result<Value> {
    Ok(match await!(vs[0].clone().pured())? {
        Normal::Dictionary(d) => {
            let dd = await!(vs[1].clone().dictionary())?;
            Value::from(d.merge(&dd))
        }
        Normal::List(_) => unimplemented!(),
        Normal::String(mut s) => {
            let ss = await!(vs[1].clone().string())?;
            s.extend(ss);
            Value::from(s)
        }
        n => return Err(await!(Error::not_collection(n))?),
    })
}
