use serde::{Deserialize, Serialize};

/// Represents a community (a group of chats).
///
/// [The official docs](https://core.telegram.org/bots/api#community).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Community {
    /// Unique identifier for this community. This number may have more than 32
    /// significant bits and some programming languages may have
    /// difficulty/silent defects in interpreting it. But it has at
    /// most 52 significant bits, so a signed 64-bit integer or double-precision
    /// float type are safe for storing this identifier.
    pub id: i64,
    /// Name of the community
    pub name: String,
}

impl Community {
    pub fn new(id: i64, name: impl Into<String>) -> Self {
        Self { id, name: name.into() }
    }
}
