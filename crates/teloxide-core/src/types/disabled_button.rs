use serde::{Deserialize, Serialize};

/// This object represents a disabled button which does nothing. Currently holds
/// no information.
///
/// [The official docs](https://core.telegram.org/bots/api#disabledbutton).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct DisabledButton {}

impl DisabledButton {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for DisabledButton {
    fn default() -> Self {
        Self::new()
    }
}
