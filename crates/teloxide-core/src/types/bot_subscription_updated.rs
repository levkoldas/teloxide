use crate::types::User;
use serde::{Deserialize, Serialize};

/// This object contains information about changes to a user payment
/// subscription toward the current bot.
///
/// [The official docs](https://core.telegram.org/bots/api#botsubscriptionupdated).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct BotSubscriptionUpdated {
    /// User who subscribed for payments toward the bot
    pub user: User,
    /// Bot-specified invoice payload
    pub invoice_payload: String,
    /// The new state of the subscription. Currently, it can be one of
    /// “canceled” if the user canceled the subscription, “active” if the
    /// user re-enabled a previously canceled subscription, or “failed” if
    /// payment for the subscription failed.
    pub state: String,
}

impl BotSubscriptionUpdated {
    pub fn new(user: User, invoice_payload: impl Into<String>, state: impl Into<String>) -> Self {
        Self { user, invoice_payload: invoice_payload.into(), state: state.into() }
    }
}
