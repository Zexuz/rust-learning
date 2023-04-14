use std::fmt;
use mongodb::{bson, Client, options::ClientOptions};


#[derive(Debug, Clone)]
pub struct Customer {
    pub(crate) id: uuid::Uuid,
    pub(crate) balance: f64,
}


impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID: {}, Balance: {}", self.id, self.balance)
    }
}

impl Customer {
    pub fn new() -> Customer {
        Customer {
            id: uuid::Uuid::new_v4(),
            balance: 0.0,
        }
    }

    pub fn update_balance(&mut self, amount: f64) {
        self.balance += amount;
    }
}

pub struct CustomerService {
    repository: CustomerRepository,
}

impl CustomerService {
    pub async fn new() -> Self {
        let config = MongoDbConfig::new();
        let repository = CustomerRepository::new(&config).await;
        CustomerService {
            repository
        }
    }

    pub async fn create_customer(&self) -> Customer {
        let customer = Customer::new();
        self.repository.collection.insert_one(customer.clone(), None).await.unwrap();
        customer
    }

    pub async fn get_customer(&self, id: uuid::Uuid) -> Option<Customer> {
        let filter = bson::doc! {
        "id": id.to_string()
    };
        self.repository.collection.find_one(filter, None).await.unwrap()
    }

    pub async fn save_customer(&self, customer: Customer) {
        println!("{}", customer.id.to_string());
        let filter = bson::doc! {
        "id": customer.id.to_string()
    };

        let serialized_customer = bson::to_bson(&customer).unwrap();
        let update_doc = bson::doc! {
        "$set": serialized_customer
    };
        self.repository.collection.update_one(filter, update_doc, None).await.unwrap();
    }
}


pub struct CustomerRepository {
    client: mongodb::Client,
    db: mongodb::Database,
    collection: mongodb::Collection::<Customer>,
}

impl CustomerRepository {
    pub async fn new(config: &MongoDbConfig) -> Self {
        let client_options = ClientOptions::parse(config.get_connection_string()).await.unwrap();
        let client = Client::with_options(client_options).unwrap();

        let db = client.database(&config.db_name);
        let collection = db.collection::<Customer>("customers");
        CustomerRepository {
            client,
            db,
            collection,
        }
    }
}

pub struct MongoDbConfig {
    pub host: String,
    pub port: i32,
    pub db_name: String,
}

impl MongoDbConfig {
    pub fn new() -> Self {
        MongoDbConfig {
            host: "localhost".to_string(),
            port: 27017,
            db_name: "rust_learning_db".to_string(),
        }
    }

    pub fn get_connection_string(&self) -> String {
        format!("mongodb://{}:{}", self.host, self.port)
    }
}
