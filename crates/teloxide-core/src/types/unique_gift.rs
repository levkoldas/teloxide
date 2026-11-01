use crate::types::{Chat, Rgb, Sticker};
use serde::{Deserialize, Serialize};

/// This object describes a unique gift that was upgraded from a regular gift.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UniqueGift {
    /// Identifier of the regular gift from which the gift was upgraded
    pub gift_id: crate::types::GiftId,

    /// Optional. True, if the original regular gift was exclusively
    /// purchaseable by Telegram Premium subscribers
    pub is_premium: Option<crate::types::True>,

    /// Optional. True, if the gift was used to craft another gift and isn't
    /// available anymore
    pub is_burned: Option<crate::types::True>,

    /// Optional. True, if the gift is assigned from the TON blockchain and
    /// can't be resold or transferred in Telegram
    pub is_from_blockchain: Option<crate::types::True>,

    /// Optional. The color scheme that can be used by the gift's owner for the
    /// chat's name, replies to messages and link previews; for business
    /// account gifts and gifts that are currently on sale only
    pub colors: Option<crate::types::UniqueGiftColors>,

    /// Human-readable name of the regular gift from which this unique gift was
    /// upgraded
    pub base_name: String,

    /// Unique name of the gift. This name can be used in `https://t.me/nft/...` links and story areas
    pub name: String,

    /// Unique number of the upgraded gift among gifts upgraded from the same
    /// regular gift
    pub number: u64,

    /// Model of the gift
    pub model: UniqueGiftModel,

    /// Symbol of the gift
    pub symbol: UniqueGiftSymbol,

    /// Backdrop of the gift
    pub backdrop: UniqueGiftBackdrop,

    /// Information about the chat that published the gift
    pub publisher_chat: Option<Chat>,
}

/// This object describes the model of a unique gift.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UniqueGiftModel {
    /// Optional. Rarity of the model if it is a crafted model. Currently, can
    /// be “uncommon”, “rare”, “epic”, or “legendary”.
    pub rarity: Option<String>,

    /// Name of the model
    pub name: String,

    /// The sticker that represents the unique gift
    pub sticker: Sticker,

    /// The number of unique gifts that receive this model for every 1000 gifts
    /// upgraded
    pub rarity_per_mille: u32,
}

/// This object describes the symbol shown on the pattern of a unique gift.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UniqueGiftSymbol {
    /// Name of the symbol
    pub name: String,

    /// The sticker that represents the unique gift
    pub sticker: Sticker,

    /// The number of unique gifts that receive this model for every 1000 gifts
    /// upgraded
    pub rarity_per_mille: u32,
}

/// This object describes the backdrop of a unique gift.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UniqueGiftBackdrop {
    /// Name of the backdrop
    pub name: String,

    /// Colors of the backdrop
    pub colors: UniqueGiftBackdropColors,

    /// The number of unique gifts that receive this backdrop for every 1000
    /// gifts upgraded
    pub rarity_per_mille: u32,
}

/// This object describes the colors of the backdrop of a unique gift.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UniqueGiftBackdropColors {
    /// The color in the center of the backdrop in RGB format
    pub center_color: Rgb,

    /// The color on the edges of the backdrop in RGB format
    pub edge_color: Rgb,

    /// The color to be applied to the symbol in RGB format
    pub symbol_color: Rgb,

    /// The color for the text on the backdrop in RGB format
    pub text_color: Rgb,
}

impl UniqueGift {
    /// Sets `gift_id`.
    pub fn gift_id(mut self, value: crate::types::GiftId) -> Self {
        self.gift_id = value;
        self
    }
    /// Sets `is_premium`.
    pub fn is_premium(mut self, value: crate::types::True) -> Self {
        self.is_premium = Some(value);
        self
    }
    /// Sets `is_burned`.
    pub fn is_burned(mut self, value: crate::types::True) -> Self {
        self.is_burned = Some(value);
        self
    }
    /// Sets `is_from_blockchain`.
    pub fn is_from_blockchain(mut self, value: crate::types::True) -> Self {
        self.is_from_blockchain = Some(value);
        self
    }
    /// Sets `colors`.
    pub fn colors(mut self, value: crate::types::UniqueGiftColors) -> Self {
        self.colors = Some(value);
        self
    }
}

