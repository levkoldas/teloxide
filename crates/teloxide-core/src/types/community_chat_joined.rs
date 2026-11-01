use crate::types::Community;
use serde::{Deserialize, Serialize};

/// Describes a service message about a chat being joined by a user from a
/// community.
///
/// [The official docs](https://core.telegram.org/bots/api#communitychatjoined).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct CommunityChatJoined {
    /// The community from which the chat was joined
    pub community: Community,
}

impl CommunityChatJoined {
    pub fn new(community: Community) -> Self {
        Self { community }
    }
}
