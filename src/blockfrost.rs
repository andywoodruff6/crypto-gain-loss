// let api_key: String =
    //     std::env::var("BLOCKFROST_API_KEY").expect("Blockfrost API key not found");
    // let api: BlockfrostAPI = build_api(&api_key)?;
    // let pagination = Pagination::default();

    // let health = api.health().await;
    // print!("Health: {:#?}", health);

    // let contents = file::read_address_file();

    // // returns a list of transactions. Will need to iterate over the list to get the transaction amount.
    // let addresses_transactions = api.addresses_transactions(&contents[0], pagination).await;

    // print!("Address transactions: {:#?}", addresses_transactions);

// fn build_api(api_key: &str) -> BlockfrostResult<BlockfrostAPI> {
//     let api = BlockfrostAPI::new(api_key, Default::default());
//     Ok(api)
// }