use serde::{Deserialize, Serialize, Serializer};

/// A lightning node public key.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct PubKey(String);

/// A lightning node alias.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Alias(String);

/// An amount in satoshis.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Sats(pub u64);

/// An amount in bitcoin.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Bitcoin(f64);

impl From<Sats> for Bitcoin {
    fn from(value: Sats) -> Self {
        Self(value.0 as f64 / 100_000_000.0)
    }
}

/// Serializes a [`Sats`] amount as a BTC amount string.
pub fn sats_to_bitcoin<S: Serializer>(sats: &Sats, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&format!("{:.8}", Bitcoin::from(*sats).0))
}

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
