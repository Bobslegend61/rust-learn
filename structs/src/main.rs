fn main() {
    let user1 = User {
        active: false,
        username: String::from("Emmanuel"),
        email: String::from("bobslegend61@gmail.com"),
        sign_in_count: 1
    };

    println!("Email is {}", user1.email);

    let user2 = User {
        email: String::from("alabiimedia@gmail.com"),
        ..user1
    };

    println!("Email is {} and username is {}", user2.email, user2.username);
    println!("User1 Email is {} and active is {} and signed is {} times", user1.email, if user1.active { "Active" } else { "Inactive" }, user1.sign_in_count);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u32
}

