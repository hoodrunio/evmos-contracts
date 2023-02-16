use anyhow::Context;
use smart_contract_verifier_http::{init_logs, run, Settings, DB};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = Settings::new().context("failed to parse config")?;
   DB::new();
   init_logs(settings.jaeger.clone());
   run(settings).await?;
   
    Ok(())
}

