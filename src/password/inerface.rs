use ring::digest;
use serde::{Deserialize, Serialize};
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

#[derive(Debug, Deserialize, Serialize)]
pub struct Passowrd {
    user_phone: String,
    salt: Credential,
}
