use std::any::TypeId;
use std::collections::HashMap;
use mongodb::Client;
use mongodb::options::ClientOptions;

pub struct MongoDbBase<T> {
    client: mongodb::Client,
    db: mongodb::Database,
    pub collection: mongodb::Collection<T>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Collections {
    Customer,
}

struct Mapper {
    dict: HashMap<TypeId, &'static str>,
}

impl Mapper {
    fn new() -> Self {
        let mut dict = HashMap::new();
        dict.insert(Collections::Orders, "orders");
        dict.insert(Collections::Payments, "payments");

        Self { dict }
    }

    fn try_get_collection_name(&self, collection: TypeId) -> Option<&&'static str> {
        self.dict.get(&collection)
    }
}


impl<T> MongoDbBase<T> {
    pub async fn new(config: &MongoDbConfig) -> Self {
        let client_options = ClientOptions::parse(config.get_connection_string()).await.unwrap();
        let client = Client::with_options(client_options).unwrap();

        let db = client.database(&config.db_name);

        let mapper = Mapper::new();
        let type_value = TypeId::of::<T>();
        let collectionName =  mapper.try_get_collection_name(type_value).unwrap();

        if String::is_empty(collectionName)  {
            let errorMsg = format!("Collection name not found for {:?}", type_value);
            panic!();
        };

        let collection = db.collection::<T>("customers");
        MongoDbBase {
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