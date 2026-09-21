use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::types::{Alias, PubKey, Sats};

/// A node as returned by the mempool API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MempoolNode {
    pub public_key: PubKey,
    pub alias: Alias,
    pub channels: u32,
    pub capacity: Sats,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub first_seen: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
    pub city: Option<HashMap<String, String>>,
    pub country: Option<HashMap<String, String>>,
}

/// The response of the mempool node rankings endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MempoolResponse(pub Vec<MempoolNode>);
