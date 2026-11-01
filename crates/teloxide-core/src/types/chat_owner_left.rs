use crate::types::User;
use serde::{Deserialize, Serialize};

/// Describes a service message about the chat owner leaving the chat.
///
/// [The official docs](https://core.telegram.org/bots/api#chatownerleft).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ChatOwnerLeft {
    /// Optional. The user who will become the new owner of the chat if the
    /// previous owner does not return to the chat
    pub new_owner: Option<User>,
}

impl ChatOwnerLeft {
    pub fn new() -> Self {
        Self { new_owner: None }
    }
    pub fn new_owner(mut self, value: User) -> Self {
        self.new_owner = Some(value);
        self
    }
}
impl Default for ChatOwnerLeft {
    fn default() -> Self {
        Self::new()
    }
}
