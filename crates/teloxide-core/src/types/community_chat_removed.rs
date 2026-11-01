use serde::{Deserialize, Serialize};

/// Describes a service message about a chat or a bot being removed from a
/// community. Currently holds no information.
///
/// [The official docs](https://core.telegram.org/bots/api#communitychatremoved).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct CommunityChatRemoved {}

impl CommunityChatRemoved {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for CommunityChatRemoved {
    fn default() -> Self {
        Self::new()
    }
}
