use human::{who_am_i, Gender};
pub mod human;

fn main() {
    let person = human::Human {
        first_name: String::from("Emmanuel"),
        last_name: String::from("Alabi"),
        age: 30
    };

    who_am_i(&person);

    let person2 = human::Human::create_human(person.first_name, person.last_name, person.age);
    who_am_i(&person2);

    let names = ["Alabi", "Angel"];
    println!("{}", names[0]);
    let (name, num) = ("Alabi", 9);
    println!("{} {}", name, num);

    let mut test = 0;

    loop {
        println!("{test}");
        if test >= 10 {
            break;
        }

        test += 1;
    }

    for item in names {
        println!("{}", item);
    }

    let person3 = Gender::Others(String::from("Mark"));
    println!("{person3:?}");
    person3.print();

    let persons = vec![Gender::Male, Gender::Female { name: String::from("Oluchi"), age: 20 }, Gender::Others(String::from("Ebere"))];

    for p in persons {
       p.print();
    }

}
