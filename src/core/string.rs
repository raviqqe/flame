use std::convert::TryInto;
use std::fmt::{self, Debug, Formatter};
use std::str::from_utf8;
use std::sync::Arc;

use super::error::Error;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Str(Arc<[u8]>);

impl Str {
    pub fn merge(&self, s: &Self) -> Self {
        let mut v = Vec::with_capacity(self.0.len() + s.0.len());
        v.extend_from_slice(&self.0);
        v.extend_from_slice(&s.0);
        Str(v.into())
    }

    pub fn split(&self, i: usize) -> (Self, Self) {
        let (f, l) = self.0.split_at(i);
        (Str(f.into()), Str(l.into()))
    }

    fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl Debug for Str {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", String::from_utf8_lossy(self.into()))
    }
}

impl Default for Str {
    fn default() -> Self {
        Str((&[] as &[u8]).into())
    }
}

impl<'a> Into<&'a [u8]> for &'a Str {
    fn into(self) -> &'a [u8] {
        &self.0
    }
}

impl<'a> From<&'a str> for Str {
    fn from(s: &'a str) -> Self {
        Str(s.as_bytes().into())
    }
}

impl From<String> for Str {
    fn from(s: String) -> Self {
        Str(s.as_bytes().into())
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
