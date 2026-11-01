use serde::{Deserialize, Serialize};

use crate::types::{ChecklistTaskId, MessageEntity, MessageId, Recipient};

/// Describes reply parameters for the message that is being sent.
#[serde_with::skip_serializing_none]
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ReplyParameters {
    /// Optional. Identifier of the incoming ephemeral message that will be
    /// replied to in the current chat. A reply to an ephemeral message must
    /// itself be an ephemeral message. An ephemeral message may only be
    /// replied to within 15 seconds of being sent. Required if message_id
    /// isn't specified.
    pub ephemeral_message_id: Option<i64>,

    /// Optional. Persistent identifier of the specific poll option to be
    /// replied to
    pub poll_option_id: Option<String>,

    /// Identifier of the message that will be replied to in the current chat,
    /// or in the chat _chat\_id_ if it is specified
    // Issue https://github.com/teloxide/teloxide/issues/1135
    #[serde(default, with = "crate::types::option_msg_id_as_int")]
    #[cfg_attr(test, schemars(with = "Option<i32>"))]
    pub message_id: Option<MessageId>,
    /// If the message to be replied to is from a different chat, unique
    /// identifier for the chat or username of the channel (in the format
    /// `@channelusername`). Not supported for messages sent on behalf of a
    /// business account.
    pub chat_id: Option<Recipient>,
    /// Pass _true_ if the message should be sent even if the specified message
    /// to be replied to is not found; can be used only for replies in the
    /// same chat and forum topic. Always `false` for replies in another chat or
    /// forum topic. Always `true` for messages sent on behalf of a business
    /// account.
    pub allow_sending_without_reply: Option<bool>,
    /// Quoted part of the message to be replied to; 0-1024 characters after
    /// entities parsing. The quote must be an exact substring of the message to
    /// be replied to, including _bold_, _italic_, _underline_, _strikethrough_,
    /// _spoiler_, and _custom_emoji_ entities. The message will fail to send if
    /// the quote isn't found in the original message.
    pub quote: Option<String>,
    /// Mode for parsing entities in the quote. See [formatting options] for
    /// more details.
    ///
    /// [formatting options]: https://core.telegram.org/bots/api#formatting-options
    pub quote_parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the quote. It
    /// can be specified instead of quote_parse_mode.
    pub quote_entities: Option<Vec<MessageEntity>>,
    /// Position of the quote in the original message in UTF-16 code units
    pub quote_position: Option<u32>,
    /// Identifier of the specific checklist task to be replied to
    pub checklist_task_id: Option<ChecklistTaskId>,
}

impl ReplyParameters {
    pub fn new(message_id: MessageId) -> Self {
        Self {
            ephemeral_message_id: None,
            poll_option_id: None,
            message_id: Some(message_id),
            ..Self::default()
        }
    }

    /// Setter for the `chat_id` field
    pub fn chat_id(mut self, chat_id: Recipient) -> Self {
        self.chat_id = Some(chat_id);
        self
    }

    /// Sets the `allow_sending_without_reply_field` to _true_
    pub fn allow_sending_without_reply(mut self) -> Self {
        self.allow_sending_without_reply = Some(true);
        self
    }

    /// Setter for the `quote` field
    pub fn quote(mut self, quote: String) -> Self {
        self.quote = Some(quote);
        self
    }

    /// Setter for the `checklist_task_id` field
    pub fn checklist_task_id(mut self, checklist_task_id: ChecklistTaskId) -> Self {
        self.checklist_task_id = Some(checklist_task_id);
        self
    }
}

impl ReplyParameters {
    /// Sets `ephemeral_message_id`.
    pub fn ephemeral_message_id(mut self, value: i64) -> Self {
        self.ephemeral_message_id = Some(value);
        self
    }
    /// Sets `poll_option_id`.
    pub fn poll_option_id(mut self, value: String) -> Self {
        self.poll_option_id = Some(value);
        self
    }
}

impl From<MessageId> for ReplyParameters {
    fn from(message_id: MessageId) -> Self {
        Self::new(message_id)
    }
}
impl From<&crate::types::Message> for ReplyParameters {
    fn from(message: &crate::types::Message) -> Self {
        match message.ephemeral_message_id {
            Some(id) => Self::default().ephemeral_message_id(id),
            None => Self::new(message.id),
        }
    }
}
impl From<crate::types::Message> for ReplyParameters {
    fn from(message: crate::types::Message) -> Self {
        Self::from(&message)
    }
}
