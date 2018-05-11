use futures::prelude::*;

use super::result::Result;
use super::signature::Signature;
use super::value::Value;

pure_function!(
    ADD,
    Signature::new(vec![], "ns".into(), vec![], "".into()),
    add
);

#[async(boxed, send)]
fn add(vs: Vec<Value>) -> Result<Value> {
    let mut l = await!(vs[0].clone().list())?;
    let mut n = 0.0;

    while !l.is_empty() {
        let m = await!(l.first()?.number())?;
        n += m;
        l = await!(l.rest())?;
    }

    Ok(n.into())
}

pure_function!(
    SUBTRACT,
    Signature::new(vec![], "ns".into(), vec![], "".into()),
    subtract
);

#[async(boxed, send)]
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
    Signature::new(vec![], "ns".into(), vec![], "".into()),
    multiply
);

#[async(boxed, send)]
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

pure_function!(
    DIVIDE,
    Signature::new(vec![], "ns".into(), vec![], "".into()),
    divide
);

#[async(boxed, send)]
fn divide(vs: Vec<Value>) -> Result<Value> {
    let mut l = await!(vs[0].clone().list())?;
    let mut n = await!(l.first()?.number())?;
    l = await!(l.rest())?;

    while !l.is_empty() {
        let m = await!(l.first()?.number())?;
        n /= m;
        l = await!(l.rest())?;
    }

    Ok(n.into())
}

#[cfg(test)]
mod test {
    use futures::stable::block_on_stable;

    use super::*;

    use super::super::utils::papp;

    #[test]
    fn add() {
        for (xs, y) in vec![
            (&[], 0.0),
            (&[42.into()], 42.0),
            (&[1.into(), 2.into(), 3.into()], 6.0),
        ]: Vec<(&[Value], f64)>
        {
            assert_eq!(block_on_stable(papp(ADD.clone(), xs).number()).unwrap(), y);
        }
    }

    #[test]
    fn subtract() {
        for (xs, y) in vec![
            (&[0.into()], 0.0),
            (&[42.into(), 1.into()], 41.0),
            (&[1.into(), 2.into(), 3.into()], -4.0),
        ]: Vec<(&[Value], f64)>
        {
            assert_eq!(
                block_on_stable(papp(SUBTRACT.clone(), xs).number()).unwrap(),
                y
            );
        }
    }

    #[test]
    fn subtract_error() {
        assert!(block_on_stable(papp(SUBTRACT.clone(), &[]).number()).is_err());
    }

    #[test]
    fn multiply() {
        for (xs, y) in vec![
            (&[], 1.0),
            (&[42.into()], 42.0),
            (&[1.into(), 2.into(), 3.into()], 6.0),
        ]: Vec<(&[Value], f64)>
        {
            assert_eq!(
                block_on_stable(papp(MULTIPLY.clone(), xs).number()).unwrap(),
                y
            );
        }
    }

    #[test]
    fn divide() {
        for (xs, y) in vec![
            (&[0.into()], 0.0),
            (&[42.into(), 1.into()], 42.0),
            (&[1.into(), 2.into()], 0.5),
            (&[1.into(), 2.into(), 2.into()], 0.25),
        ]: Vec<(&[Value], f64)>
        {
            assert_eq!(
                block_on_stable(papp(DIVIDE.clone(), xs).number()).unwrap(),
                y
            );
        }
    }

    #[test]
    fn divide_error() {
        assert!(block_on_stable(papp(DIVIDE.clone(), &[]).number()).is_err());
    }
}
