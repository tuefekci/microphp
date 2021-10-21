use super::{arity, typecheck};
use crate::object::Object;
use crate::vm::Machine;

pub fn count(_: &mut Machine, mut args: Vec<Object>) -> Object {
    arity!(args, 1);

    let array = args.remove(0);
    typecheck!(array, is_array);
    
    let hash = array.to_hash().borrow();

    Object::Integer(hash.len() as i64)
}