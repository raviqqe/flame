use futures::prelude::*;

use super::result::Result;
use super::signature::Signature;
use super::value::Value;

pure_function!(
    SUBTRACT,
    Signature::new(vec![], vec![], "ns".into(), vec![], vec![], "".into()),
    subtract
);

#[async_move(boxed_send)]
fn subtract(vs: Vec<Value>) -> Result<Value> {
    let mut l = await!(vs[0].clone().list())?;
    let mut n = await!(l.first()?.number())?;
    l = await!(l.rest())?;

    while !l.is_empty() {
        let m = await!(l.first()?.number())?;
        n -= m;
        l = await!(l.rest())?;
    }

    Ok(n.into())
}

pure_function!(
    MULTIPLY,
    Signature::new(vec![], vec![], "ns".into(), vec![], vec![], "".into()),
    multiply
);

#[async_move(boxed_send)]
fn multiply(vs: Vec<Value>) -> Result<Value> {
    let mut l = await!(vs[0].clone().list())?;
    let mut n: f64 = 1.0;

    while !l.is_empty() {
        let m = await!(l.first()?.number())?;
        n *= m;
        l = await!(l.rest())?;
    }

    Ok(n.into())
}

#[cfg(test)]
mod test {
    use futures::executor::block_on;

    use super::*;

    use super::super::utils::papp;

    #[test]
    fn subtract() {
        for (xs, y) in vec![
            (&[0.into()], 0.0),
            (&[42.into(), 1.into()], 41.0),
            (&[1.into(), 2.into(), 3.into()], -4.0),
        ]: Vec<(&[Value], f64)>
        {
            assert_eq!(block_on(papp(SUBTRACT.clone(), xs).number()).unwrap(), y);
        }
    }

    #[test]
    fn subtract_error() {
        assert!(block_on(papp(SUBTRACT.clone(), &[]).number()).is_err());
    }

    #[test]
    fn multiply() {
        for (xs, y) in vec![
            (&[], 1.0),
            (&[42.into()], 42.0),
            (&[1.into(), 2.into(), 3.into()], 6.0),
        ]: Vec<(&[Value], f64)>
        {
            assert_eq!(block_on(papp(MULTIPLY.clone(), xs).number()).unwrap(), y);
        }
    }
}
