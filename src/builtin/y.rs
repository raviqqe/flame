use futures::prelude::*;

use super::super::core::{papp, Arguments, Function, Result, Signature, Value};

pure_function!(
    Y,
    Signature::new(
        vec!["function".into()],
        vec![],
        "".into(),
        vec![],
        vec![],
        "".into()
    ),
    y
);

#[async(boxed_send)]
fn y(vs: Vec<Value>) -> Result {
    let f = vs[0].clone();

    Ok(Function::closure(f.clone(), Arguments::positionals(&[papp(Y.clone(), &[f])])).into())
}

#[cfg(test)]
mod test {
    use super::*;

    use super::super::super::core::functions::{EQUAL, IF, MULTIPLY, SUBTRACT};

    pure_function!(
        FACTORIAL,
        Signature::new(
            vec!["me".into(), "n".into()],
            vec![],
            "".into(),
            vec![],
            vec![],
            "".into()
        ),
        factorial
    );

    #[async(boxed_send)]
    fn factorial(vs: Vec<Value>) -> Result {
        let f = vs[0].clone();
        let n = vs[1].clone();

        Ok(papp(
            IF.clone(),
            &[
                papp(EQUAL.clone(), &[n.clone(), 0.into()]),
                1.into(),
                papp(
                    MULTIPLY.clone(),
                    &[
                        n.clone(),
                        papp(f, &[papp(SUBTRACT.clone(), &[n, 1.into()])]),
                    ],
                ),
            ],
        ))
    }

    fn strict_factorial(n: f64) -> f64 {
        if n == 0.0 {
            return 1.0;
        }

        n * strict_factorial(n - 1.0)
    }

    #[test]
    fn recursive_function() {
        papp(Y.clone(), &[FACTORIAL.clone()])
            .function()
            .wait()
            .unwrap();
    }

    #[test]
    fn y_factorial() {
        for x in vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 64.0, 256.0] {
            assert_eq!(
                papp(papp(Y.clone(), &[FACTORIAL.clone()]), &[x.into()])
                    .number()
                    .wait()
                    .unwrap(),
                strict_factorial(x)
            );
        }
    }
}
