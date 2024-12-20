fn main() {
    let name = "Alabi";
    let s = name.to_string();

    println!("{name}");
    println!("String: {s}");

    let s1 = String::from("Hello ");
    let s2 = String::from("World!.");
    let s4 = format!("{s1}{s2}");
    let s3 = s1 + &s2;
    println!("{s3}");
    println!("{s4}");

    let names = ["Emma", "Alabi"];
    for name in names {
        println!("{name}");
    }

    println!("{:?}", names);
}
