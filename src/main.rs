use blockfrost::{BlockfrostAPI, BlockfrostResult};
use dotenv::dotenv;

pub mod file;
use crate::file::read_address_file;


#[tokio::main]
async fn main() -> BlockfrostResult<()> {
    dotenv().ok();

    let api_key: String =
        std::env::var("BLOCKFROST_API_KEY").expect("Blockfrost API key not found");
    let api: BlockfrostAPI = build_api(&api_key)?;

    let health = api.health().await;
    print!("Health: {:#?}", health);

    let contents = read_address_file();

    // print!("Addresses: {}", contents);


    Ok(())
}

fn build_api(api_key: &str) -> BlockfrostResult<BlockfrostAPI> {
    let api = BlockfrostAPI::new(api_key, Default::default());
    Ok(api)
}
