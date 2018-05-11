use super::super::core::Value;

use super::interpreter::Interpreter;

pub fn interpret(vs: Vec<Value>, bs: &[u8]) -> Value {
    Interpreter::new(vs, bs).interpret()
}

#[cfg(test)]
mod test {
    use futures::prelude::*;
    use futures::stable::block_on_stable;

    use super::super::super::core::functions::{EQUAL, IDENTITY};
    use super::super::super::core::{papp, Dictionary, List, OptionalParameter, Result, Signature};

    use super::super::ir;

    use super::*;

    pure_function!(
        IDENTITY_KEYWORD,
        Signature::new(
            vec![],
            "".into(),
            vec![OptionalParameter::new("x", 42)],
            "".into()
        ),
        identity
    );

    #[async(boxed, send)]
    fn identity(vs: Vec<Value>) -> Result {
        Ok(vs[0].clone())
    }

    #[test]
    fn interpretation() {
        for (vs, bs, v) in vec![
            (vec![42.into()], &[0], 42.into()),
            (
                vec![IDENTITY.clone(), 42.into()],
                &[0, 1, ir::Expansion::Unexpanded as u8, 1, 0, 2],
                42.into(),
            ),
            (
                vec![IDENTITY.clone(), List::new(&[42.into()]).into()],
                &[0, 1, ir::Expansion::Expanded as u8, 1, 0, 2],
                42.into(),
            ),
            (
                vec![IDENTITY_KEYWORD.clone(), "x".into(), 42.into()],
                &[0, 0, 1, ir::Expansion::Unexpanded as u8, 1, 2, 3],
                42.into(),
            ),
            (
                vec![
                    IDENTITY_KEYWORD.clone(),
                    Dictionary::new().strict_insert("x", 42).into(),
                ],
                &[0, 0, 1, ir::Expansion::Expanded as u8, 1, 2],
                42.into(),
            ),
        ]: Vec<(Vec<Value>, &[u8], Value)>
        {
            assert!(
                block_on_stable(papp(EQUAL.clone(), &[interpret(vs, bs), v]).boolean()).unwrap()
            );
        }
    }
}
