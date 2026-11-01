use crate::types::{CustomEmojiId, Rgb};
use serde::{Deserialize, Serialize};

/// This object contains information about the color scheme for a user's name,
/// message replies and link previews based on a unique gift.
///
/// [The official docs](https://core.telegram.org/bots/api#uniquegiftcolors).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UniqueGiftColors {
    /// Custom emoji identifier of the unique gift's model
    pub model_custom_emoji_id: CustomEmojiId,
    /// Custom emoji identifier of the unique gift's symbol
    pub symbol_custom_emoji_id: CustomEmojiId,
    /// Main color used in light themes; RGB format
    pub light_theme_main_color: Rgb,
    /// List of 1-3 additional colors used in light themes; RGB format
    pub light_theme_other_colors: Vec<Rgb>,
    /// Main color used in dark themes; RGB format
    pub dark_theme_main_color: Rgb,
    /// List of 1-3 additional colors used in dark themes; RGB format
    pub dark_theme_other_colors: Vec<Rgb>,
}

impl UniqueGiftColors {
    pub fn new(
        model_custom_emoji_id: CustomEmojiId,
        symbol_custom_emoji_id: CustomEmojiId,
        light_theme_main_color: Rgb,
        light_theme_other_colors: Vec<Rgb>,
        dark_theme_main_color: Rgb,
        dark_theme_other_colors: Vec<Rgb>,
    ) -> Self {
        Self {
            model_custom_emoji_id,
            symbol_custom_emoji_id,
            light_theme_main_color,
            light_theme_other_colors,
            dark_theme_main_color,
            dark_theme_other_colors,
        }
    }
}
