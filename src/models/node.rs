use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{
    mempool::{MempoolNode, MempoolResponse},
    types::{Alias, PubKey, Sats, sats_to_bitcoin},
};

/// A lightning node stored in the database.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;

    const PUBLIC_KEY: &str = "03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f";

    fn sample_node() -> Node {
        Node {
            public_key: PubKey(PUBLIC_KEY.to_owned()),
            alias: Alias("ACINQ".to_owned()),
            capacity: Sats(36_010_516_297),
            first_seen: DateTime::parse_from_rfc3339("2018-04-05T15:13:42Z")
                .unwrap()
                .with_timezone(&Utc),
        }
    }

    #[test]
    fn node_serializes_in_the_challenge_format() {
        let json = serde_json::to_string(&sample_node()).unwrap();

        assert_eq!(
            json,
            format!(
                r#"{{"public_key":"{PUBLIC_KEY}","alias":"ACINQ","capacity":"360.10516297","first_seen":"2018-04-05T15:13:42Z"}}"#
            )
        );
    }

    #[test]
    fn small_capacity_keeps_eight_decimals() {
        let node = Node {
            capacity: Sats(550_000),
            ..sample_node()
        };

        let json = serde_json::to_value(&node).unwrap();

        assert_eq!(json["capacity"], "0.00550000");
    }

    #[test]
    fn mempool_node_maps_to_node() {
        let mempool = MempoolNode {
            public_key: PubKey(PUBLIC_KEY.to_owned()),
            alias: Alias("ACINQ".to_owned()),
            channels: 2908,
            capacity: Sats(36_010_516_297),
            first_seen: sample_node().first_seen,
            updated_at: sample_node().first_seen,
            city: None,
            country: None,
        };

        let node = Node::from(mempool);

        assert_eq!(node, sample_node());
    }
}
