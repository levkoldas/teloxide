use serde::{Deserialize, Serialize};

/// Describes a keyboard button to be used by a user of a Mini App.
///
/// [The official docs](https://core.telegram.org/bots/api#preparedkeyboardbutton).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PreparedKeyboardButton {
    /// Unique identifier of the keyboard button
    pub id: String,
}

impl PreparedKeyboardButton {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}
