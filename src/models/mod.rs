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

impl sqlx::Type<sqlx::Postgres> for Sats {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <i64 as sqlx::Type<sqlx::Postgres>>::type_info()
    }

    fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
        <i64 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Sats {
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let raw_i64 = <i64 as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        let sats_u64 = u64::try_from(raw_i64)?;
        Ok(Sats(sats_u64))
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Bitcoin(f64);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MempoolNode {
    pub public_key: PubKey,
    pub alias: Alias,
    pub channels: u32,
    capacity: Sats,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub first_seen: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
    pub city: Option<HashMap<String, String>>,
    pub country: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MempoolResponse(pub Vec<MempoolNode>);

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ApiNode {
    pub public_key: PubKey,
    pub alias: Alias,
    #[serde(serialize_with = "sats_to_bitcoin")]
    pub capacity: Sats,
    pub first_seen: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse(pub Vec<ApiNode>);

fn sats_to_bitcoin<S: Serializer>(sats: &Sats, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&format!("{:.8}", Bitcoin::from(*sats).0))
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
