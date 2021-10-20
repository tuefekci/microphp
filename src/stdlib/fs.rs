use super::{arity, typecheck};
use crate::object::Object;
use std::path::Path;
use crate::vm::Machine;

pub fn basename(_: &mut Machine, args: Vec<Object>) -> Object {
    arity!(args, 1);

    let path = args.first().unwrap();

    typecheck!(path, is_string);

    let path = path.to_string();
    let path = Path::new(&path);

    let basename = path.file_name().and_then(|s| s.to_str()).unwrap();

    Object::String(basename.into())
}