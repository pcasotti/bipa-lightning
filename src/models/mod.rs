use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubKey(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alias(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sats(u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bitcoin(u64);

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
pub struct MempoolResponse(Vec<MempoolNode>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiNode {
    public_key: PubKey,
    alias: Alias,
    capacity: Bitcoin,
    #[serde(serialize_with = "datetime_to_iso")]
    first_seen: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse(Vec<ApiNode>);

fn datetime_to_iso<S: Serializer>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&date.to_rfc3339())
}
