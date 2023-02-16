use mongodb::{Client, Database, Collection, options::{ClientOptions, ResolverConfig}};
use chrono::{TimeZone, Utc};
use mongodb::bson::doc;

pub struct DB {
    /// The MongoDB client that works with a MongoDB instance.
    mongo: Client,

    /// Database name and chain name are the same.
    db_name: String,
}

impl DB {
    /// Connects to MongoDB instance at given URI and creates a client to work with that instance.
    /// # Usage
    /// ```rs
    /// let database = DB::new();
    /// ```

    pub async fn new() -> DB {
        // Load the MongoDB connection string
        let client_uri = "mongodb://127.0.0.1:27017";

        DB {
            mongo: (Client::with_uri_str(client_uri).await.expect("Cannot connect to MongoDB instance.")),
            db_name: "unexpected_db".to_string(),
        }
    }

    /// Changes the name of the database and returns a new one.
    pub fn change_name(self, db_name: &str) -> DB {
        DB {
            db_name: db_name.to_string(),
            ..self
        }
    }

    /// Returns the MongoDB database.
    /// # Usage
    /// ```rs
    /// let db = database.get_db();
    /// ```
    fn db(&self) -> Database {
        self.mongo.database(&self.db_name)
    }   

    fn collection(&self, collection_name: &str) -> Collection<Contract> {
        self.db().collection(collection_name);
    }

}
