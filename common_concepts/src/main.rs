fn main() {
    println!("Hello, world!");

    another_function();

    yet_another_function(45);

    expressions();

    println!("The 100th Fibonacci number is {}", fib(100));
}

// Rust uses snake case conventionally
// It doesn't care where you define your functions, as long as they're somewhere in the scope that
// can be seen by the caller
fn another_function() {
    println!("Another function.");
}

// In function signatures, you must declare the type of each parameter
fn yet_another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn expressions() {
    // Calling a function is an expression. Calling a macro is an expression. A new scope block
    // created within curly brackets is an expression
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

// We must declare the types of function return values with an arrow (->)
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn ternary(cond: bool) -> i32 {
    let number = if cond { 5 } else { 6 };
    number
}

fn return_from_loop() {
    // Values can be returned from loops by adding them after the break expression
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}

fn countdown() {
    // the 4 is excluded
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!");
}

// EXERCISES

// Linear time Fibonacci numbers
fn fib(n: usize) -> i128 {
    let mut f1 = 0;
    let mut f2 = 1;
    for _ in (0..n) {
        let temp = f1 + f2;
        f1 = f2;
        f2 = temp;
    }
    return f2;
}
