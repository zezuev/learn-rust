## Mutability

By default, variables are immutable.

## Constants

Constants are always immutable.
They can be declared in any scope, including the global one.
The type of the value _must_ be annotated.

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

## Shadowing

You can declare a new variable with the same name as a previous variable.

The first variable is then _shadowed_ by the second.

The variable may even change types when being shadowed.

## Data Types

A scalar represents a single value.
The four primary scalar types are integers, floats, booleans, and characters.

### Integers

The _isize_ und _usize_ depend on the architecture of the system.

Integer types default to _i32_.

In the case of overflow, the program panics in debug mode. When compiling for release, 2s-complement wrapping is used. Reliance on this behavior is considered an error.

Integer division truncates.

### Floats

The default type is _f64_.

### Chars

_char_ literals are specified with single quotes.

They are four bytes in size and represent a Unicode scalar value.

## Compound Types

### Tuples

A tuple is general way of grouping together a number of values with a variety of types. They have a fixed length.

    let tup = (500, 6.4, 1);

To get the individual values of a tuple, one can use pattern matching to destructure a tuple value

    let (x, y, z) = tup;

Tuple elements can be accessed directly by using a period followed by the index of the value one wants to access.

The tuple without any values is called the _unit_. It is written () and reprsents an empty value or an empty return type. Expressions implicitly return the unit value if they don't return any other value.

### Arrays

Every element of an array must have the same type. Arrays in Rust have a fixed length.

    let a = [1, 2, 3, 4, 5];

Array data is allocated on the stack.

Arrays can be initialized to contain the same value for each element by specifying the initial value.

    let a = [3; 5];
