use std::collections::HashMap;

fn main() {
    let mut score = HashMap::new();

    score.insert(String::from("Blue"), 10);
    score.insert(String::from("Yellow"), 50);

    for (key, value) in &score {
        println!("Key: {}\nValue: {}", key, value);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
