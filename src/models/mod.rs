use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct PubKey(String);

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Alias(String);

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Sats(pub u64);

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Bitcoin(f64);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MempoolNode {
    public_key: PubKey,
    alias: Alias,
    channels: u32,
    capacity: Sats,
    #[serde(with = "chrono::serde::ts_seconds")]
    first_seen: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    updated_at: DateTime<Utc>,
    city: Option<HashMap<String, String>>,
    country: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MempoolResponse(pub Vec<MempoolNode>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiNode {
    pub public_key: PubKey,
    pub alias: Alias,
    #[serde(serialize_with = "sats_to_bitcoin")]
    pub capacity: Sats,
    #[serde(serialize_with = "datetime_to_iso")]
    pub first_seen: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse(pub Vec<ApiNode>);

fn sats_to_bitcoin<S: Serializer>(sats: &Sats, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_f64(Bitcoin::from(*sats).0)
}

fn datetime_to_iso<S: Serializer>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&date.to_rfc3339())
}

impl From<Sats> for Bitcoin {
    fn from(value: Sats) -> Self {
        Self(value.0 as f64 / 100_000_000.0)
    }
}

impl From<MempoolResponse> for ApiResponse {
    fn from(value: MempoolResponse) -> Self {
        Self(value.0.into_iter().map(Into::into).collect())
    }
}

impl From<MempoolNode> for ApiNode {
    fn from(value: MempoolNode) -> Self {
        Self {
            public_key: value.public_key,
            alias: value.alias,
            capacity: value.capacity,
            first_seen: value.first_seen,
        }
    }
}
