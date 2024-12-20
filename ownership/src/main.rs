fn main() {
    let s1 = String::from("Hello, World!");
    println!("s1: {s1}");

    let s2 = s1;

    println!("s2: {s2}");

    let s = String::from("Good Boy!");

    takes_ownership(s);

    let mut x = 5;

    make_copy(&mut x);
    println!("{x}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn make_copy(x: &mut i32) {
     *x = *x + 2;
    println!("{x}");
}
