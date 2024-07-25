use actix_web::{post, web, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use web3::transports::Http;
use web3::types::{Address, U256};

use crate::contract::*;

use crate::contract::disperse_erc20;

#[derive(Deserialize)]
struct DisperseRequest {
    recipients: Vec<String>,
    values: Vec<u64>,
    token_address: Option<String>,
    total_amount: Option<u64>,
}

#[derive(Serialize)]
struct ApiResponse {
    tx_hash: String,
}

#[post("/disperse")]
async fn disperse(req: web::Json<DisperseRequest>) -> impl Responder {
    let http = Http::new("https://mainnet.infura.io/v3/PROJECT_ID").unwrap();
    let web3 = web3::Web3::new(http);

    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");

    let recipients: Vec<Address> = req.recipients.iter().map(|r| r.parse().unwrap()).collect();
    let values: Vec<U256> = req.values.iter().map(|v| U256::from(*v)).collect();

    let contract_address = "CONTRACT_ADDRESS".parse().unwrap();
    let from: Address = "WALLET_ADDRESS".parse().unwrap();

    let tx_hash = if let Some(token_address) = &req.token_address {
        let token_address: Address = token_address.parse().unwrap();
        let total_amount = U256::from(req.total_amount.unwrap_or(0));
        disperse_erc20(
            web3,
            contract_address,
            from,
            token_address,
            recipients,
            values,
            total_amount,
            &private_key,
        )
        .await
        .unwrap()
    } else {
        disperse_eth(
            web3,
            contract_address,
            from,
            recipients,
            values,
            &private_key,
        )
        .await
        .unwrap()
    };

    web::Json(ApiResponse {
        tx_hash: format!("{:?}", tx_hash),
    })
}

#[derive(Deserialize)]
struct CollectRequest {
    senders: Vec<String>,
    recipient: String,
    token_address: Option<String>,
    percentage: Option<u8>,
}

#[post("/collect")]
async fn collect(req: web::Json<CollectRequest>) -> impl Responder {
    let http = Http::new("https://mainnet.infura.io/v3/PROJECT_ID").unwrap();
    let web3 = web3::Web3::new(http);

    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");

    let senders: Vec<Address> = req.senders.iter().map(|s| s.parse().unwrap()).collect();
    let recipient: Address = req.recipient.parse().unwrap();

    let contract_address = "CONTRACT_ADDRESS".parse().unwrap();
    let from: Address = "WALLET_ADDRESS".parse().unwrap();

    let tx_hash = if let Some(token_address) = &req.token_address {
        let token_address: Address = token_address.parse().unwrap();
        let percentage = req.percentage.unwrap_or(0);
        collect_erc20(
            web3,
            contract_address,
            from,
            token_address,
            senders,
            recipient,
            percentage,
            &private_key,
        )
        .await
        .unwrap()
    } else {
        collect_eth(
            web3,
            contract_address,
            from,
            senders,
            recipient,
            &private_key,
        )
        .await
        .unwrap()
    };

    web::Json(ApiResponse {
        tx_hash: format!("{:?}", tx_hash),
    })
}
