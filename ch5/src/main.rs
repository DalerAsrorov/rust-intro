
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

fn print_user(user: &User) {
    println!("\n\temail: {}\n\tusername: {}\n\tactive: {}\n\tsign_in_count: {}", user.email, user.username, user.active, user.sign_in_count);
}

fn main() {
    // creating an instance of struct
    let mut user1 = build_user(String::from("daler@gmail.com"), String::from("daler"));

    println!("{}, {}", user1.email, user1.username);

    user1.email = String::from("daler@yahoo.com");

    print_user(&user1);

    // Struct Update Syntax
    let user2 = User {
        email: String::from("jake@gmail.com"),
        username: String::from("jake"),
        ..user1
    };

    print_user(&user2);
}
