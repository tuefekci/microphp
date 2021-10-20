use crate::object::Object;
use crate::vm::Machine;
use super::arity;

pub fn is_string(_: &mut Machine, args: Vec<Object>) -> Object {
    arity!(args, 1);

    let subject = args.first().unwrap();

    Object::from_bool(subject.is_string())
}

pub fn strval(_: &mut Machine, args: Vec<Object>) -> Object {
    arity!(args, 1);

    let subject = args.first().unwrap();

    Object::String(subject.to_string())
}