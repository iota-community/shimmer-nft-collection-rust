// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example send_nft --release
// In this example we will send an nft
// Rename `.env.example` to `.env` first

use std::{env, str::FromStr};
use std::error::Error;
use std::io;
use std::process;
use dotenv::dotenv;
use iota_wallet::{account_manager::AccountManager, iota_client::block::output::NftId, AddressAndNftId, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    // Create the account manager
    let manager = AccountManager::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = manager.get_account("Alice").await?;

    // Set the stronghold password
    manager
        .set_stronghold_password(&env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // Sync and get the balance
    let _account_balance = account.sync(None).await?;
        // If already synced, just get the balance
    let account_balance = account.balance().await?;
    
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for (i, result) in rdr.records().enumerate() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result.unwrap_or_else(|e| { 
            println!("Error: {}", e);
            // Let's assume that an Err from func_result should end programme.
            process::exit(1);
        });
        println!("Send NFT to: {:#}", &record[0]);
        let outputs = vec![AddressAndNftId {
            address: (&record[0]).to_string(),
            // Replace with an NftId that is available in the account
            nft_id: account_balance.nfts[i],
        }];
    
        let transaction = account.send_nft(outputs, None).await?;
    
        println!(
            "Transaction: {} Block sent: {}/api/core/v2/blocks/{}",
            transaction.transaction_id,
            &env::var("NODE_URL").unwrap(),
            transaction.block_id.expect("no block created yet")
        );

    }

    

    Ok(())
}