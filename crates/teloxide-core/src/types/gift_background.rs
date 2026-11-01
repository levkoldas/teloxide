use crate::types::Rgb;
use serde::{Deserialize, Serialize};

/// This object describes the background of a gift.
///
/// [The official docs](https://core.telegram.org/bots/api#giftbackground).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct GiftBackground {
    /// Center color of the background in RGB format
    pub center_color: Rgb,
    /// Edge color of the background in RGB format
    pub edge_color: Rgb,
    /// Text color of the background in RGB format
    pub text_color: Rgb,
}

impl GiftBackground {
    pub fn new(center_color: Rgb, edge_color: Rgb, text_color: Rgb) -> Self {
        Self { center_color, edge_color, text_color }
    }
}
