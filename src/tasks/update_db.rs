use std::time::Duration;

use sqlx::PgPool;
use tokio::task::JoinHandle;

use crate::{
    db, env,
    models::{MempoolResponse, NodesResponse},
    tasks::error::Error,
};

static UPDATE_INTERVAL_KEY: &str = "UPDATE_INTERVAL_SECS";
static UPDATE_INTERVAL_DEFAULT: Duration = Duration::from_secs(30);
static UPDATE_INTERVAL_MIN: u64 = 1;

static MEMPOOL_URL_KEY: &str = "MEMPOOL_URL";
static MEMPOOL_URL_DEFAULT: &str =
    "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

/// Starts the periodic node update loop on a background task.
///
/// Returns a [`JoinHandle`] for the task.
pub fn start(pool: &PgPool) -> JoinHandle<()> {
    let pool = pool.clone();
    tokio::spawn(async move {
        update_loop(&pool).await;
    })
}

/// Runs the periodic update loop.
///
/// Reads the update interval and mempool URL from environment variables
/// (`UPDATE_INTERVAL_SECS` and `MEMPOOL_URL`), then continuously fetches the
/// latest node rankings from mempool and upserts them into the database. The
/// interval is clamped to a minimum of [`UPDATE_INTERVAL_MIN`] seconds.
///
/// If the loop fails it will retry in the same update interval.
pub async fn update_loop(pool: &PgPool) -> ! {
    let mut interval = env::get_from_str_or(UPDATE_INTERVAL_KEY, UPDATE_INTERVAL_DEFAULT.as_secs());
    if interval < UPDATE_INTERVAL_MIN {
        tracing::warn!(
            "{UPDATE_INTERVAL_KEY} value {interval} below minimum \
             {UPDATE_INTERVAL_MIN}, using minimum"
        );
        interval = UPDATE_INTERVAL_MIN;
    }

    let duration = Duration::from_secs(interval);
    let mut interval = tokio::time::interval(duration);

    let url = env::get_or(MEMPOOL_URL_KEY, MEMPOOL_URL_DEFAULT.to_string());

    let client = reqwest::Client::new();

    tracing::info!(interval_secs = duration.as_secs(), "starting update loop");

    loop {
        interval.tick().await;

        if let Err(e) = fetch_and_upsert(pool, &client, &url).await {
            tracing::error!("update loop iteration failed, will retry: {e}");
        }
    }
}

async fn fetch_and_upsert(pool: &PgPool, client: &reqwest::Client, url: &str) -> Result<(), Error> {
    let nodes: NodesResponse = client
        .get(url)
        .send()
        .await?
        .json::<MempoolResponse>()
        .await?
        .into();

    db::nodes::upsert_nodes(pool, &nodes.0).await?;

    tracing::debug!(count = nodes.0.len(), "mempool nodes fetched and upserted");

    Ok(())
}