impl UniqueGiftModel {
    /// Sets `rarity`.
    pub fn rarity(mut self, value: String) -> Self {
        self.rarity = Some(value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::types::{FileMeta, StickerFormatFlags, StickerKind};

    #[test]
    fn deserialize() {
        let sticker = Sticker {
            file: FileMeta {
                id: "CAACAgIAAxUAAWMwcTidRlq7bai-xUkcHQLa6vgJAALZBwACwRieC1FFIeQlHsPdKQQ".into(),
                unique_id: "AgAD2QcAAsEYngs".into(),
                size: 25734,
            },
            width: 463,
            height: 512,
            kind: StickerKind::Regular { premium_animation: None },
            flags: StickerFormatFlags { is_animated: false, is_video: false },
            thumbnail: None,
            emoji: None,
            set_name: None,
            needs_repainting: false,
        };

        let unique_gift = UniqueGift {
            gift_id: "gift".into(),
            is_premium: None,
            is_burned: None,
            is_from_blockchain: None,
            colors: None,
            base_name: "name".to_owned(),
            name: "name".to_owned(),
            number: 123,
            model: UniqueGiftModel {
                rarity: None,
                name: "name".to_owned(),
                sticker: sticker.clone(),
                rarity_per_mille: 123,
            },
            symbol: UniqueGiftSymbol {
                name: "name".to_owned(),
                sticker: sticker.clone(),
                rarity_per_mille: 123,
            },
            backdrop: UniqueGiftBackdrop {
                name: "name".to_owned(),
                colors: UniqueGiftBackdropColors {
                    center_color: Rgb { r: 255, g: 255, b: 0 },
                    edge_color: Rgb { r: 255, g: 255, b: 0 },
                    symbol_color: Rgb { r: 255, g: 255, b: 0 },
                    text_color: Rgb { r: 255, g: 255, b: 0 },
                },
                rarity_per_mille: 123,
            },
            publisher_chat: None,
        };

        let unique_gift_json = r#"{
            "gift_id": "gift",
            "base_name": "name",
            "name": "name",
            "number": 123,
            "model": {
                "name": "name",
                "sticker": {
                    "file_id": "CAACAgIAAxUAAWMwcTidRlq7bai-xUkcHQLa6vgJAALZBwACwRieC1FFIeQlHsPdKQQ",
                    "file_unique_id": "AgAD2QcAAsEYngs",
                    "file_size": 25734,
                    "width": 463,
                    "height": 512,
                    "type": "regular",
                    "premium_animation": null,
                    "is_animated": false,
                    "is_video": false,
                    "needs_repainting": false
                },
                "rarity_per_mille": 123
            },
            "symbol": {
                "name": "name",
                "sticker": {
                    "file_id": "CAACAgIAAxUAAWMwcTidRlq7bai-xUkcHQLa6vgJAALZBwACwRieC1FFIeQlHsPdKQQ",
                    "file_unique_id": "AgAD2QcAAsEYngs",
                    "file_size": 25734,
                    "width": 463,
                    "height": 512,
                    "type": "regular",
                    "premium_animation": null,
                    "is_animated": false,
                    "is_video": false,
                    "needs_repainting": false
                },
                "rarity_per_mille": 123
            },
            "backdrop": {
                "name": "name",
                "colors": {
                    "center_color": 16776960,
                    "edge_color": 16776960,
                    "symbol_color": 16776960,
                    "text_color": 16776960
                },
                "rarity_per_mille": 123
            }
        }"#;

        assert_eq!(serde_json::from_str::<UniqueGift>(unique_gift_json).unwrap(), unique_gift);
    }
}
