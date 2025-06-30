use std::str::FromStr;

use crate::types::{Response, error, success};
use axum::response::Json;
use base64::Engine;
use base64::engine::general_purpose;
use serde::{Deserialize, Serialize};
use solana_sdk::{bs58, pubkey::Pubkey, signature::Keypair, signer::Signer};
use spl_token::ID as TOKEN_PROGRAM_ID;
use spl_token::instruction::initialize_mint;

#[derive(Serialize)]
pub struct KeypairResponse {
    pub pubkey: String,
    pub secret: String,
}

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    pub mintAuthority: String,
    pub mint: String,
    pub decimals: u8,
}

#[derive(Serialize)]
pub struct TokenInstructionResponse {
    pub program_id: String,
    pub accounts: Vec<AccountMetaJson>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct AccountMetaJson {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

//
pub async fn generate_keypair() -> Json<Response<KeypairResponse>> {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();

    success(KeypairResponse { pubkey, secret })
}

pub async fn create_token(
    Json(payload): Json<CreateTokenRequest>,
) -> Json<Response<TokenInstructionResponse>> {
    let mint_authority = match Pubkey::from_str(&payload.mintAuthority) {
        Ok(pubkey) => pubkey,
        Err(_) => return error("Invalid mint authority public key"),
    };

    let mint = match Pubkey::from_str(&payload.mint) {
        Ok(pubkey) => pubkey,
        Err(_) => return error("Invalid mint public key"),
    };

    let instruction = match initialize_mint(
        &TOKEN_PROGRAM_ID,
        &mint,
        &mint_authority,
        None,
        payload.decimals,
    ) {
        Ok(instruction) => instruction,
        Err(_) => return error("Failed to create initialize mint instruction"),
    };

    let instruction_data = general_purpose::STANDARD.encode(instruction.data);

    let accounts: Vec<AccountMetaJson> = instruction
        .accounts
        .iter()
        .map(|a| AccountMetaJson {
            pubkey: a.pubkey.to_string(),
            is_signer: a.is_signer,
            is_writable: a.is_writable,
        })
        .collect();

    let response = TokenInstructionResponse {
        program_id: TOKEN_PROGRAM_ID.to_string(),
        accounts,
        instruction_data,
    };

    success(response)
}
