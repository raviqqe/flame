#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Expansion {
    Expanded = 0,
    Unexpanded = 1,
}

impl From<u8> for Expansion {
    fn from(b: u8) -> Self {
        match b {
            0 => Expansion::Expanded,
            1 => Expansion::Unexpanded,
            _ => unreachable!(),
        }
    }
}
