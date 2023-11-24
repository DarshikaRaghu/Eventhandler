use ethers::{
    contract::abigen,
    core::types::Address,
    providers::{Provider, StreamExt, Ws},
};
use eyre::Result;
use std::sync::Arc;
use dotenv::dotenv;
use serde_json;
use serde::{Serialize, Deserialize};
use std::thread;

#[derive(Deserialize)]
struct WebSocketUrl {
    url: String,
}

#[derive(Deserialize)]
struct ContractMetadata {
    contractAddress: String,
    eventNames: Vec<String>,
}

impl ContractMetadata {
    fn from_str(s: &str) -> Result<Self, serde_json::Error> {
        Ok(serde_json::from_str(s)?)
    }
}

// Generate the type-safe contract bindings by providing the ABI
// definition in human readable format
abigen!(
    ERC20,
    r#"[
        event  Transfer(address indexed src, address indexed dst, uint wad)
        event Approval(address indexed owner, address indexed spender, uint wad)
    ]"#,
);

#[tokio::main]
async fn main() -> Result<()> {
    //loading .env file data
    dotenv().ok();
    let mut providers: Vec<&str> = Vec::new();
    let provider_env: String ;
    provider_env = std::env::var("PROVIDER").unwrap();
        // Split the comma-separated values into an array of strings
        
    providers = provider_env.split(',').collect();
    println!("{}",providers.len());
    //Reading providers url in a WebSocketUrl structure
    // let mut websocket_urls: Vec<WebSocketUrl> = Vec::new();
    // for url_str in std::env::var("PROVIDER").unwrap().split(',') {
    //     websocket_urls.push(WebSocketUrl { url: url_str.to_string() });
    // }

    //connecting each provider url
    // for provider_str in websocket_urls {
    //     let handle = thread::spawn(||{
    //         fun(provider_str);
    //     });
    //     handle.join().unwrap();
    // }
    //    let providers = providers.clone();
    let length = providers.len();
        for i in 0..length{
        let handle = thread::spawn(move || {
            // Spawn an asynchronous task using tokio
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                fun(i).await.unwrap();
            });
        });
        
        handle.join().unwrap();
        
    }
    let handle1 = thread::spawn(move || {
        // Spawn an asynchronous task using tokio
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            fun(1).await.unwrap();
        });
    });
    
    handle1.join().unwrap();
    Ok(())
}


async fn fun(i:usize)-> Result<()>{

    let mut providers: Vec<&str> = Vec::new();
    let provider_env: String ;
    provider_env = std::env::var("PROVIDER").unwrap();
        // Split the comma-separated values into an array of strings
        
    providers = provider_env.split(',').collect();
    let client =
    Provider::<Ws>::connect(providers[i]).await?;
    let client = Arc::new(client);

    //Reading Contract metadata from .env file
    let contract_metadata_str: String = std::env::var("CONTRACT_METADATA").unwrap();
    let contract_metadata: Vec<ContractMetadata> = serde_json::from_str(&contract_metadata_str).unwrap();

    for metadata in contract_metadata{
            let contract_address_str = metadata.contractAddress;
            let contract_address = contract_address_str.parse::<Address>().unwrap();
            let weth = ERC20::new(contract_address, Arc::clone(&client));
            println!("contract metadata {}",contract_address_str);

            // Subscribe events
            let events = weth.events();
            let mut stream = events.stream().await?.with_meta();
            while let Some(Ok((event, meta))) = stream.next().await {
                match event {
                    ERC20Events::ApprovalFilter(f) =>{ println!("{f:?}");
                    println!(
                    r#"address: {:?}, 
                        block_number: {:?}, 
                        block_hash: {:?}, 
                        transaction_hash: {:?}, 
                        transaction_index: {:?}, 
                        log_index: {:?}
                    "#,
                    meta.address,
                    meta.block_number,
                    meta.block_hash,
                    meta.transaction_hash,
                    meta.transaction_index,
                    meta.log_index
                    );}
        
                    ERC20Events::TransferFilter(f) =>{ println!("{f:?}");
                    println!(
                    r#"address: {:?}, 
                        block_number: {:?}, 
                        block_hash: {:?}, 
                        transaction_hash: {:?}, 
                        transaction_index: {:?}, 
                        log_index: {:?}
                    "#,
                    meta.address,
                    meta.block_number,
                    meta.block_hash,
                    meta.transaction_hash,
                    meta.transaction_index,
                    meta.log_index
                    );}
                }
            }
        
    //Hello commit
    }
    Ok(())
}