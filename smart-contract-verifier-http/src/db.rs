use mongodb::{Client, Database, Collection, options::{ClientOptions, ResolverConfig}};
use chrono::{TimeZone, Utc};
use mongodb::bson::doc;
use crate::verification_response::{VerificationResult};


/// Define cvr from resposne of smart contract verification.
/// #Usage
/// ```rs
/// cvr_$type()
/// ```

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

    /// Returns the specified collection
    /// #Usage
    /// ```rs
    /// let collect = database.collection("name_of_collection")
    /// ```
    fn cvr_collection(&self) -> Collection<VerificationResult> {
        self.db().collection("cvr")
    }

    /// Adds a new response of contract verification to the contract_verify_response collection of the database.
    /// # Usage
    /// ```rs
    /// database.add_contract_verify_response(Contract_verify_response).await;
    /// ```
    pub async fn add_contract_verify_response(&self, cvr: VerificationResult) -> Result<(), String> {
        match self.cvr_collection().insert_one(cvr, None).await {
            Ok(_) => Ok(()),
            Err(_) => Err("Cannot save the contract_verify_response.".into()),
        }
    }

}
