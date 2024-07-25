use secp256k1::SecretKey;
use std::str::FromStr;
use web3::contract::Contract;
use web3::ethabi::Token;
use web3::transports::Http;
use web3::types::{Address, TransactionParameters, H256, U256};

pub async fn disperse_eth(
    web3: web3::Web3<Http>,
    contract_address: Address,
    // from: Address,
    recipients: Vec<Address>,
    values: Vec<U256>,
    private_key: &str,
) -> web3::Result<H256> {
    let contract = Contract::from_json(
        web3.eth(),
        contract_address,
        include_bytes!("../build/Contract.abi"),
    )
    .map_err(|e| web3::Error::Decoder(e.to_string()))?;

    // Set Token arr
    let tokens: Vec<Token> = vec![
        Token::Array(recipients.into_iter().map(Token::Address).collect()),
        Token::Array(values.into_iter().map(Token::Uint).collect()),
    ];

    // Encode disperseETH function call
    let tx_data = contract
        .abi()
        .function("disperseETH")
        .map_err(|e| web3::Error::InvalidResponse(e.to_string()))?
        .encode_input(&tokens)
        .map_err(|e| web3::Error::Decoder(e.to_string()))?;

    let tx = TransactionParameters {
        to: Some(contract_address),
        value: U256::zero(),
        gas: U256::from(21000), // Set appropriate gas limit
        gas_price: Some(web3.eth().gas_price().await?),
        data: tx_data.into(),
        ..Default::default()
    };

    // Parse the private key and sign the transaction
    let secret_key = SecretKey::from_str(private_key).expect("Private key is invalid!");
    let signed_tx = web3.accounts().sign_transaction(tx, &secret_key).await?;

    // Send the signed transaction
    let result = web3
        .eth()
        .send_raw_transaction(signed_tx.raw_transaction)
        .await?;

    Ok(result)
}

pub async fn disperse_erc20(
    web3: web3::Web3<Http>,
    contract_address: Address,
    // from: Address,
    token_address: Address,
    recipients: Vec<Address>,
    values: Vec<U256>,
    total_amount: U256,
    private_key: &str,
) -> web3::Result<H256> {
    let contract = Contract::from_json(
        web3.eth(),
        contract_address,
        include_bytes!("../build/Contract.abi"),
    )
    .map_err(|e| web3::Error::Decoder(e.to_string()))?;

    // Set Token arr
    let tokens: Vec<Token> = vec![
        Token::Address(token_address),
        Token::Array(recipients.into_iter().map(Token::Address).collect()),
        Token::Array(values.into_iter().map(Token::Uint).collect()),
        Token::Uint(total_amount),
    ];

    // Encode disperseERC20Percent function call
    let tx_data = contract
        .abi()
        .function("disperseERC20Percent")
        .map_err(|e| web3::Error::InvalidResponse(e.to_string()))?
        .encode_input(&tokens)
        .map_err(|e| web3::Error::Decoder(e.to_string()))?;

    let tx = TransactionParameters {
        to: Some(contract_address),
        value: U256::zero(),
        gas: U256::from(21000), // Set appropriate gas limit
        gas_price: Some(web3.eth().gas_price().await?),
        data: tx_data.into(),
        ..Default::default()
    };

    // Parse the private key and sign the transaction
    let secret_key = SecretKey::from_str(private_key).expect("Private key is invalid!");
    let signed_tx = web3.accounts().sign_transaction(tx, &secret_key).await?;

    // Send the signed transaction
    let result = web3
        .eth()
        .send_raw_transaction(signed_tx.raw_transaction)
        .await?;

    Ok(result)
}

pub async fn collect_eth(
    web3: web3::Web3<Http>,
    contract_address: Address,
    // from: Address,
    senders: Vec<Address>,
    recipient: Address,
    private_key: &str,
) -> web3::Result<H256> {
    let contract = Contract::from_json(
        web3.eth(),
        contract_address,
        include_bytes!("../build/Contract.abi"),
    )
    .map_err(|e| web3::Error::Decoder(e.to_string()))?;

    // Set token arr
    let tokens: Vec<Token> = vec![
        Token::Array(senders.into_iter().map(Token::Address).collect()),
        Token::Address(recipient),
    ];

    // Encode collectETH function call
    let tx_data = contract
        .abi()
        .function("collectETH")
        .map_err(|e| web3::Error::InvalidResponse(e.to_string()))?
        .encode_input(&tokens)
        .map_err(|e| web3::Error::Decoder(e.to_string()))?;

    let tx = TransactionParameters {
        to: Some(contract_address),
        value: U256::zero(),
        gas: U256::from(21000), // Set appropriate gas limit
        gas_price: Some(web3.eth().gas_price().await?),
        data: tx_data.into(),
        ..Default::default()
    };

    // Parse the private key and sign the transaction
    let secret_key = SecretKey::from_str(private_key).expect("Private key is invalid!");
    let signed_tx = web3.accounts().sign_transaction(tx, &secret_key).await?;

    // Send the signed transaction
    let result = web3
        .eth()
        .send_raw_transaction(signed_tx.raw_transaction)
        .await?;

    Ok(result)
}

pub async fn collect_erc20(
    web3: web3::Web3<Http>,
    contract_address: Address,
    from: Address,
    token_address: Address,
    senders: Vec<Address>,
    recipient: Address,
    percentage: u8,
    private_key: &str,
) -> web3::Result<H256> {
    let contract = Contract::from_json(
        web3.eth(),
        contract_address,
        include_bytes!("../build/Contract.abi"),
    )
    .map_err(|e| web3::Error::Decoder(e.to_string()))?;

    // // Set Token arr
    let tokens: Vec<Token> = vec![
        Token::Address(token_address),
        Token::Array(senders.into_iter().map(Token::Address).collect()),
        Token::Address(recipient),
        Token::Uint(U256::from(percentage)),
    ];

    // Encode collectERC20Percent function call
    let tx_data = contract
        .abi()
        .function("collectERC20Percent")
        .map_err(|e| web3::Error::InvalidResponse(e.to_string()))?
        .encode_input(&tokens)
        .map_err(|e| web3::Error::Decoder(e.to_string()))?;

    let tx = TransactionParameters {
        to: Some(contract_address),
        value: U256::zero(),
        gas: U256::from(21000), // Set appropriate gas limit
        gas_price: Some(web3.eth().gas_price().await?),
        data: tx_data.into(),
        ..Default::default()
    };

    // Parse the private key and the transaction
    let secret_key = SecretKey::from_str(private_key).expect("Private key is invalid!");
    let signed_tx = web3.accounts().sign_transaction(tx, &secret_key).await?;

    // Send the signed transaction
    let result = web3
        .eth()
        .send_raw_transaction(signed_tx.raw_transaction)
        .await?;

    Ok(result)
}
