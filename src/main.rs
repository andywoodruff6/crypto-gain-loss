// use chrono::prelude::*;
use color_eyre::Result;
use dotenv::dotenv;
// use std::thread;
// use std::time::Duration;

pub mod blockfrost;
pub mod database_calls;
pub mod file;
pub mod gecko;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    color_eyre::install()?;

    //----------//
    // Build and update price table
    // database_calls::create_cardano_price_table()?;

    // let mut last_date = database_calls::check_last_price_date()?;
    // let now: i64 = Local::now().timestamp();


    // while last_date <= now {
    //     println!("Date: {:#?}", &last_date);

    //     let json_response = gecko::get_cardano_history(last_date).await?;

    //     if json_response.get("status").is_some() {
    //         println!("API limit reached, sleeping for 20 seconds");
    //         thread::sleep(Duration::from_secs(20));
    //     } else {
    //         // return price as f64s
    //         let price = json_response["market_data"]["current_price"]["usd"].as_f64().unwrap();

    //         database_calls::add_price_to_table(price, last_date)?;
    //         last_date += 86400; // add 1 day in seconds
    //     }
    // }
    //----------//
    //----------//
    // Get staking addresses
    let stake_address = blockfrost::get_staking_addresses().await?;

    let staking_rewards = blockfrost::get_stake_address_delegation_history(stake_address).await?;







    //----------//
    Ok(())
}
