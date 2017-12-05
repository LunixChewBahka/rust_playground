#[allow(unused_variables)]

// `User` struct definition
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    // of `User` struct instantiation
    // key: value pairs; no need to arrange the fields according to how the
    // struct was defined
    let user1 = User {
        email: String::from("some@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("user2another@example.com"),
        username: String::from("anotherusernamebyuser249832432"),
        // implicit declaration of other fields inherited from user1
        // shortcut :/
        ..user1
    };

    let user3 = build_user("lunixchewbahka@gmail.com".to_owned(),
                           "lunixchewbahka".to_owned());
    println!("{:?}", user3.sign_in_count);
    println!("{:?}", user3.active);
    println!("{:?}", user3.username);
    println!("{:?}", user3.email);
    // to access this data we need to declare user1 as a mutable var
    //user1.email = String::from("anotheremail@example.com");
}
