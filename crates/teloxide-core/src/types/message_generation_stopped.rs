use crate::types::{Chat, ThreadId};
use serde::{Deserialize, Serialize};

/// This object describes an update about a user stopping message generation.
///
/// [The official docs](https://core.telegram.org/bots/api#messagegenerationstopped).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct MessageGenerationStopped {
    /// Chat in which the message is generated
    pub chat: Chat,
    /// Optional. Unique identifier of the message thread in which the message
    /// is generated
    pub message_thread_id: Option<ThreadId>,
    /// Unique identifier of the message draft which was stopped
    pub draft_id: i64,
}

impl MessageGenerationStopped {
    pub fn new(chat: Chat, draft_id: i64) -> Self {
        Self { chat, message_thread_id: None, draft_id }
    }
    pub fn message_thread_id(mut self, value: ThreadId) -> Self {
        self.message_thread_id = Some(value);
        self
    }
}
