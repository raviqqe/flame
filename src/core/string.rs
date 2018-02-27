use std::convert::TryInto;
use std::str::from_utf8;

use super::error::Error;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Str(Vec<u8>);

impl Str {
    pub fn merge(&self, s: &Self) -> Self {
        let mut v = self.0.clone();
        v.extend(&s.0);
        Str(v)
    }

    pub fn split(&self, i: usize) -> (Self, Self) {
        let mut v = self.0.clone();
        let w = v.split_off(i);
        (Str(v), Str(w))
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl<'a> Into<&'a [u8]> for &'a Str {
    fn into(self) -> &'a [u8] {
        &self.0
    }
}

impl<'a> From<&'a str> for Str {
    fn from(s: &'a str) -> Self {
        Str(s.as_bytes().to_vec())
    }
}

impl From<String> for Str {
    fn from(s: String) -> Self {
        Str(s.as_bytes().to_vec())
    }
}

impl TryInto<String> for Str {
    type Error = Error;

    fn try_into(self) -> Result<String, Self::Error> {
        Ok(from_utf8(self.as_slice())?.into())
    }
}

impl<'a> PartialEq<&'a str> for Str {
    fn eq(&self, x: &&'a str) -> bool {
        self.as_slice() == x.as_bytes()
    }
}

impl PartialEq<String> for Str {
    fn eq(&self, x: &String) -> bool {
        self.as_slice() == x.as_bytes()
    }
}
