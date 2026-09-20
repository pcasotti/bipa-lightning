use std::time::Duration;

use sqlx::PgPool;

use crate::models::{ApiNode, ApiResponse, MempoolResponse};

static UPDATE_INTERVAL_KEY: &str = "UPDATE_INTERVAL_SECS";
static DEFAULT_UPDATE_INTERVAL: Duration = Duration::from_secs(30);
static MEMPOOL_URL_KEY: &str = "MEMPOOL_URL";
static DEFAULT_MEMPOOL_URL: &str =
    "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

pub async fn update_loop(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
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

    let client = reqwest::Client::new();

    loop {
        let resp = client.get(&url).send().await?.json::<MempoolResponse>().await?;
        let nodes = ApiResponse::from(resp);

        update_nodes(pool, &nodes.0).await.unwrap();

        tokio::time::sleep(duration).await;
    }
}

pub async fn update_nodes(pool: &PgPool, nodes: &[ApiNode]) -> Result<(), sqlx::Error> {
    let keys: Vec<_> = nodes.iter().map(|n| n.public_key.clone()).collect();
    let aliases: Vec<_> = nodes.iter().map(|n| n.alias.clone()).collect();
    let capacities: Vec<_> = nodes.iter().map(|n| n.capacity.0 as i64).collect();
    let dates: Vec<_> = nodes.iter().map(|n| n.first_seen).collect();

    sqlx::query(
        "
        INSERT INTO nodes (public_key, alias, capacity, first_seen)
        SELECT * FROM UNNEST($1::text[], $2::text[], $3::bigint[], $4::timestamptz[])
        ON CONFLICT (public_key) DO UPDATE SET
            alias = EXCLUDED.alias,
            capacity = EXCLUDED.capacity,
            first_seen = EXCLUDED.first_seen
        ",
    )
    .bind(&keys)
    .bind(&aliases)
    .bind(&capacities)
    .bind(&dates)
    .execute(pool)
    .await?;

    Ok(())
}
