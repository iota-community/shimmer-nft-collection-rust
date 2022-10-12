// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example send_nft --release
// In this example we will send an nft
// Rename `.env.example` to `.env` first

use std::{env, str::FromStr};

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

    let outputs = vec![AddressAndNftId {
        address: "rms1qpts4zvd6pwymcuz9rd2245nt0t47juns8zes8w38krd0kqatmea2yul4jd".to_string(),
        // Replace with an NftId that is available in the account
        nft_id: NftId::from_str("0xcc3fa4d255dfeb6e50f02a845c9d1a9597defecea86c801ef9930af935b49bf4")?,
    }];

    let transaction = account.send_nft(outputs, None).await?;

    println!(
        "Transaction: {} Block sent: {}/api/core/v2/blocks/{}",
        transaction.transaction_id,
        &env::var("NODE_URL").unwrap(),
        transaction.block_id.expect("no block created yet")
    );

    Ok(())
}