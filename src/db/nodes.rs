use sqlx::PgPool;

use crate::{db::error::Error, models::Node};

/// Inserts or updates the given nodes in the database.
///
/// Existing rows are matched on `public_key` and updated with the latest
/// alias, capacity and first seen timestamp.
///
/// Returns [`Error::Query`] if the query fails.
pub async fn upsert_nodes(pool: &PgPool, nodes: &[Node]) -> Result<(), Error> {
    let keys: Vec<_> = nodes.iter().map(|n| n.public_key.clone()).collect();
    let aliases: Vec<_> = nodes.iter().map(|n| n.alias.clone()).collect();
    let capacities: Vec<_> = nodes.iter().map(|n| n.capacity.0 as i64).collect();
    let dates: Vec<_> = nodes.iter().map(|n| n.first_seen).collect();

    let result = sqlx::query(
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

    tracing::debug!(
        input = nodes.len(),
        rows = result.rows_affected(),
        "nodes upserted"
    );

    Ok(())
}

/// Fetches all nodes from the database.
///
/// Returns [`Error::Query`] if the query fails.
pub async fn fetch_all(pool: &PgPool) -> Result<Vec<Node>, Error> {
    let nodes = sqlx::query_as::<_, Node>("SELECT * FROM nodes")
        .fetch_all(pool)
        .await?;

    tracing::debug!(count = nodes.len(), "nodes fetched");

    Ok(nodes)
}
