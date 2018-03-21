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

#[async_move(boxed_send)]
fn y(vs: Vec<Value>) -> Result {
    let f = vs[0].clone();

    Ok(Function::closure(f.clone(), Arguments::positionals(&[papp(Y.clone(), &[f])])).into())
}

#[cfg(test)]
mod test {
    use std::thread::sleep;
    use std::time::Duration;

    use futures::executor::{block_on, ThreadPool};
    use test::Bencher;

    use run::evaluate;

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

    #[async_move(boxed_send)]
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
        block_on(papp(Y.clone(), &[FACTORIAL.clone()]).function()).unwrap();
    }

    #[test]
    fn y_factorial() {
        for x in 0..32 {
            assert_eq!(
                block_on(papp(papp(Y.clone(), &[FACTORIAL.clone()]), &[x.into()]).number())
                    .unwrap(),
                strict_factorial(x as f64)
            );
        }
    }

    #[bench]
    fn bench_y_factorial(b: &mut Bencher) {
        let f = block_on(papp(Y.clone(), &[FACTORIAL.clone()]).function()).unwrap();

        b.iter(|| block_on(papp(f.clone().into(), &[50.into()]).number()).unwrap());
    }

    pure_function!(
        INFINITY,
        Signature::new(
            vec!["me".into()],
            vec![],
            "".into(),
            vec![],
            vec![],
            "".into()
        ),
        infinity
    );

    #[async_move(boxed_send)]
    fn infinity(vs: Vec<Value>) -> Result {
        Ok(papp(vs[0].clone(), &[]))
    }

    #[test]
    fn infinite_recursion() {
        ThreadPool::new()
            .spawn(evaluate(papp(papp(Y.clone(), &[INFINITY.clone()]), &[])))
            .unwrap();

        sleep(Duration::from_secs(10));
    }

    pure_function!(
        DECREMENT_TO_0,
        Signature::new(
            vec!["me".into(), "n".into()],
            vec![],
            "".into(),
            vec![],
            vec![],
            "".into()
        ),
        decrement_to_0
    );

    #[async_move(boxed_send)]
    fn decrement_to_0(vs: Vec<Value>) -> Result {
        let n = await!(vs[1].clone().number())?;

        if n == 0.0 {
            return Ok(Value::Nil);
        }

        Ok(papp(vs[0].clone(), &[(n - 1.0).into()]))
    }

    #[bench]
    fn bench_y_decrements(b: &mut Bencher) {
        let f = papp(Y.clone(), &[DECREMENT_TO_0.clone()]);

        b.iter(|| {
            block_on(papp(f.clone(), &[1000.into()]).pured()).unwrap();
        });
    }
}
