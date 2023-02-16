use anyhow::Context;
use smart_contract_verifier_http::{init_logs, run, Settings};
use std::error::Error;
use crate::db::DB;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = Settings::new().context("failed to parse config")?;
   DB::new().db();
   init_logs(settings.jaeger.clone());
   run(settings).await?;
   
    Ok(())
}

