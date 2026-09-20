use std::time::Duration;

use crate::models::MempoolResponse;

mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async {
        update_nodes().await.unwrap();
    });

    loop {}

    Ok(())
}

static UPDATE_INTERVAL_KEY: &str = "UPDATE_INTERVAL_SECS";
static DEFAULT_UPDATE_INTERVAL: Duration = Duration::from_secs(30);
static MEMPOOL_URL_KEY: &str = "MEMPOOL_URL";
static DEFAULT_MEMPOOL_URL: &str =
    "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

async fn update_nodes() -> Result<(), Box<dyn std::error::Error>> {
    let duration = std::env::var(UPDATE_INTERVAL_KEY)
        .inspect_err(|e| eprintln!("{e}: {UPDATE_INTERVAL_KEY}"))
        .ok()
        .and_then(|s| {
            s.parse::<u64>()
                .inspect_err(|e| eprintln!("{e}: {UPDATE_INTERVAL_KEY}: {s}"))
                .ok()
        })
        .map(Duration::from_secs)
        .unwrap_or(DEFAULT_UPDATE_INTERVAL);

    // TODO: set min interval
    let url = std::env::var(MEMPOOL_URL_KEY)
        .inspect_err(|e| eprintln!("{e}: {MEMPOOL_URL_KEY}"))
        .unwrap_or(DEFAULT_MEMPOOL_URL.to_owned());

    loop {
        let resp = reqwest::get(&url).await?.json::<MempoolResponse>().await?;

        println!("{resp:#?}");

        tokio::time::sleep(duration).await;
    }
}
