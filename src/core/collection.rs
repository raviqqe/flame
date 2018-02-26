use futures::prelude::*;

use super::error::Error;
use super::normal::Normal;
use super::result::Result;
use super::signature::Signature;
use super::value::Value;

pure_function!(
    INSERT,
    Signature::new(
        vec!["collection".into()],
        vec![],
        "keyValuePairs".into(),
        vec![],
        vec![],
        "".into()
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
                l = await!(l.rest())?;

                let v = l.first()?;
                l = await!(l.rest())?;

                d = await!(d.insert(k, v))?;
            }

            Value::from(d)
        }
        Normal::List(mut l) => {
            let mut ivs = await!(vs[1].clone().list())?;

            while !ivs.is_empty() {
                let i = await!(ivs.first()?.index())? - 1;
                ivs = await!(ivs.rest())?;

                let v = ivs.first()?;
                ivs = await!(ivs.rest())?;

                l = l.insert(i, v)?;
            }

            Value::from(l)
        }
        Normal::String(mut s) => {
            let mut l = await!(vs[1].clone().list())?;

            while !l.is_empty() {
                let i = await!(l.first()?.index())? - 1;
                l = await!(l.rest())?;

                let m = await!(l.first()?.string())?;
                l = await!(l.rest())?;

                let (f, l) = s.split(i);
                s = f.extend(&m).extend(&l);
            }

            Value::from(s)
        }
        n => return Err(await!(Error::not_collection(n))?),
    })
}

pure_function!(
    MERGE,
    Signature::new(
        vec!["collection".into()],
        vec![],
        "collections".into(),
        vec![],
        vec![],
        "".into()
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
            Value::from(s.extend(&ss))
        }
        n => return Err(await!(Error::not_collection(n))?),
    })
}
