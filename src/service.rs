use serde::{Serialize, Deserialize};
use mongodb::bson::{DateTime, doc};
use mongodb::Collection;
use mongodb::results::InsertOneResult;
use mongodb::error::Result;

#[derive(Serialize, Deserialize)]
pub struct View {
    name: String,
    date: DateTime,
}

#[derive(Clone)]
pub struct ViewService {
    collection: Collection<View>,
}

impl ViewService {
    pub fn new(collection: Collection<View>) -> Self {
        Self { collection }
    }

    pub async fn add_view<T: AsRef<str>>(&self, name: T) -> Result<InsertOneResult> {
        let view = View {
            name: name.as_ref().to_owned(),
            date: DateTime::now(),
        };
        self.collection.insert_one(view, None).await
    }

    pub async fn get_view_count<T: AsRef<str>>(&self, name: T) -> Result<u64> {
        self.collection
            .count_documents(doc! {"name": name.as_ref()}, None)
            .await
    }
}
