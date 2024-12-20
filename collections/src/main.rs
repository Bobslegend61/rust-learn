#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Text(String),
    Float(f64)
}

fn main() {
    println!("Hello, world!");

    // * Vectors
    // let mut first_vector: Vec<i32> = Vec::new();
    // first_vector.push(30);
    // let second_vector = vec![1, 2, 3];
    let array = [1, 2, 3];
    let tuple = (3, 20, "Hello");


    let new_tuple = tuple;
    println!("Tuple: {:?}", tuple);
    println!("Tuple: {:?}", new_tuple);

    for item in array {
        println!("Item: {}", item);
    }

    let new_array = array;

    println!("Array: {:?}", array);

    println!("Array: {:?}", new_array);

    let v = vec![1, 2, 3, 4, 5];
    let third_index = &v[2];
    let third_get = v.get(2);

    println!("Index: {}\nGet: {:?}\nv: {:?}", third_index, third_get, v);

    let hundred_index = &v[4];
    let hundred_get = v.get(100);

    println!("Index: {}\nGet: {:?}\nv: {:?}", hundred_index, hundred_get, v);

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // println!("The first element is: {first}");

    v.push(6);

    // println!("The first element is: {first}");
    println!("The first element is: {:?}", v);

    let vector = vec![10, 20, 30 , 40, 50];
    println!("Vector: {:?}", vector);

    let new_vector = vector;
    println!("Vector: {:?}", new_vector);

    for item in &new_vector {
        println!("Item: {}", *item);
    }

    println!("Test: {:?}", new_vector.get(1));

    let row = vec![
        SpreadSheetCell::Int(20),
        SpreadSheetCell::Text(String::from("Hello Boy!.")),
        SpreadSheetCell::Float(12.20)
    ];

    for item in &row {
        match item {
            SpreadSheetCell::Int(number) => {
                println!("Int: {number}");
            },
            SpreadSheetCell::Float(decimal) => {
                println!("Decimal: {decimal}");
            },
            SpreadSheetCell::Text(text) => {
                println!("String: {text}");
            }
        }
    }

    println!("Index 1: {:?}", &row[1]);

}
