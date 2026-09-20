use std::time::Duration;

use sqlx::PgPool;
use tokio::task::JoinHandle;

use crate::{
    env,
    models::{ApiNode, ApiResponse, MempoolResponse},
};

static UPDATE_INTERVAL_KEY: &str = "UPDATE_INTERVAL_SECS";
static UPDATE_INTERVAL_DEFAULT: Duration = Duration::from_secs(30);
static MEMPOOL_URL_KEY: &str = "MEMPOOL_URL";
static MEMPOOL_URL_DEFAULT: &str =
    "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

pub async fn start(pool: &PgPool) -> JoinHandle<()> {
    let pool = pool.clone();
    tokio::spawn(async move {
        update_loop(&pool).await.unwrap();
    })
}

pub async fn update_loop(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: set min interval
    let duration = Duration::from_secs(env::get_from_str_or(
        UPDATE_INTERVAL_KEY,
        UPDATE_INTERVAL_DEFAULT.as_secs(),
    ));

    let url = env::get_or(MEMPOOL_URL_KEY, MEMPOOL_URL_DEFAULT.to_string());

    let client = reqwest::Client::new();

    loop {
        let nodes: ApiResponse = client
            .get(&url)
            .send()
            .await?
            .json::<MempoolResponse>()
            .await?
            .into();

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
