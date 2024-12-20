#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn create(size: u32) -> Rectangle {
        return  Rectangle {
            width: size,
            height: size
        };
    }
}

struct Numbers(u32, u32);

#[derive(Debug)]
enum Message {
    Move {x: i32, y: i32},
    Write(String)
}

impl Message {
    fn call(&self) {
        println!("Self: {:?}", self);
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    NewYork,
    Florida,
    WashingtonDC,
    California
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

impl Coin {
    fn value_in_cent(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("The state is {:?}!", state);
                return 25;
            }
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main() {
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;

    println!("V4: {:?}", v4);
    println!("V6: {:?}", v6);

    let rect1 = Rectangle {
        width: 20,
        height: 30
    };

    println!("Rectangle Width: {} and Height: {} and Area: {}", rect1.width, rect1.height, rect1.area());

    let rect2 = Rectangle::create(20);
    println!("Rectangle Width: {} and Height: {} and Area: {}", rect2.width, rect2.height, rect2.area());

    let numbers = Numbers(40, 50);
    println!("First: {} and second: {}", numbers.0, numbers.1);

    let (first, second) = (40, 80);
    println!("First: {} and second: {}", first, second);

    let m = Message::Write(String::from("Hello World"));
    println!("The message is: {:?}", m);
    m.call();

    let mover = Message::Move { x: 10, y: 15 };
    println!("Mover: {:?}", mover);
    mover.call();
    Message::call(&Message::Write(String::from("Hello New")));

    let mut some_number = Some(32);
    println!("Number is {:?}", some_number);

    some_number = None;
    println!("Number is {:?}", some_number);

    let coin = Coin::Quarter(UsState::Alabama);
    let value = coin.value_in_cent();

    println!("The value in cent is {}", value);

    match m {
        Message::Write(text) => {
            println!("Text: {}", text);
        }
        Message::Move { x, y } => {
            println!("X: {}, Y: {}", x, y);
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Five: {:?}\nSix: {:?}\nNone: {:?}", five, six, none);

}
