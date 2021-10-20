use super::{arity, typecheck};
use crate::object::Object;
use crate::vm::Machine;

pub fn define(vm: &mut Machine, mut args: Vec<Object>) -> Object {
    arity!(args, 2);

    let name = args.remove(0);
    typecheck!(name, is_string);

    let value = args.remove(0);

    vm.globals.create_constant(name.to_string(), value);
    
    Object::True
}