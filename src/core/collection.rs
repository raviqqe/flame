use futures::prelude::*;

use super::error::Error;
use super::normal::Normal;
use super::result::Result;
use super::signature::Signature;
use super::value::Value;

pure_function!(
    INSERT,
    Signature::new(
        vec!["collection".to_string()],
        vec![],
        "keyValuePairs".to_string(),
        vec![],
        vec![],
        "".to_string()
    ),
    insert
);

#[async(boxed_send)]
fn insert(vs: Vec<Value>) -> Result<Value> {
    Ok(match await!(vs[0].clone().pured())? {
        Normal::Dictionary(mut d) => {
            let mut l = await!(vs[1].clone().list())?;

            while !l.is_empty() {
                let k = l.first()?;
                l = await!(l.rest()?.list())?;
                let v = l.first()?;
                l = await!(l.rest()?.list())?;
                d = await!(d.insert(k, v))?;
            }

            Value::from(d)
        }
        Normal::List(_) => unimplemented!(),
        Normal::String(mut s) => {
            let mut l = await!(vs[1].clone().list())?;

            while !l.is_empty() {
                let i = await!(l.first()?.index())? - 1;
                l = await!(l.rest()?.list())?;

                let ss = await!(l.first()?.string())?;
                l = await!(l.rest()?.list())?;

                let sss = s.split_off(i);
                s.extend(ss);
                s.extend(sss);
            }

            Value::from(s)
        }
        n => return Err(await!(Error::not_collection(n))?),
    })
}

pure_function!(
    MERGE,
    Signature::new(
        vec!["collection".to_string()],
        vec![],
        "collections".to_string(),
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
        Normal::List(l) => l.merge(vs[1].clone())?,
        Normal::String(mut s) => {
            let ss = await!(vs[1].clone().string())?;
            s.extend(ss);
            Value::from(s)
        }
        n => return Err(await!(Error::not_collection(n))?),
    })
}
