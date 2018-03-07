#[cfg(test)]
mod test {
    use std::sync::Arc;

    use futures::prelude::*;
    use test::Bencher;

    use super::super::core::{Normal, Result};

    fn normal_function() -> Result {
        Ok(Normal::Nil.into())
    }

    #[bench]
    fn bench_normal_function(b: &mut Bencher) {
        b.iter(|| normal_function().unwrap());
    }

    #[async]
    fn async_function() -> Result {
        Ok(Normal::Nil.into())
    }

    #[bench]
    fn bench_async_function(b: &mut Bencher) {
        b.iter(|| async_function().wait().unwrap());
    }

    #[async(boxed_send)]
    fn boxed_async_function() -> Result {
        Ok(Normal::Nil.into())
    }

    #[bench]
    fn bench_boxed_async_function(b: &mut Bencher) {
        b.iter(|| boxed_async_function().wait().unwrap());
    }

    #[bench]
    fn bench_box(b: &mut Bencher) {
        b.iter(|| Box::new(Ok(Normal::Nil.into()): Result).unwrap());
    }

    #[bench]
    fn bench_arc(b: &mut Bencher) {
        b.iter(|| {
            Arc::try_unwrap(Arc::new(Ok(Normal::Nil.into()): Result))
                .unwrap()
                .unwrap()
        });
    }
}
