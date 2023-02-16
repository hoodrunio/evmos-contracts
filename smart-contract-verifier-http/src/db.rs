use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
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

        // A Client is needed to connect to MongoDB:
        // An extra line of code to work around a DNS issue on Windows:
        let options =
            ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
                .await?;
        let client = Client::with_options(options)?;
    }

    /// Returns the MongoDB database.
    /// # Usage
    /// ```rs
    /// let db = database.get_db();
    /// ```
    fn db(&self) -> Database {
        self.mongo.database(&self.db_name)
        println!("success");
    }   
}

//    // Get the 'movies' collection from the 'sample_mflix' database:
//    let movies = client.database("sample_mflix").collection("movies");
//    let insert_result = movies.insert_one(new_doc.clone(), None).await?;
//    println!("New document ID: {}", insert_result.inserted_id);
