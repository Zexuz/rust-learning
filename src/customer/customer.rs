use std::fmt;
use mongodb::{bson};
use serde::{Deserialize, Serialize};
use crate::customer::uuid_parser::UuidWrapper;
use crate::database::mongobase::{MongoDbBase, MongoDbConfig};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub(crate) id: UuidWrapper,
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
            id: UuidWrapper::new_v4(),
            balance: 0.0,
        }
    }

    pub fn update_balance(&mut self, amount: f64) {
        self.balance += amount;
    }
}

pub struct CustomerService {
    repository: MongoDbBase<Customer>,
}

impl CustomerService {
    pub async fn new() -> Self {
        let config = MongoDbConfig::new();
        let repository = MongoDbBase::new(&config).await;
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
