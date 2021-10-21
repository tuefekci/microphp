mod types;
mod fs;
mod misc;
mod array;

pub use types::*;
pub use fs::*;
pub use misc::*;
pub use array::*;

macro_rules! arity {
    ($args:expr, $len:expr) => {
        if ($args.len() > $len) {
            panic!("Expected {} arguments, received {}.", $len, $args.len())
        }
    };
}

macro_rules! typecheck {
    ($arg:expr, $type:ident) => {
        if ! $arg.$type() {
            panic!("Type check failed.")
        }
    };
}

pub(crate) use arity;
pub(crate) use typecheck;