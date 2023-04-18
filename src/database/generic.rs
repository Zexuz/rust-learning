use std::future::Future;
use mongodb::{bson::{Bson, Document}, bson, Collection, error::Result as MongoResult};
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use std::pin::Pin;

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
}


pub trait Repository<T> {
    type FindOneFut: Future<Output=MongoResult<Option<T>>> + Send;
    type FindFut: Future<Output=MongoResult<Vec<T>>> + Send;
    type InsertOneFut: Future<Output=MongoResult<()>> + Send;

    fn find_one(&self, filter: Document) -> Self::FindOneFut;
    fn find(&self, filter: Document) -> Self::FindFut;
    fn insert_one(&self, doc: T) -> Self::InsertOneFut;
}

pub struct MongoRepository<T> {
    collection: Collection<T>,
    phantom: std::marker::PhantomData<T>,
}

impl<T: serde::Serialize + serde::de::DeserializeOwned> MongoRepository<T> {
    pub fn new(collection: Collection<T>) -> Self {
        Self {
            collection,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T: serde::Serialize + serde::de::DeserializeOwned + Unpin + Send + Sync> Repository<T> for MongoRepository<T> {
    type FindOneFut = Pin<Box<dyn Future<Output=MongoResult<Option<T>>> + Send>>;
    type FindFut = Pin<Box<dyn Future<Output=MongoResult<Vec<T>>> + Send>>;
    type InsertOneFut = Pin<Box<dyn Future<Output=MongoResult<()>> + Send>>;

    fn find_one(&self, filter: Document) -> Self::FindOneFut {
        Box::pin(async move {
            let doc = self.collection.find_one(filter, None).await?;
            match doc {
                Some(d) => {
                    let t = bson::from_bson(Bson::Document(d))?;
                    Ok(Some(t))
                }
                None => Ok(None),
            }
        })
    }

    fn find(&self, filter: Document) -> Self::FindFut {
        Box::pin(async move {
            let cursor = self.collection.find(filter, None).await?;
            let mut results = Vec::new();
            for result in cursor {
                let doc = result?;
                let t = bson::from_bson(Bson::Document(doc))?;
                results.push(t);
            }
            Ok(results)
        })
    }

    fn insert_one(&self, doc: T) -> Self::InsertOneFut {
        Box::pin(async move {
            let bson = bson::to_bson(&doc)?;
            if let Bson::Document(document) = bson {
                self.collection.insert_one(document, None).await?;
            }
            Ok(())
        })
    }
}
