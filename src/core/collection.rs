use futures::prelude::*;

use super::error::Error;
use super::normal::Normal;
use super::result::Result;
use super::signature::Signature;
use super::string::Str;
use super::value::Value;

pure_function!(
    INSERT,
    Signature::new(
        vec!["collection".into()],
        "keyValuePairs".into(),
        vec![],
        "".into()
    ),
    insert
);

#[async_move(boxed_send)]
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
                s = Str::merge(&[f, m, l]);
            }

            Value::from(s)
        }
        n => return Err(await!(Error::not_collection(n.into()))?),
    })
}

pure_function!(
    MERGE,
    Signature::new(
        vec!["collection".into()],
        "collections".into(),
        vec![],
        "".into()
    ),
    merge
);

#[async_move(boxed_send)]
fn merge(vs: Vec<Value>) -> Result<Value> {
    Ok(match await!(vs[0].clone().pured())? {
        Normal::Dictionary(d) => {
            let dd = await!(vs[1].clone().dictionary())?;
            Value::from(d.merge(&dd))
        }
        Normal::List(l) => await!(l.merge(vs[1].clone()))?,
        Normal::String(mut s) => {
            let mut l = await!(vs[1].clone().list())?;
            let mut ss = vec![s];

            while !l.is_empty() {
                let s = await!(l.first()?.string())?;
                l = await!(l.rest())?;
                ss.push(s);
            }

            Str::merge(&ss).into()
        }
        n => return Err(await!(Error::not_collection(n.into()))?),
    })
}

#[cfg(test)]
mod test {
    use futures::executor::block_on;

    use super::*;

    use super::super::list::List;
    use super::super::utils::papp;

    #[test]
    fn merge() {
        for (vs, x) in vec![
            (&[List::default().into()], List::default().into()),
            (
                &[List::default().into(), List::new(&[0.into()]).into()],
                List::new(&[0.into()]).into(),
            ),
            (
                &[List::new(&[0.into()]).into(), List::default().into()],
                List::new(&[0.into()]).into(),
            ),
            (
                &[List::new(&[0.into()]).into(), List::new(&[1.into()]).into()],
                List::new(&[0.into(), 1.into()]).into(),
            ),
            (
                &[
                    List::new(&[0.into()]).into(),
                    List::new(&[1.into()]).into(),
                    List::new(&[2.into()]).into(),
                ],
                List::new(&[0.into(), 1.into(), 2.into()]).into(),
            ),
            (&["".into()], "".into()),
            (
                &["foo".into(), "bar".into(), "baz".into()],
                "foobarbaz".into(),
            ),
        ]: Vec<(&[Value], Value)>
        {
            assert!(block_on(papp(MERGE.clone(), vs).equal(x)).unwrap());
        }
    }
}
