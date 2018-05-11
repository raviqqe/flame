#[cfg(test)]
mod test {
    use std::ops::{Generator, GeneratorState};
    use std::sync::Arc;

    use futures::prelude::*;
    use futures::stable::block_on_stable;
    use futures::stable::StableFuture;
    use test::Bencher;

    use super::super::core::{Result, Value};

    fn normal_function() -> Result {
        Ok(Value::Nil)
    }

    #[bench]
    fn bench_normal_function(b: &mut Bencher) {
        b.iter(|| normal_function().unwrap());
    }

    fn generator_function() -> impl Generator<Yield = Async<Never>, Return = Result> {
        return || {
            if false {
                yield Async::Pending;
            }

            Ok(Value::Nil): Result
        };
    }

    #[bench]
    fn bench_generator_function(b: &mut Bencher) {
        b.iter(|| {
            let mut g = generator_function();

            loop {
                match unsafe { g.resume() } {
                    GeneratorState::Complete(r) => {
                        r.unwrap();
                        break;
                    }
                    GeneratorState::Yielded(_) => {}
                }
            }
        });
    }

    #[async]
    fn async_function() -> Result {
        Ok(Value::Nil)
    }

    #[bench]
    fn bench_async_function(b: &mut Bencher) {
        b.iter(|| block_on_stable(async_function().pin()).unwrap());
    }

    #[async(boxed, send)]
    fn boxed_async_function() -> Result {
        Ok(Value::Nil)
    }

    #[bench]
    fn bench_boxed_async_function(b: &mut Bencher) {
        b.iter(|| block_on_stable(boxed_async_function()).unwrap());
    }

    #[bench]
    fn bench_box(b: &mut Bencher) {
        b.iter(|| Box::new(Ok(Value::Nil): Result).unwrap());
    }

    #[bench]
    fn bench_arc(b: &mut Bencher) {
        b.iter(|| {
            Arc::try_unwrap(Arc::new(Ok(Value::Nil): Result))
                .unwrap()
                .unwrap()
        });
    }
}
