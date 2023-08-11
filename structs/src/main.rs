//STRUCTS
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//TUPLE STRUCTS
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//UNIT-LIKE STRUCTS
struct AlwaysEqual;

fn main() {
    //STRUCTS IMPLEMENTATION
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };
    //To access data from the struct, we use dot notation
    user1.email = String::from("another@example.com");

    //Ways of creating instances from other instances
    //Typical
    /* let _user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another1@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    */
    
    //Rust's Struct Update Syntax
    let user2 = User {
        email: String::from("another1@example.com"),
        ..user1     //If it uses values that doesn't implement the copy trait, user1 would be no
                    //no longer valid.
    };

    //TUPLE STRUCTS IMPLEMENTATION
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    //UNIT-LIKE STRUCTS IMPLEMENTATION
    let subject = AlwaysEqual;
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
