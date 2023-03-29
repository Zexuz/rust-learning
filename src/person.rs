use std::io::stdin;
use std::fmt;

pub struct Person {
    name: String,
    age: u8,
    height: u8,
    weight: u8,
}


impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}, Age: {}, Height: {}, Weight: {}", self.name, self.age, self.height, self.weight)
    }
}

impl Person {
    fn new(name: String, age: u8, height: u8, weight: u8) -> Person {
        Person {
            name,
            age,
            height,
            weight,
        }
    }

    pub(crate) fn from_stdin() -> Person {
        let mut name = String::new();
        let mut age = String::new();
        let mut height = String::new();
        let mut weight = String::new();

        println!("Enter name: ");
        stdin().read_line(&mut name).expect("Failed to read name");
        println!("Enter age: ");
        stdin().read_line(&mut age).expect("Failed to read age");
        println!("Enter height: ");
        stdin().read_line(&mut height).expect("Failed to read height");
        println!("Enter weight: ");
        stdin().read_line(&mut weight).expect("Failed to read weight");

        Person::new(
            name.trim().to_string(),
            age.trim().parse().expect("Failed to parse age"),
            height.trim().parse().expect("Failed to parse height"),
            weight.trim().parse().expect("Failed to parse weight"),
        )
    }
}
