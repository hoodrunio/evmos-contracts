use serde::{Deserialize, Serialize};
use crate::verification_response::VerificationResult;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Verified_Contract_Result {
    contract_address: String,
    result: VerificationResult
}