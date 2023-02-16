use serde::{Deserialize, Serialize};

/// This is response struct for contract verification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Contract_verify_response {
    pub file_name: String,
    pub contract_name: String,
    pub compiler_version: String,
    pub evm_version: String,
    pub constructor_arguments: String,
    pub optimization: bool,
    pub optimization_runs: String,
    pub contract_libraries: {},
    pub abi: String,
    pub sources: {},
    pub compiler_settings: "",
    pub local_creation_input_parts: [],
    pub local_deployed_bytecode_parts: []
}