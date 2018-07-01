use super::result::Result;
use super::signature::Signature;
use super::value::Value;

pure_function!(
    IF,
    Signature::new(vec![], "conds".into(), vec![], "".into()),
    iff
);

async fn iff(vs: Vec<Value>) -> Result<Value> {
    let mut l = await!(vs[0].clone().list())?;

    loop {
        let r = await!(l.clone().rest())?;

        if r.is_empty() {
            return l.first();
        }

        if await!(l.first()?.boolean())? {
            return r.first();
        }

        l = await!(r.rest())?;
    }
}

#[cfg(test)]
mod test {
    use futures::stable::block_on_stable;

    use super::*;

    use super::super::utils::papp;

    #[test]
    fn iff() {
        for (xs, y) in vec![
            (&[42.into()], 42.into()),
            (&[true.into(), 42.into(), 0.into()], 42.into()),
            (&[false.into(), 42.into(), 0.into()], 0.into()),
            (
                &[false.into(), 42.into(), true.into(), 123.into(), 0.into()],
                123.into(),
            ),
        ]: Vec<(&[Value], Value)>
        {
            assert!(block_on_stable(papp(IF.clone(), xs).equal(y)).unwrap());
        }
    }
}
