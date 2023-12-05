// enum Result<String, VarError> {
//     Ok(String),
//     Err(VarError),
// }

use anyhow::Ok;
use bdk::{Wallet, bitcoin::Network, database::MemoryDatabase, SyncOptions, blockchain::ElectrumBlockchain, electrum_client::Client, wallet::AddressIndex, miniscript::serde::de, descriptor};

fn function_that_takes_strings(s: &str) {
    println!("String: {}", s);
}

fn setup() -> anyhow::Result<String>{
    println!("Hello, {}!", "Salenga!");
    dotenv::from_filename(".env").ok();
    dotenv::dotenv().ok();

    let descriptor = std::env::var("WALLET_DESCRIPTOR")?;
    
    Ok(descriptor)
}

fn main() -> anyhow::Result<()> {
    let descriptor = match setup(){
        Ok(descriptor) => descriptor,
        Err(e) => {
            println!("Error: {}", e);
            return Ok(());
        }
    };

    // println!("Descriptor; {}", descriptor);
    // dbg!(descriptor.clone());

    // let str_desc = "wpkh([d34db33f/84h/1h/0h]tpubD6Nz";
    // let string_desc = String::from(str_desc);

    // function_that_takes_strings(&descriptor);

let wallet = Wallet::new(
    &descriptor, 
    None, 
    Network::Testnet, 
    MemoryDatabase::default(),
)?;

// let blockchain =
//     ElectrumBlockchain::from(Client::new("ssl://electrum.blockstream.info:60002")?);

// wallet.sync(&blockchain, SyncOptions::default())?;
let balance = wallet.get_balance()?;

dbg!(balance);

let address = wallet.get_address(AddressIndex::New)?;

dbg!(address);

Ok(())

}
