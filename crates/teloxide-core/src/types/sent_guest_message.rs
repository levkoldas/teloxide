use serde::{Deserialize, Serialize};

/// Describes an inline message sent by a guest bot.
///
/// [The official docs](https://core.telegram.org/bots/api#sentguestmessage).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct SentGuestMessage {
    /// Identifier of the sent inline message
    pub inline_message_id: String,
}

impl SentGuestMessage {
    pub fn new(inline_message_id: impl Into<String>) -> Self {
        Self { inline_message_id: inline_message_id.into() }
    }
}
