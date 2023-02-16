use anyhow::Context;
use smart_contract_verifier_http::{init_logs, run, Settings, DB, Contract_verify_response};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   let settings = Settings::new().context("failed to parse config")?;
   //    // Get the 'movies' collection from the 'sample_mflix' database:
//    let movies = client.database("sample_mflix").collection("movies");
//    let insert_result = movies.insert_one(new_doc.clone(), None).await?;
//    println!("New document ID: {}", insert_result.inserted_id);
   let verify_database = DB::new().await;
   verify_database.change_name("evmos");
   let cvr = Contract_verify_response{
      file_name: "hello.sol".to_string()
   };
   verify_database.add_contract_verify_response(cvr);
   init_logs(settings.jaeger.clone());
   run(settings).await?;
   
   Ok(())
}

