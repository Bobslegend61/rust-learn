
fn main() {
    print_label_measurement(5, 'h');
    let x = five();
    println!("The value of x is {x}");

    let x = plus_one(126);
    println!("The value of x is {x}");

    let numbers = [20, 30, 50, 100];
    for number in numbers {
        println!("The number is {number}");
    }
}

fn print_label_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}

fn five() -> i32 {
    return 5;
}

fn plus_one(x: i8) -> i8 {
    return x + 1;
}