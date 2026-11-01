use crate::types::User;
use serde::{Deserialize, Serialize};

/// This object describes the access settings of a bot.
///
/// [The official docs](https://core.telegram.org/bots/api#botaccesssettings).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct BotAccessSettings {
    /// True, if only selected users can access the bot. The bot's owner can
    /// always access it.
    pub is_access_restricted: bool,
    /// Optional. The list of other users who have access to the bot if the
    /// access is restricted
    pub added_users: Option<Vec<User>>,
}

impl BotAccessSettings {
    pub fn new(is_access_restricted: bool) -> Self {
        Self { is_access_restricted, added_users: None }
    }
    pub fn added_users(mut self, value: Vec<User>) -> Self {
        self.added_users = Some(value);
        self
    }
}
