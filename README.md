# `microphp`

A small subset of PHP implemented in Rust.

## About

This project aims to implement a small subset of PHP's feature-set using a Rust powered parser and virtual machine.

It uses a recursive-descent parser based on binding power similar to PHP's own precedence model. The interpreter is a bytecode virtual machine where the AST is compiled into a series of bytecode operations and constants. The virtual machine is stack-based, whereas PHP's Zend Engine is a register-based machine.

Both engines share similar opcodes making implementation of features much simpler as the structure and flow of the program can essentially be replicated.

## Supported Features

* Strings (surrounded by double-quotes)
* A handful of binary and boolean operations.
* If/else statements (no `else if` support)
* While statements (along with `break`)
* User-defined functions (no default parameter values or type declarations)
* Internal / native functions

## Optimisations

* [ ] Use a `Vec<Value>` and store each variable against an index, instead of storing things in a `HashMap`.
* [ ] Intern strings to reduce memory consumption.
* [ ] Replace the `Object::Array` implementation with something smarter. Using a `HashMap` is kind of weird and it's enforcing `String` based keys which we should avoid since PHP (associative) array keys can also be integer-based (floats get casted to an integer). In my head, I feel like some sort of separated `keys` and `values` setup would make more sense. The index of the `key` correlates to the index inside of `values` where the `Object` can be found. We could also have a dedicated `ArrayKey` type that is stored inside of the the `keys` structure. The other alternative would be using the `ArrayKey` type as the key in the `HashMap`, that should be easy enough to derive `Hash` on. Then it's just a case of casting an `Object` to an `ArrayKey` (implict type-conversions for things like `true` -> `1`).