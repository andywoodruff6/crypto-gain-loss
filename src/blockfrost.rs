use blockfrost::{BlockfrostAPI, Pagination};
use color_eyre::eyre::{Error, Result, Ok};
use crate::file;

pub async fn get_staking_addresses() -> Result<String, Error> {
    let api_key: String = std::env::var("BLOCKFROST_API_KEY").expect("Blockfrost API key not found");
    let api: BlockfrostAPI = build_api(&api_key)?;
    let contents = file::read_address_file();

    let health: bool = is_health_ok(&api).await;
    if !health {
        panic!("Blockfrost API is not healthy");
    }

    // get stake addresses
    let stake_addresses_json = api.addresses(&contents[0]).await?;
    let stake_addresses = match stake_addresses_json.stake_address {
        Some(address) => address,
        None => panic!("No stake addresses found"),
    };
    print!("Stake addresses: {:#?}", stake_addresses);

    Ok(stake_addresses)
}

pub async fn get_stake_address_delegation_history(stake_address: String) -> Result<(), Error> {
    let api_key: String = std::env::var("BLOCKFROST_API_KEY").expect("Blockfrost API key not found");
    let api: BlockfrostAPI = build_api(&api_key)?;
    let pagination = Pagination::default();
    let sa_str = stake_address.as_str();

    let health: bool = is_health_ok(&api).await;
    if !health {
        panic!("Blockfrost API is not healthy");
    }

    // get stake address delegation history
    let stake_address_delegation_history = api.accounts_rewards(sa_str, pagination);

    Ok(())
}



async fn is_health_ok(api: &BlockfrostAPI) -> bool {
    let health = api.health().await;
    health.is_ok()
}

fn build_api(api_key: &str) -> Result<BlockfrostAPI> {
    let api = BlockfrostAPI::new(api_key, Default::default());
    Ok(api)
}