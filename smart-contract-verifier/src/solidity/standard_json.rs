use super::client::Client;
use crate::{
    compiler::Version,
    verifier::{ContractVerifier, Error, Success},
};
use bytes::Bytes;
use ethers_solc::{artifacts::output_selection::OutputSelection, CompilerInput};
use std::sync::Arc;
use web3_rpc::web3::Web3;
use actix_web::error;
use std::str::FromStr;
use crate::DisplayBytes;

#[derive(Clone)]
pub struct VerificationRequest {
    pub contract_address: String,
    pub creation_bytecode: Option<Bytes>,
    pub compiler_version: Version,

    pub content: StandardJsonContent,
}

#[derive(Clone)]
pub struct StandardJsonContent {
    pub input: CompilerInput,
}

impl From<StandardJsonContent> for CompilerInput {
    fn from(content: StandardJsonContent) -> Self {
        let mut input = content.input;

        // always overwrite output selection as it customizes what compiler outputs and
        // is not what is returned to the user, but only used internally by our service
        let output_selection = OutputSelection::default_output_selection();
        input.settings.output_selection = output_selection;

        input
    }
}

pub async fn get_Code(contract_address: &str) -> Result<Option<String>, anyhow::Error> {
    let rpc = Web3::new("https://evmos-evm.publicnode.com".to_string());
    match rpc.eth_get_code(contract_address, None).await {
        Ok(r) =>  {println!("Fetching success!"); return Ok(r.result)},
        Err(e) => {
            tracing::error!("There is no contract {}", e);
            Err(e)
        }
    }
}

pub async fn verify(client: Arc<Client>, request: VerificationRequest) -> Result<Success, Error> {
    let compiler_input = CompilerInput::from(request.content);
    let _deployed_bytecode = get_Code(request.contract_address.as_str()).await.expect("invalid address address.");
    
    let deployed_bytecode = DisplayBytes::from_str(_deployed_bytecode.expect("no deployed bytecode for this address.").as_str()).expect("invalide bytecode").0;

    let verifier = ContractVerifier::new(
        client.compilers(),
        &request.compiler_version,
        request.creation_bytecode,
        deployed_bytecode
    )?;
    let result = verifier.verify(&compiler_input).await;

    // If case of success, we allow middlewares to process success and only then return it to the caller
    let success = result?;
    if let Some(middleware) = client.middleware() {
        middleware.call(&success).await;
    }
    Ok(success)
}
