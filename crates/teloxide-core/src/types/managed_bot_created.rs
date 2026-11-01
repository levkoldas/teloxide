use crate::types::User;
use serde::{Deserialize, Serialize};

/// This object contains information about the bot that was created to be
/// managed by the current bot.
///
/// [The official docs](https://core.telegram.org/bots/api#managedbotcreated).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ManagedBotCreated {
    /// Information about the bot. The bot's token can be fetched using the
    /// method getManagedBotToken.
    pub bot: User,
}

impl ManagedBotCreated {
    pub fn new(bot: User) -> Self {
        Self { bot }
    }
}
