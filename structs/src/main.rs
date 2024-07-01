fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("The user's email: {}", user1.email);

    // we can reuse attributes easily with the struct update syntax
    // after this, we can no longer use user1 since its mutable attributes move to user2
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1,
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// Rust does not allow us to mark only certain fields as mutable
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // we are using the field init shorthand here
        username,
        email,
        sign_in_count: 1,
    }
}

// Rust also supports structs with unnamed fields, called tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
