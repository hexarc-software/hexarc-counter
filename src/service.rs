use serde::{Serialize, Deserialize};
use mongodb::bson::{DateTime, doc};
use mongodb::Collection;
use mongodb::results::InsertOneResult;
use mongodb::error::Result;

/// This structure represents a view event of a GitHub or Web page.
#[derive(Serialize, Deserialize)]
pub struct View {
    /// The unique tracker name of this view event.
    name: String,
    /// The date time of this view event.
    date: DateTime,
}

/// The data service to add views and collect stats.
#[derive(Clone)]
pub struct ViewService {
    collection: Collection<View>,
}

impl ViewService {
    /// Create an instance of the ViewService struct from a given db collection.
    pub fn new(collection: Collection<View>) -> Self {
        Self { collection }
    }

    /// Register a view event in the storage.
    pub async fn add_view<T: AsRef<str>>(&self, name: T) -> Result<InsertOneResult> {
        let view = View {
            name: name.as_ref().to_owned(),
            date: DateTime::now(),
        };
        self.collection.insert_one(view, None).await
    }

    /// Collect the view counts for a give unique tracker name.
    pub async fn get_view_count<T: AsRef<str>>(&self, name: T) -> Result<u64> {
        self.collection
            .count_documents(doc! {"name": name.as_ref()}, None)
            .await
    }
}
