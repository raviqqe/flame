use super::super::core::{papp, Arguments, Function, Result, Signature, Value};

pure_function!(
    Y,
    Signature::new(vec!["function".into()], "".into(), vec![], "".into()),
    y
);

async fn y(vs: Vec<Value>) -> Result {
    let f = vs[0].clone();

    Ok(Function::closure(f.clone(), Arguments::positionals(&[papp(Y.clone(), &[f])])).into())
}

#[cfg(test)]
mod test {
    use std::thread::sleep;
    use std::time::Duration;

    use futures::{executor::ThreadPool, stable::block_on_stable};
    use test::Bencher;

    use run::evaluate;

    use super::*;

    use super::super::super::core::functions::{EQUAL, IF, MULTIPLY, SUBTRACT};

    pure_function!(
        FACTORIAL,
        Signature::new(vec!["me".into(), "n".into()], "".into(), vec![], "".into()),
        factorial
    );

    async fn factorial(vs: Vec<Value>) -> Result {
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
        block_on_stable(papp(Y.clone(), &[FACTORIAL.clone()]).function()).unwrap();
    }

    #[test]
    fn y_factorial() {
        for x in 0..32 {
            assert_eq!(
                block_on_stable(papp(papp(Y.clone(), &[FACTORIAL.clone()]), &[x.into()]).number())
                    .unwrap(),
                strict_factorial(x as f64)
            );
        }
    }

    #[bench]
    fn bench_y_factorial(b: &mut Bencher) {
        let f = block_on_stable(papp(Y.clone(), &[FACTORIAL.clone()]).function()).unwrap();

        b.iter(|| block_on_stable(papp(f.clone().into(), &[100.into()]).number()).unwrap());
    }

    pure_function!(
        INFINITY,
        Signature::new(vec!["me".into()], "".into(), vec![], "".into()),
        infinity
    );

    async fn infinity(vs: Vec<Value>) -> Result {
        Ok(papp(vs[0].clone(), &[]))
    }

    #[test]
    fn infinite_recursion() {
        ThreadPool::new()
            .unwrap()
            .spawn_pinned(evaluate(papp(papp(Y.clone(), &[INFINITY.clone()]), &[])))
            .unwrap();

        sleep(Duration::from_secs(10));
    }

    pure_function!(
        DECREMENT_TO_0,
        Signature::new(vec!["me".into(), "n".into()], "".into(), vec![], "".into()),
        decrement_to_0
    );

    async fn decrement_to_0(vs: Vec<Value>) -> Result {
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
            block_on_stable(papp(f.clone(), &[1000.into()]).pured()).unwrap();
        });
    }
}
