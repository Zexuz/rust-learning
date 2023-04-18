#[macro_use]
extern crate rocket;

use customer::customer::CustomerService;

mod customer;
mod database;

struct MongoService();


trait DocumentStorage<T> {
    fn save(&self, entity: T);
}

impl MongoService {
    fn new() -> MongoService {
        return MongoService {};
    }
}

struct Person {
    name: String,
    age: u8,
}

impl DocumentStorage<Person> for MongoService {
    fn save(&self, entity: Person) {
        todo!()
    }
}

#[get("/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    let mongoService = MongoService::new();
    mongoService.save();
    let str = format!("Hello, {} year old named {}!", age, name);
    str
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let mut customerService = CustomerService::new().await;

    let mut customer = customerService.create_customer().await;
    customer.update_balance(10.54f64);
    println!("{}", customer);
    customer.update_balance(50f64);
    println!("{}", customer);
    customerService.save_customer(customer).await;


    let _rocket = rocket::build()
        .mount("/", routes![index, hello])
        .launch()
        .await?;

    Ok(())
}

