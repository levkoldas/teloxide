use serde::{Deserialize, Serialize};

use crate::types::{CallbackQueryId, UserId};

/// Parameters for a message visible only to its recipient and the bot.
///
/// [The official docs](https://core.telegram.org/bots/api#ephemeralmessageparameters).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct EphemeralMessageParameters {
    /// User who will be able to see the message.
    pub receiver_user_id: UserId,

    /// Callback query that the message answers, if any.
    pub callback_query_id: Option<CallbackQueryId>,

    /// Replace the callback query's message with this ephemeral message.
    pub replace_callback_query_message: Option<bool>,
}

impl EphemeralMessageParameters {
    pub const fn new(receiver_user_id: UserId) -> Self {
        Self { receiver_user_id, callback_query_id: None, replace_callback_query_message: None }
    }

    pub fn callback_query_id(mut self, callback_query_id: impl Into<CallbackQueryId>) -> Self {
        self.callback_query_id = Some(callback_query_id.into());
        self
    }

    pub const fn replace_callback_query_message(mut self, replace: bool) -> Self {
        self.replace_callback_query_message = Some(replace);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn send_payload_keeps_ephemeral_parameters_nested() {
        use crate::{
            payloads::{SendMessage, SendMessageSetters},
            types::ChatId,
        };
        let payload = SendMessage::new(ChatId(-100123), "Only you can see this")
            .ephemeral_message_parameters(EphemeralMessageParameters::new(UserId(42)));
        assert_eq!(
            serde_json::to_value(payload).unwrap(),
            json!({
                "chat_id": -100123,
                "text": "Only you can see this",
                "ephemeral_message_parameters": {"receiver_user_id": 42}
            })
        );
    }

    #[test]
    fn optional_parameters_are_omitted() {
        assert_eq!(
            serde_json::to_value(EphemeralMessageParameters::new(UserId(42))).unwrap(),
            json!({"receiver_user_id": 42})
        );
    }

    #[test]
    fn callback_replacement_roundtrips() {
        let parameters = EphemeralMessageParameters::new(UserId(42))
            .callback_query_id(CallbackQueryId("callback".into()))
            .replace_callback_query_message(true);
        let wire = json!({
            "receiver_user_id": 42,
            "callback_query_id": "callback",
            "replace_callback_query_message": true
        });
        assert_eq!(serde_json::to_value(&parameters).unwrap(), wire);
        assert_eq!(serde_json::from_value::<EphemeralMessageParameters>(wire).unwrap(), parameters);
    }
}
