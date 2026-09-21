use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{
    mempool::{MempoolNode, MempoolResponse},
    types::{Alias, PubKey, Sats, sats_to_bitcoin},
};

/// A lightning node stored in the database.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Node {
    pub public_key: PubKey,
    pub alias: Alias,
    #[serde(serialize_with = "sats_to_bitcoin")]
    pub capacity: Sats,
    pub first_seen: DateTime<Utc>,
}

/// The `GET /nodes` response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodesResponse(pub Vec<Node>);

impl From<MempoolNode> for Node {
    fn from(value: MempoolNode) -> Self {
        Self {
            public_key: value.public_key,
            alias: value.alias,
            capacity: value.capacity,
            first_seen: value.first_seen,
        }
    }
}

impl From<MempoolResponse> for NodesResponse {
    fn from(value: MempoolResponse) -> Self {
        Self(value.0.into_iter().map(Into::into).collect())
    }
}
