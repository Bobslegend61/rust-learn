pub struct Human {
    pub first_name: String,
    pub last_name: String,
    pub age: u8
}

impl Human {
    pub fn create_human(first_name: String, last_name: String, age: u8) -> Human {
        return Human {
            first_name,
            last_name,
            age
        }
    }

    fn get_name(&self) -> String {
        return format!("{} {}", self.first_name, self.last_name)
    }
}

pub fn who_am_i(human: &Human) {
    println!("My name is {} and i'm {} years old.", human.get_name(), human.age);
}

#[derive(Debug)]
pub enum Gender {
    Male,
    Female {
        name: String,
        age: u8
    },
    Others(String)
}

impl Gender {
   pub fn print(&self) {
        match self {
            Gender::Female { name, age } => {
                println!("Name: {}, age: {}", name, age);
            },
            Gender::Male => {
                println!("Male");
            },
            Gender::Others(name) => {
                println!("{name}");
            }
        }
    }
}