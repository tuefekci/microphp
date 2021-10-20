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