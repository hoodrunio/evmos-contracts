use anyhow::Context;
use smart_contract_verifier_http::{init_logs, run, Settings};
use std::error::Error;

use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = Settings::new().context("failed to parse config")?;
    init_logs(settings.jaeger.clone());
    run(settings).await?;

    // Load the MongoDB connection string from an environment variable:
   let client_uri =
      env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

   // A Client is needed to connect to MongoDB:
   // An extra line of code to work around a DNS issue on Windows:
   let options =
      ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
         .await?;
   let client = Client::with_options(options)?;

   // Print the databases in our MongoDB cluster:
   println!("Databases:");
   for name in client.list_database_names(None, None).await? {
      println!("- {}", name);
   }
   
    Ok(())
}

