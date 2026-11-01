use serde::{Deserialize, Serialize};

/// Color style of a reply or inline keyboard button.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ButtonStyle {
    /// Red button.
    Danger,
    /// Green button.
    Success,
    /// Blue button.
    Primary,
}
