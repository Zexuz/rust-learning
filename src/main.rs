use person::Person;

mod person;

fn main() {
    let person = Person::from_stdin();

    println!("{}", person);
}
