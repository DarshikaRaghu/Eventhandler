use ethers::{
    contract::abigen,
    core::types::Address,
    providers::{Provider, StreamExt, Ws},
};
use eyre::Result;
use std::sync::Arc;

abigen!(
    IERC20,
    r#"[
        event Transfer(address indexed from, address indexed to, uint256 value)
        event Approval(address indexed owner, address indexed spender, uint256 value)
    ]"#,
);

const WSS_URL: &str = "wss://eth-sepolia.g.alchemy.com/v2/WNWhj53VsCFhmiAKeo1mI5DVjtXQBlNk";
const WETH_ADDRESS: &str = "0xF4F37329561cAc2142374B109c0A9d9567837490";

#[tokio::main]
async fn fun() -> Result<()> {
    let provider = Provider::<Ws>::connect(WSS_URL).await?;
    let client = Arc::new(provider);
    let address: Address = WETH_ADDRESS.parse()?;
    let contract = IERC20::new(address, client);

    listen_all_events(&contract).await?;
    listen_specific_events(&contract).await?;

    Ok(())
}

/// Given a contract instance, subscribe to all possible events.
/// This allows to centralize the event handling logic and dispatch
/// proper actions.
///
/// Note that all event bindings have been generated
/// by abigen. Feel free to investigate the abigen expanded code to
/// better understand types and functionalities.
async fn listen_all_events(contract: &IERC20<Provider<Ws>>) -> Result<()> {
    let events = contract.events();
    let mut stream = events.stream().await?;

    while let Some(Ok(evt)) = stream.next().await {
        match evt {
            IERC20Events::ApprovalFilter(f) => println!("{f:?}"),
            IERC20Events::TransferFilter(f) => println!("{f:?}"),
        }
    }

    Ok(())
}

/// Given a contract instance subscribe to a single type of event.
///
/// Note that all event bindings have been generated
/// by abigen. Feel free to investigate the abigen expanded code to
/// better understand types and functionalities.
async fn listen_specific_events(contract: &IERC20<Provider<Ws>>) -> Result<()> {
    let events = contract.event::<ApprovalFilter>();
    let mut stream = events.stream().await?.take(1);

    while let Some(Ok(f)) = stream.next().await {
        println!("ApprovalFilter event: {f:?}");
    }

    Ok(())
}
