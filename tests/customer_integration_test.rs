// tests/integration_tests.rs
use mongodb::{bson, Client, Database, options::ClientOptions};
use customer::CustomerService;

async fn setup() -> Client {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    Client::with_options(client_options).unwrap()
}

async fn create_db(client: &Client) -> Database {
    client.database("rust_mongodb_test_db")
}

async fn clear_db(client: &Client) {
    let db = client.database("rust_mongodb_test_db");
    db.drop(None).await.unwrap();
}


#[tokio::test]
async fn test_mongodb_crud_operations() {
    // Connect to MongoDB
    let client = setup().await;
    clear_db(&client).await;
    let db = create_db(&client).await;
    let collection = db.collection("test_collection");

    let cucstomer_service = CustomerService::new().await;

    // Insert a document
    let document = bson::doc! {
        "name": "John Doe",
        "age": 30
    };

    let insert_result = collection.insert_one(document.clone(), None).await.unwrap();
    assert_ne!(insert_result.inserted_id, bson::Bson::Null);

    // Find the inserted document
    let filter = bson::doc! {
        "name": "John Doe"
    };

    let find_result = collection.find_one(filter.clone(), None).await.unwrap();
    assert!(find_result.is_some());
    assert_eq!(find_result.unwrap().get("name").as_ref().unwrap(), &document.get("name").unwrap());


    // Update the inserted document
    let update = bson::doc! {
        "$set": {
            "age": 31
        }
    };

    let update_result = collection.update_one(filter.clone(), update, None).await.unwrap();
    assert_eq!(update_result.modified_count, 1);

    // Delete the inserted document
    let delete_result = collection.delete_one(filter.clone(), None).await.unwrap();
    assert_eq!(delete_result.deleted_count, 1);
}

// teardown in case of failure
