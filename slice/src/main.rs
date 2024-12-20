fn main() {
    let s1 = String::from("Hello World!");

    let s2 = first_word(&s1);

    println!("{s2}");

    let s3 = "Hello Boy";

}

fn first_word(s: &str) -> &str {
    let byte = s.as_bytes();

    for (i, &item) in byte.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s;
}