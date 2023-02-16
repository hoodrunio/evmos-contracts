use serde::{Deserialize, Serialize};

/// This is response struct for contract verification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Contract_verify_response {
    pub file_name: String,
    pub contract_name: String,
    compiler_version: String,
    evm_version: String,
    constructor_arguments: String,
    optimization: bool,
    optimization_runs: String,
    contract_libraries: {},
    abi: String,
    sources: {},
    compiler_settings: "",
    local_creation_input_parts: [],
    local_deployed_bytecode_parts: []
}