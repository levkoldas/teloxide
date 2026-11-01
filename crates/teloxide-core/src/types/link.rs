use serde::{Deserialize, Serialize};

/// Represents an HTTP link.
///
/// [The official docs](https://core.telegram.org/bots/api#link).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Link {
    /// URL of the link
    pub url: String,
}

impl Link {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}
