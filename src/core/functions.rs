use futures::prelude::*;

use super::result::Result;
use super::signature::Signature;
use super::value::Value;

pub use super::boolean::IF;
pub use super::collection::{INSERT, MERGE};
pub use super::list::{FIRST, PREPEND, REST};
pub use super::number::{ADD, DIVIDE, MULTIPLY, SUBTRACT};
pub use super::utils::IDENTITY;

pure_function!(
    EQUAL,
    Signature::new(vec![], vec![], "xs".into(), vec![], vec![], "".into()),
    equal
);

#[async_move(boxed_send)]
fn equal(vs: Vec<Value>) -> Result<Value> {
    let mut l = await!(vs[0].clone().list())?;

    if l.is_empty() {
        return Ok(true.into());
    }

    let f = l.first()?;
    l = await!(l.rest())?;

    while !l.is_empty() {
        if !await!(f.clone().equal(l.first()?))? {
            return Ok(false.into());
        }

        l = await!(l.rest())?;
    }

    Ok(true.into())
}

#[cfg(test)]
mod test {
    use futures::executor::block_on;

    use super::*;

    use super::super::dictionary::Dictionary;
    use super::super::list::List;
    use super::super::normal::Normal;
    use super::super::utils::papp;

    #[test]
    fn equal_true() {
        for xs in vec![
            &[],
            &[42.into()],
            &[42.into(), 42.into()],
            &[true.into(), true.into()],
            &[
                false.into(),
                false.into(),
                false.into(),
                false.into(),
                false.into(),
            ],
            &[List::Empty.into(), List::Empty.into()],
            &[
                List::new(&[42.into()]).into(),
                List::new(&[42.into()]).into(),
            ],
            &[
                Dictionary::new()
                    .strict_insert("foo", 42)
                    .strict_insert("bar", 42)
                    .into(),
                Dictionary::new()
                    .strict_insert("bar", 42)
                    .strict_insert("foo", 42)
                    .into(),
            ],
        ]: Vec<&[Value]>
        {
            assert!(block_on(papp(EQUAL.clone(), xs).boolean()).unwrap());
        }
    }

    #[test]
    fn equal_false() {
        for xs in vec![
            &[42.into(), 0.into()],
            &[42.into(), 42.into(), Normal::Nil.into()],
            &[
                List::new(&[42.into()]).into(),
                List::new(&["foo".into()]).into(),
            ],
        ]: Vec<&[Value]>
        {
            assert!(!block_on(papp(EQUAL.clone(), xs).boolean()).unwrap());
        }
    }
}
