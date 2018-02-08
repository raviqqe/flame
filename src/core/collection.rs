use futures::prelude::*;

use super::error::Error;
use super::normal::Normal;
use super::result::Result;
use super::signature::Signature;
use super::value::Value;

pure_function!(
    INSERT,
    Signature::new(
        vec![
            "collection".to_string(),
            "key".to_string(),
            "value".to_string(),
        ],
        vec![],
        "".to_string(),
        vec![],
        vec![],
        "".to_string()
    ),
    insert
);

#[async(boxed_send)]
fn insert(_: Vec<Value>) -> Result<Value> {
    unimplemented!()
}

pure_function!(
    MERGE,
    Signature::new(
        vec!["collection".to_string(), "merged".to_string()],
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
