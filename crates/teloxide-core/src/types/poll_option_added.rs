use crate::types::{MaybeInaccessibleMessage, MessageEntity};
use serde::{Deserialize, Serialize};

/// Describes a service message about an option added to a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#polloptionadded).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PollOptionAdded {
    /// Optional. Message containing the poll to which the option was added, if
    /// known. Note that the Message object in this field will not contain
    /// the reply_to_message field even if it itself is a reply.
    pub poll_message: Option<MaybeInaccessibleMessage>,
    /// Unique identifier of the added option
    pub option_persistent_id: String,
    /// Option text
    pub option_text: String,
    /// Optional. Special entities that appear in the option_text
    pub option_text_entities: Option<Vec<MessageEntity>>,
}

impl PollOptionAdded {
    pub fn new(option_persistent_id: impl Into<String>, option_text: impl Into<String>) -> Self {
        Self {
            poll_message: None,
            option_persistent_id: option_persistent_id.into(),
            option_text: option_text.into(),
            option_text_entities: None,
        }
    }
    pub fn poll_message(mut self, value: MaybeInaccessibleMessage) -> Self {
        self.poll_message = Some(value);
        self
    }
    pub fn option_text_entities(mut self, value: Vec<MessageEntity>) -> Self {
        self.option_text_entities = Some(value);
        self
    }
}

impl PollOptionAdded {
    /// Returns users in the original poll message and the option's text
    /// entities.
    pub fn mentioned_users(&self) -> impl Iterator<Item = &crate::types::User> {
        self.poll_message
            .as_ref()
            .and_then(crate::types::MaybeInaccessibleMessage::regular_message)
            .into_iter()
            .flat_map(crate::types::Message::mentioned_users_rec)
            .chain(crate::util::mentioned_users_from_entities(
                self.option_text_entities.as_deref().unwrap_or_default(),
            ))
    }
}
