#[macro_use]
mod function;
mod arguments;
mod vague_normal;
mod collection;
mod dictionary;
mod error;
mod half_signature;
mod list;
mod normal;
mod optional_argument;
mod result;
mod signature;
mod thunk;
mod unsafe_ref;
mod utils;
mod value;

pub use self::vague_normal::VagueNormal;
pub use self::function::{Function, Result};
pub use self::normal::Normal;
pub use self::signature::Signature;
pub use self::value::Value;
pub use self::utils::{app, papp};
