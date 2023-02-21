use anyhow::Context;
use smart_contract_verifier_http::{init_logs, run, Settings};
use std::error::Error;
use web3_rpc::web3::Web3;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   let settings = Settings::new().context("failed to parse config")?;
   
   let rpc = Web3::new("http://0.0.0.0:5000".to_string());
    let r = rpc.eth_get_code("0xc00e94cb662c3520282e6f5717214004a7f26888", None).await?;
    println!("start");
    println!("{:?}", r);
    println!("end");

    
   init_logs(settings.jaeger.clone());
   run(settings).await?;
   
   Ok(())
}

