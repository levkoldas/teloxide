use crate::types::Community;
use serde::{Deserialize, Serialize};

/// Describes a service message about a chat or a bot being added to a
/// community.
///
/// [The official docs](https://core.telegram.org/bots/api#communitychatadded).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct CommunityChatAdded {
    /// The new community to which the chat or the bot belongs
    pub community: Community,
}

impl CommunityChatAdded {
    pub fn new(community: Community) -> Self {
        Self { community }
    }
}
