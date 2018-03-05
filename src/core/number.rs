use futures::prelude::*;

use super::result::Result;
use super::signature::Signature;
use super::value::Value;

pure_function!(
    MULTIPLY,
    Signature::new(vec![], vec![], "ns".into(), vec![], vec![], "".into()),
    multiply
);

#[async(boxed_send)]
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
    use super::*;

    use super::super::utils::papp;

    #[test]
    fn merge() {
        for (xs, y) in vec![
            (&[], 1.0),
            (&[42.into()], 42.0),
            (&[1.into(), 2.into(), 3.into()], 6.0),
        ]: Vec<(&[Value], f64)>
        {
            assert_eq!(papp(MULTIPLY.clone(), xs).number().wait().unwrap(), y);
        }
    }
}
