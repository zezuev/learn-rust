## Ownership

Ownership enables Rust to make memory safety guarantees without needing a garbage collector.

It is a set of rules that govern how a Rust program manages memory.

When a variable goes out of scope, Rust calls the special _drop_ function - that's where the author of a type can put the code to return the memory.

### Rules

- Each value in Rust has an _owner_
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

### Move

When copying nontrivial values, Rust makes shallow copies and invalidates the first variable. this is called a _move_.

This does not hold for stack-only data, which is always copied. If a type implements the _Copy_ trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.

Rust won't let us annotate a type with _Copy_ if the type, or any of its parts, has implemented the _Drop_ trait.

### Clone

_clone_ is used to perform deep copies of data.

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

### Functions

Passing a variable to a function will move or copy, depending on the type.
