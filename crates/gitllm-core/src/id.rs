use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

macro_rules! define_id {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            /// Generates a new random (v4) identifier.
            pub fn new_v4() -> Self {
                Self(Uuid::new_v4())
            }

            /// Generates a new time-ordered (v7) identifier.
            pub fn new_v7() -> Self {
                Self(Uuid::now_v7())
            }

            /// Wraps an existing Uuid
            pub fn from_uuid(uuid: Uuid) -> Self {
                Self(uuid)
            }

            /// Returns the inner Uuid
            pub fn as_uuid(&self) -> &Uuid {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl FromStr for $name {
            type Err = uuid::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self(Uuid::parse_str(s)?))
            }
        }
    };
}

define_id!(
    ConversationId,
    "Unique identifier for a Conversation container."
);
define_id!(
    NodeId,
    "Unique identifier for an immutable historical Node."
);
define_id!(
    BranchId,
    "Unique identifier for a lightweight Branch reference."
);
define_id!(RunId, "Unique identifier for a transient execution Run.");
define_id!(
    SnapshotId,
    "Unique identifier for a named Snapshot reference."
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typed_id_generation_and_formatting() {
        let node_id = NodeId::new_v7();
        let node_id_str = node_id.to_string();

        let parsed: NodeId = node_id_str.parse().expect("Failed to parse UUID string");
        assert_eq!(node_id, parsed);
    }

    #[test]
    fn test_serde_roundtrip() {
        let branch_id = BranchId::new_v4();
        let json = serde_json::to_string(&branch_id).expect("Serialization failed");
        let deserialized: BranchId = serde_json::from_str(&json).expect("Deserialization failed");

        assert_eq!(branch_id, deserialized);
    }
}
