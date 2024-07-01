fn main() {
    println!("Hello, world!");
    // s comes into scope
    let s = String::from("hello");
    // s's value moves into the function
    takes_ownership(s);
    // nothing special happens, since s was moved

    let s2 = String::from("christmas");
    let len = calculate_length(&s2);
    println!("The length of '{s2}' is {len}.");

    let mut s3 = String::from("hello");
    change(&mut s3);
    println!("{s3}");

    let x = 5;
    // x is not moved into y but cloned instead, because integers implement the Copy trait
    let y = x;

    let my_string = String::from("hello world");
    // this function works on slices of Strings
    let word = first_word_slice(&my_string[..]);
    // and also on regular String references
    let word = first_word_slice(&my_string);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
    // some_string goes out of scope and 'drop' is called
}

// use a reference with & instead of taking ownership
fn calculate_length(s: &String) -> usize {
    s.len()
    // s goes out of scope. Because it has no ownership, the value pointer to by the reference is
    // not dropped
}

fn change(some_string: &mut String) {
    // mutable references allow us to modify borrowed values
    some_string.push_str(", world");
}

fn first_word_idx(s: &String) -> usize {
    let bytes = s.as_bytes();

    // this works just like Python's enumerate
    for (i, &item) in bytes.iter().enumerate() {
        // this is the byte literal syntax
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// we are now returning a String slice instead
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    // the compiler will now ensure that references into the String remain valid
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
