use crate::types::{
    Animation, Audio, CopyTextButton, CustomEmojiId, DisabledButton, Document, Location, LoginUrl,
    PhotoSize, SwitchInlineQueryChosenChat, True, User, Video, Voice, WebAppInfo,
};
use serde::{Deserialize, Serialize};

/// Rich text accepts plain strings, concatenated pieces, and tagged formatting.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum RichText {
    Plain(String),
    Concat(Vec<RichText>),
    Formatted(Box<RichTextKind>),
}
impl From<String> for RichText {
    fn from(value: String) -> Self {
        Self::Plain(value)
    }
}
impl From<&str> for RichText {
    fn from(value: &str) -> Self {
        Self::Plain(value.into())
    }
}
impl From<Vec<RichText>> for RichText {
    fn from(value: Vec<RichText>) -> Self {
        Self::Concat(value)
    }
}

/// Rich formatted message.
///
/// [The official docs](https://core.telegram.org/bots/api#richmessage).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichMessage {
    /// Content of the message
    pub blocks: Vec<RichBlock>,
    /// Optional. True, if the rich message must be shown right-to-left
    pub is_rtl: Option<bool>,
}

impl RichMessage {
    pub fn new(blocks: Vec<RichBlock>) -> Self {
        Self { blocks, is_rtl: None }
    }
    pub fn is_rtl(mut self, value: bool) -> Self {
        self.is_rtl = Some(value);
        self
    }
}

/// This object represents a button in a RichMessage. Exactly one of the fields
/// other than text and style must be used to specify the type of the button.
///
/// [The official docs](https://core.telegram.org/bots/api#richmessagebutton).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichMessageButton {
    /// Text of the button. May contain only plain text, RichTextCustomEmoji and
    /// RichTextDateTime entities.
    pub text: RichText,
    /// Optional. Style of the button. Must be one of “danger”, “success”,
    /// “primary”, or “link” (the button is shown as a regular link without
    /// borders). Apps may use theme-specific colors for the
    /// button background and text based on the style. The style “link” is
    /// allowed only for callback buttons.
    pub style: Option<String>,
    /// Optional. HTTP or tg:// URL to be opened when the button is pressed.
    /// Links tg://user?id=`<user_id>` can be used to mention a user by
    /// their identifier without using a username, if this is allowed by
    /// their privacy settings.
    pub url: Option<String>,
    /// Optional. Data to be sent in a callback query to the bot when the button
    /// is pressed, 1-64 bytes
    pub callback_data: Option<String>,
    /// Optional. Description of the Web App that will be launched when the user
    /// presses the button. The Web App will be able to send an arbitrary
    /// message on behalf of the user using the method answerWebAppQuery.
    /// Available only in private chats between a user and the bot. Not
    /// supported for messages sent on behalf of a business account.
    pub web_app: Option<WebAppInfo>,
    /// Optional. An HTTPS URL used to automatically authorize the user. Can be
    /// used as a replacement for the Telegram Login Widget. Not supported
    /// for ephemeral messages.
    pub login_url: Option<LoginUrl>,
    /// Optional. If set, pressing the button will prompt the user to select one
    /// of their chats, open that chat and insert the bot's username and the
    /// specified inline query in the input field. May be empty, in which
    /// case just the bot's username will be inserted. Not supported for
    /// messages sent in channel direct messages chats and on behalf of a
    /// business account.
    pub switch_inline_query: Option<String>,
    /// Optional. If set, pressing the button will insert the bot's username and
    /// the specified inline query in the current chat's input field. May be
    /// empty, in which case only the bot's username will be inserted. Not
    /// supported in channels and for messages sent in channel direct messages
    /// chats and on behalf of a business account.
    pub switch_inline_query_current_chat: Option<String>,
    /// Optional. If set, pressing the button will prompt the user to select one
    /// of their chats of the specified type, open that chat and insert the
    /// bot's username and the specified inline query in the input field.
    /// Not supported for messages sent in channel direct messages chats and on
    /// behalf of a business account.
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,
    /// Optional. A button that copies the specified text to the clipboard
    pub copy_text: Option<CopyTextButton>,
    /// Optional. If set, then the button is disabled and does nothing
    pub disabled: Option<DisabledButton>,
}

impl RichMessageButton {
    pub fn new(text: RichText) -> Self {
        Self {
            text,
            style: None,
            url: None,
            callback_data: None,
            web_app: None,
            login_url: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            switch_inline_query_chosen_chat: None,
            copy_text: None,
            disabled: None,
        }
    }
    pub fn style(mut self, value: impl Into<String>) -> Self {
        self.style = Some(value.into());
        self
    }
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }
    pub fn callback_data(mut self, value: impl Into<String>) -> Self {
        self.callback_data = Some(value.into());
        self
    }
    pub fn web_app(mut self, value: WebAppInfo) -> Self {
        self.web_app = Some(value);
        self
    }
    pub fn login_url(mut self, value: LoginUrl) -> Self {
        self.login_url = Some(value);
        self
    }
    pub fn switch_inline_query(mut self, value: impl Into<String>) -> Self {
        self.switch_inline_query = Some(value.into());
        self
    }
    pub fn switch_inline_query_current_chat(mut self, value: impl Into<String>) -> Self {
        self.switch_inline_query_current_chat = Some(value.into());
        self
    }
    pub fn switch_inline_query_chosen_chat(mut self, value: SwitchInlineQueryChosenChat) -> Self {
        self.switch_inline_query_chosen_chat = Some(value);
        self
    }
    pub fn copy_text(mut self, value: CopyTextButton) -> Self {
        self.copy_text = Some(value);
        self
    }
    pub fn disabled(mut self, value: DisabledButton) -> Self {
        self.disabled = Some(value);
        self
    }
}

/// This object represents a rich formatted text. Currently, it can be either a
/// String for plain text, an Array of RichText, or any of the following types:
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type")]
pub enum RichTextKind {
    #[serde(rename = "bold")]
    Bold(Box<RichTextBold>),
    #[serde(rename = "italic")]
    Italic(Box<RichTextItalic>),
    #[serde(rename = "underline")]
    Underline(Box<RichTextUnderline>),
    #[serde(rename = "strikethrough")]
    Strikethrough(Box<RichTextStrikethrough>),
    #[serde(rename = "spoiler")]
    Spoiler(Box<RichTextSpoiler>),
    #[serde(rename = "date_time")]
    DateTime(Box<RichTextDateTime>),
    #[serde(rename = "text_mention")]
    TextMention(Box<RichTextTextMention>),
    #[serde(rename = "subscript")]
    Subscript(Box<RichTextSubscript>),
    #[serde(rename = "superscript")]
    Superscript(Box<RichTextSuperscript>),
    #[serde(rename = "marked")]
    Marked(Box<RichTextMarked>),
    #[serde(rename = "code")]
    Code(Box<RichTextCode>),
    #[serde(rename = "custom_emoji")]
    CustomEmoji(Box<RichTextCustomEmoji>),
    #[serde(rename = "mathematical_expression")]
    MathematicalExpression(Box<RichTextMathematicalExpression>),
    #[serde(rename = "url")]
    Url(Box<RichTextUrl>),
    #[serde(rename = "email_address")]
    EmailAddress(Box<RichTextEmailAddress>),
    #[serde(rename = "phone_number")]
    PhoneNumber(Box<RichTextPhoneNumber>),
    #[serde(rename = "bank_card_number")]
    BankCardNumber(Box<RichTextBankCardNumber>),
    #[serde(rename = "mention")]
    Mention(Box<RichTextMention>),
    #[serde(rename = "hashtag")]
    Hashtag(Box<RichTextHashtag>),
    #[serde(rename = "cashtag")]
    Cashtag(Box<RichTextCashtag>),
    #[serde(rename = "bot_command")]
    BotCommand(Box<RichTextBotCommand>),
    #[serde(rename = "button")]
    Button(Box<RichTextButton>),
    #[serde(rename = "anchor")]
    Anchor(Box<RichTextAnchor>),
    #[serde(rename = "anchor_link")]
    AnchorLink(Box<RichTextAnchorLink>),
    #[serde(rename = "reference")]
    Reference(Box<RichTextReference>),
    #[serde(rename = "reference_link")]
    ReferenceLink(Box<RichTextReferenceLink>),
}
impl From<RichTextBold> for RichTextKind {
    fn from(value: RichTextBold) -> Self {
        Self::Bold(Box::new(value))
    }
}
impl From<RichTextBold> for RichText {
    fn from(value: RichTextBold) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextItalic> for RichTextKind {
    fn from(value: RichTextItalic) -> Self {
        Self::Italic(Box::new(value))
    }
}
impl From<RichTextItalic> for RichText {
    fn from(value: RichTextItalic) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextUnderline> for RichTextKind {
    fn from(value: RichTextUnderline) -> Self {
        Self::Underline(Box::new(value))
    }
}
impl From<RichTextUnderline> for RichText {
    fn from(value: RichTextUnderline) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextStrikethrough> for RichTextKind {
    fn from(value: RichTextStrikethrough) -> Self {
        Self::Strikethrough(Box::new(value))
    }
}
impl From<RichTextStrikethrough> for RichText {
    fn from(value: RichTextStrikethrough) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextSpoiler> for RichTextKind {
    fn from(value: RichTextSpoiler) -> Self {
        Self::Spoiler(Box::new(value))
    }
}
impl From<RichTextSpoiler> for RichText {
    fn from(value: RichTextSpoiler) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextDateTime> for RichTextKind {
    fn from(value: RichTextDateTime) -> Self {
        Self::DateTime(Box::new(value))
    }
}
impl From<RichTextDateTime> for RichText {
    fn from(value: RichTextDateTime) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextTextMention> for RichTextKind {
    fn from(value: RichTextTextMention) -> Self {
        Self::TextMention(Box::new(value))
    }
}
impl From<RichTextTextMention> for RichText {
    fn from(value: RichTextTextMention) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextSubscript> for RichTextKind {
    fn from(value: RichTextSubscript) -> Self {
        Self::Subscript(Box::new(value))
    }
}
impl From<RichTextSubscript> for RichText {
    fn from(value: RichTextSubscript) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextSuperscript> for RichTextKind {
    fn from(value: RichTextSuperscript) -> Self {
        Self::Superscript(Box::new(value))
    }
}
impl From<RichTextSuperscript> for RichText {
    fn from(value: RichTextSuperscript) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextMarked> for RichTextKind {
    fn from(value: RichTextMarked) -> Self {
        Self::Marked(Box::new(value))
    }
}
impl From<RichTextMarked> for RichText {
    fn from(value: RichTextMarked) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextCode> for RichTextKind {
    fn from(value: RichTextCode) -> Self {
        Self::Code(Box::new(value))
    }
}
impl From<RichTextCode> for RichText {
    fn from(value: RichTextCode) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextCustomEmoji> for RichTextKind {
    fn from(value: RichTextCustomEmoji) -> Self {
        Self::CustomEmoji(Box::new(value))
    }
}
impl From<RichTextCustomEmoji> for RichText {
    fn from(value: RichTextCustomEmoji) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextMathematicalExpression> for RichTextKind {
    fn from(value: RichTextMathematicalExpression) -> Self {
        Self::MathematicalExpression(Box::new(value))
    }
}
impl From<RichTextMathematicalExpression> for RichText {
    fn from(value: RichTextMathematicalExpression) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextUrl> for RichTextKind {
    fn from(value: RichTextUrl) -> Self {
        Self::Url(Box::new(value))
    }
}
impl From<RichTextUrl> for RichText {
    fn from(value: RichTextUrl) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextEmailAddress> for RichTextKind {
    fn from(value: RichTextEmailAddress) -> Self {
        Self::EmailAddress(Box::new(value))
    }
}
impl From<RichTextEmailAddress> for RichText {
    fn from(value: RichTextEmailAddress) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextPhoneNumber> for RichTextKind {
    fn from(value: RichTextPhoneNumber) -> Self {
        Self::PhoneNumber(Box::new(value))
    }
}
impl From<RichTextPhoneNumber> for RichText {
    fn from(value: RichTextPhoneNumber) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextBankCardNumber> for RichTextKind {
    fn from(value: RichTextBankCardNumber) -> Self {
        Self::BankCardNumber(Box::new(value))
    }
}
impl From<RichTextBankCardNumber> for RichText {
    fn from(value: RichTextBankCardNumber) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextMention> for RichTextKind {
    fn from(value: RichTextMention) -> Self {
        Self::Mention(Box::new(value))
    }
}
impl From<RichTextMention> for RichText {
    fn from(value: RichTextMention) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextHashtag> for RichTextKind {
    fn from(value: RichTextHashtag) -> Self {
        Self::Hashtag(Box::new(value))
    }
}
impl From<RichTextHashtag> for RichText {
    fn from(value: RichTextHashtag) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextCashtag> for RichTextKind {
    fn from(value: RichTextCashtag) -> Self {
        Self::Cashtag(Box::new(value))
    }
}
impl From<RichTextCashtag> for RichText {
    fn from(value: RichTextCashtag) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextBotCommand> for RichTextKind {
    fn from(value: RichTextBotCommand) -> Self {
        Self::BotCommand(Box::new(value))
    }
}
impl From<RichTextBotCommand> for RichText {
    fn from(value: RichTextBotCommand) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextButton> for RichTextKind {
    fn from(value: RichTextButton) -> Self {
        Self::Button(Box::new(value))
    }
}
impl From<RichTextButton> for RichText {
    fn from(value: RichTextButton) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextAnchor> for RichTextKind {
    fn from(value: RichTextAnchor) -> Self {
        Self::Anchor(Box::new(value))
    }
}
impl From<RichTextAnchor> for RichText {
    fn from(value: RichTextAnchor) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextAnchorLink> for RichTextKind {
    fn from(value: RichTextAnchorLink) -> Self {
        Self::AnchorLink(Box::new(value))
    }
}
impl From<RichTextAnchorLink> for RichText {
    fn from(value: RichTextAnchorLink) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextReference> for RichTextKind {
    fn from(value: RichTextReference) -> Self {
        Self::Reference(Box::new(value))
    }
}
impl From<RichTextReference> for RichText {
    fn from(value: RichTextReference) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}
impl From<RichTextReferenceLink> for RichTextKind {
    fn from(value: RichTextReferenceLink) -> Self {
        Self::ReferenceLink(Box::new(value))
    }
}
impl From<RichTextReferenceLink> for RichText {
    fn from(value: RichTextReferenceLink) -> Self {
        Self::Formatted(Box::new(value.into()))
    }
}

/// A bold text.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextbold).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextBold {
    /// The text
    pub text: RichText,
}

impl RichTextBold {
    pub fn new(text: RichText) -> Self {
        Self { text }
    }
}

/// An italicized text.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextitalic).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextItalic {
    /// The text
    pub text: RichText,
}

impl RichTextItalic {
    pub fn new(text: RichText) -> Self {
        Self { text }
    }
}

/// An underlined text.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextunderline).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextUnderline {
    /// The text
    pub text: RichText,
}

impl RichTextUnderline {
    pub fn new(text: RichText) -> Self {
        Self { text }
    }
}

/// A strikethrough text.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextstrikethrough).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextStrikethrough {
    /// The text
    pub text: RichText,
}

impl RichTextStrikethrough {
    pub fn new(text: RichText) -> Self {
        Self { text }
    }
}

/// A text covered by a spoiler.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextspoiler).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextSpoiler {
    /// The text
    pub text: RichText,
}

impl RichTextSpoiler {
    pub fn new(text: RichText) -> Self {
        Self { text }
    }
}

/// Formatted date and time.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextdatetime).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextDateTime {
    /// The text
    pub text: RichText,
    /// The Unix time associated with the entity
    pub unix_time: i64,
    /// The string that defines the formatting of the date and time. See
    /// date-time entity formatting for more details.
    pub date_time_format: String,
}

impl RichTextDateTime {
    pub fn new(text: RichText, unix_time: i64, date_time_format: impl Into<String>) -> Self {
        Self { text, unix_time, date_time_format: date_time_format.into() }
    }
}

/// A mention of a Telegram user by their identifier.
///
/// [The official docs](https://core.telegram.org/bots/api#richtexttextmention).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextTextMention {
    /// The text
    pub text: RichText,
    /// The mentioned user
    pub user: User,
}

impl RichTextTextMention {
    pub fn new(text: RichText, user: User) -> Self {
        Self { text, user }
    }
}

/// A subscript text.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextsubscript).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextSubscript {
    /// The text
    pub text: RichText,
}

impl RichTextSubscript {
    pub fn new(text: RichText) -> Self {
        Self { text }
    }
}

/// A superscript text.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextsuperscript).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextSuperscript {
    /// The text
    pub text: RichText,
}

impl RichTextSuperscript {
    pub fn new(text: RichText) -> Self {
        Self { text }
    }
}

/// A marked text.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextmarked).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextMarked {
    /// The text
    pub text: RichText,
}

impl RichTextMarked {
    pub fn new(text: RichText) -> Self {
        Self { text }
    }
}

/// A monowidth text.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextcode).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextCode {
    /// The text
    pub text: RichText,
}

impl RichTextCode {
    pub fn new(text: RichText) -> Self {
        Self { text }
    }
}

/// A custom emoji.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextcustomemoji).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextCustomEmoji {
    /// Unique identifier of the custom emoji. Use getCustomEmojiStickers to get
    /// full information about the sticker.
    pub custom_emoji_id: CustomEmojiId,
    /// Alternative emoji for the custom emoji
    pub alternative_text: String,
}

impl RichTextCustomEmoji {
    pub fn new(custom_emoji_id: CustomEmojiId, alternative_text: impl Into<String>) -> Self {
        Self { custom_emoji_id, alternative_text: alternative_text.into() }
    }
}

/// A mathematical expression.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextmathematicalexpression).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextMathematicalExpression {
    /// The expression in LaTeX format
    pub expression: String,
}

impl RichTextMathematicalExpression {
    pub fn new(expression: impl Into<String>) -> Self {
        Self { expression: expression.into() }
    }
}

/// A text with a link.
///
/// [The official docs](https://core.telegram.org/bots/api#richtexturl).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextUrl {
    /// The text
    pub text: RichText,
    /// URL of the link
    pub url: String,
}

impl RichTextUrl {
    pub fn new(text: RichText, url: impl Into<String>) -> Self {
        Self { text, url: url.into() }
    }
}

/// A text with an email address.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextemailaddress).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextEmailAddress {
    /// The text
    pub text: RichText,
    /// The email address
    pub email_address: String,
}

impl RichTextEmailAddress {
    pub fn new(text: RichText, email_address: impl Into<String>) -> Self {
        Self { text, email_address: email_address.into() }
    }
}

/// A text with a phone number.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextphonenumber).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextPhoneNumber {
    /// The text
    pub text: RichText,
    /// The phone number
    pub phone_number: String,
}

impl RichTextPhoneNumber {
    pub fn new(text: RichText, phone_number: impl Into<String>) -> Self {
        Self { text, phone_number: phone_number.into() }
    }
}

/// A text with a bank card number.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextbankcardnumber).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextBankCardNumber {
    /// The text
    pub text: RichText,
    /// The bank card number
    pub bank_card_number: String,
}

impl RichTextBankCardNumber {
    pub fn new(text: RichText, bank_card_number: impl Into<String>) -> Self {
        Self { text, bank_card_number: bank_card_number.into() }
    }
}

/// A mention by a username.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextmention).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextMention {
    /// The text
    pub text: RichText,
    /// The username
    pub username: String,
}

impl RichTextMention {
    pub fn new(text: RichText, username: impl Into<String>) -> Self {
        Self { text, username: username.into() }
    }
}

/// A hashtag.
///
/// [The official docs](https://core.telegram.org/bots/api#richtexthashtag).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextHashtag {
    /// The text
    pub text: RichText,
    /// The hashtag
    pub hashtag: String,
}

impl RichTextHashtag {
    pub fn new(text: RichText, hashtag: impl Into<String>) -> Self {
        Self { text, hashtag: hashtag.into() }
    }
}

/// A cashtag.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextcashtag).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextCashtag {
    /// The text
    pub text: RichText,
    /// The cashtag
    pub cashtag: String,
}

impl RichTextCashtag {
    pub fn new(text: RichText, cashtag: impl Into<String>) -> Self {
        Self { text, cashtag: cashtag.into() }
    }
}

/// A bot command.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextbotcommand).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextBotCommand {
    /// The text
    pub text: RichText,
    /// The bot command
    pub bot_command: String,
}

impl RichTextBotCommand {
    pub fn new(text: RichText, bot_command: impl Into<String>) -> Self {
        Self { text, bot_command: bot_command.into() }
    }
}

/// A button.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextbutton).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextButton {
    /// The button
    pub button: RichMessageButton,
}

impl RichTextButton {
    pub fn new(button: RichMessageButton) -> Self {
        Self { button }
    }
}

/// An anchor.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextanchor).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextAnchor {
    /// The name of the anchor
    pub name: String,
}

impl RichTextAnchor {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// A link to an anchor.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextanchorlink).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextAnchorLink {
    /// The link text
    pub text: RichText,
    /// The name of the anchor. If the name is empty, then the link brings back
    /// to the top of the message.
    pub anchor_name: String,
}

impl RichTextAnchorLink {
    pub fn new(text: RichText, anchor_name: impl Into<String>) -> Self {
        Self { text, anchor_name: anchor_name.into() }
    }
}

/// A reference.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextreference).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextReference {
    /// Text of the reference
    pub text: RichText,
    /// The name of the reference
    pub name: String,
}

impl RichTextReference {
    pub fn new(text: RichText, name: impl Into<String>) -> Self {
        Self { text, name: name.into() }
    }
}

/// A link to a reference.
///
/// [The official docs](https://core.telegram.org/bots/api#richtextreferencelink).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichTextReferenceLink {
    /// The link text
    pub text: RichText,
    /// The name of the reference
    pub reference_name: String,
}

impl RichTextReferenceLink {
    pub fn new(text: RichText, reference_name: impl Into<String>) -> Self {
        Self { text, reference_name: reference_name.into() }
    }
}

/// Caption of a rich formatted block.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockcaption).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockCaption {
    /// Block caption
    pub text: RichText,
    /// Optional. Block credit which corresponds to the HTML tag `<cite>`
    pub credit: Option<RichText>,
}

impl RichBlockCaption {
    pub fn new(text: RichText) -> Self {
        Self { text, credit: None }
    }
    pub fn credit(mut self, value: RichText) -> Self {
        self.credit = Some(value);
        self
    }
}

/// Cell in a table.
///
/// [The official docs](https://core.telegram.org/bots/api#richblocktablecell).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockTableCell {
    /// Optional. Text in the cell. If omitted, then the cell is invisible.
    pub text: Option<RichText>,
    /// Optional. True, if the cell is a header cell
    pub is_header: Option<True>,
    /// Optional. The number of columns the cell spans if it is bigger than 1
    pub colspan: Option<u32>,
    /// Optional. The number of rows the cell spans if it is bigger than 1
    pub rowspan: Option<u32>,
    /// Horizontal cell content alignment. Currently, must be one of “left”,
    /// “center”, or “right”.
    pub align: String,
    /// Vertical cell content alignment. Currently, must be one of “top”,
    /// “middle”, or “bottom”.
    pub valign: String,
}

impl RichBlockTableCell {
    pub fn new(align: impl Into<String>, valign: impl Into<String>) -> Self {
        Self {
            text: None,
            is_header: None,
            colspan: None,
            rowspan: None,
            align: align.into(),
            valign: valign.into(),
        }
    }
    pub fn text(mut self, value: RichText) -> Self {
        self.text = Some(value);
        self
    }
    pub fn is_header(mut self, value: True) -> Self {
        self.is_header = Some(value);
        self
    }
    pub fn colspan(mut self, value: u32) -> Self {
        self.colspan = Some(value);
        self
    }
    pub fn rowspan(mut self, value: u32) -> Self {
        self.rowspan = Some(value);
        self
    }
}

/// An item of a list.
///
/// [The official docs](https://core.telegram.org/bots/api#richblocklistitem).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockListItem {
    /// Label of the item
    pub label: String,
    /// The content of the item
    pub blocks: Vec<RichBlock>,
    /// Optional. True, if the item has a checkbox
    pub has_checkbox: Option<True>,
    /// Optional. True, if the item has a checked checkbox
    pub is_checked: Option<True>,
    /// Optional. For ordered lists, the numeric value of the item label
    pub value: Option<i64>,
    /// Optional. For ordered lists, the type of the item label; must be one of
    /// “a” for lowercase letters, “A” for uppercase letters, “i” for
    /// lowercase Roman numerals, “I” for uppercase Roman numerals, or “1”
    /// for decimal numbers
    #[serde(rename = "type")]
    pub kind: Option<String>,
}

impl RichBlockListItem {
    pub fn new(label: impl Into<String>, blocks: Vec<RichBlock>) -> Self {
        Self {
            label: label.into(),
            blocks,
            has_checkbox: None,
            is_checked: None,
            value: None,
            kind: None,
        }
    }
    pub fn has_checkbox(mut self, value: True) -> Self {
        self.has_checkbox = Some(value);
        self
    }
    pub fn is_checked(mut self, value: True) -> Self {
        self.is_checked = Some(value);
        self
    }
    pub fn value(mut self, value: i64) -> Self {
        self.value = Some(value);
        self
    }
    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }
}

/// This object represents a block in a rich formatted message. Currently, it
/// can be any of the following types:
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type")]
pub enum RichBlock {
    #[serde(rename = "paragraph")]
    Paragraph(Box<RichBlockParagraph>),
    #[serde(rename = "heading")]
    SectionHeading(Box<RichBlockSectionHeading>),
    #[serde(rename = "pre")]
    Preformatted(Box<RichBlockPreformatted>),
    #[serde(rename = "footer")]
    Footer(Box<RichBlockFooter>),
    #[serde(rename = "divider")]
    Divider(Box<RichBlockDivider>),
    #[serde(rename = "mathematical_expression")]
    MathematicalExpression(Box<RichBlockMathematicalExpression>),
    #[serde(rename = "anchor")]
    Anchor(Box<RichBlockAnchor>),
    #[serde(rename = "list")]
    List(Box<RichBlockList>),
    #[serde(rename = "blockquote")]
    BlockQuotation(Box<RichBlockBlockQuotation>),
    #[serde(rename = "expandable_blockquote")]
    ExpandableBlockQuotation(Box<RichBlockExpandableBlockQuotation>),
    #[serde(rename = "pullquote")]
    PullQuotation(Box<RichBlockPullQuotation>),
    #[serde(rename = "collage")]
    Collage(Box<RichBlockCollage>),
    #[serde(rename = "slideshow")]
    Slideshow(Box<RichBlockSlideshow>),
    #[serde(rename = "table")]
    Table(Box<RichBlockTable>),
    #[serde(rename = "details")]
    Details(Box<RichBlockDetails>),
    #[serde(rename = "map")]
    Map(Box<RichBlockMap>),
    #[serde(rename = "buttons")]
    Buttons(Box<RichBlockButtons>),
    #[serde(rename = "animation")]
    Animation(Box<RichBlockAnimation>),
    #[serde(rename = "audio")]
    Audio(Box<RichBlockAudio>),
    #[serde(rename = "document")]
    Document(Box<RichBlockDocument>),
    #[serde(rename = "photo")]
    Photo(Box<RichBlockPhoto>),
    #[serde(rename = "video")]
    Video(Box<RichBlockVideo>),
    #[serde(rename = "voice_note")]
    VoiceNote(Box<RichBlockVoiceNote>),
    #[serde(rename = "thinking")]
    Thinking(Box<RichBlockThinking>),
}
impl From<RichBlockParagraph> for RichBlock {
    fn from(value: RichBlockParagraph) -> Self {
        Self::Paragraph(Box::new(value))
    }
}
impl From<RichBlockSectionHeading> for RichBlock {
    fn from(value: RichBlockSectionHeading) -> Self {
        Self::SectionHeading(Box::new(value))
    }
}
impl From<RichBlockPreformatted> for RichBlock {
    fn from(value: RichBlockPreformatted) -> Self {
        Self::Preformatted(Box::new(value))
    }
}
impl From<RichBlockFooter> for RichBlock {
    fn from(value: RichBlockFooter) -> Self {
        Self::Footer(Box::new(value))
    }
}
impl From<RichBlockDivider> for RichBlock {
    fn from(value: RichBlockDivider) -> Self {
        Self::Divider(Box::new(value))
    }
}
impl From<RichBlockMathematicalExpression> for RichBlock {
    fn from(value: RichBlockMathematicalExpression) -> Self {
        Self::MathematicalExpression(Box::new(value))
    }
}
impl From<RichBlockAnchor> for RichBlock {
    fn from(value: RichBlockAnchor) -> Self {
        Self::Anchor(Box::new(value))
    }
}
impl From<RichBlockList> for RichBlock {
    fn from(value: RichBlockList) -> Self {
        Self::List(Box::new(value))
    }
}
impl From<RichBlockBlockQuotation> for RichBlock {
    fn from(value: RichBlockBlockQuotation) -> Self {
        Self::BlockQuotation(Box::new(value))
    }
}
impl From<RichBlockExpandableBlockQuotation> for RichBlock {
    fn from(value: RichBlockExpandableBlockQuotation) -> Self {
        Self::ExpandableBlockQuotation(Box::new(value))
    }
}
impl From<RichBlockPullQuotation> for RichBlock {
    fn from(value: RichBlockPullQuotation) -> Self {
        Self::PullQuotation(Box::new(value))
    }
}
impl From<RichBlockCollage> for RichBlock {
    fn from(value: RichBlockCollage) -> Self {
        Self::Collage(Box::new(value))
    }
}
impl From<RichBlockSlideshow> for RichBlock {
    fn from(value: RichBlockSlideshow) -> Self {
        Self::Slideshow(Box::new(value))
    }
}
impl From<RichBlockTable> for RichBlock {
    fn from(value: RichBlockTable) -> Self {
        Self::Table(Box::new(value))
    }
}
impl From<RichBlockDetails> for RichBlock {
    fn from(value: RichBlockDetails) -> Self {
        Self::Details(Box::new(value))
    }
}
impl From<RichBlockMap> for RichBlock {
    fn from(value: RichBlockMap) -> Self {
        Self::Map(Box::new(value))
    }
}
impl From<RichBlockButtons> for RichBlock {
    fn from(value: RichBlockButtons) -> Self {
        Self::Buttons(Box::new(value))
    }
}
impl From<RichBlockAnimation> for RichBlock {
    fn from(value: RichBlockAnimation) -> Self {
        Self::Animation(Box::new(value))
    }
}
impl From<RichBlockAudio> for RichBlock {
    fn from(value: RichBlockAudio) -> Self {
        Self::Audio(Box::new(value))
    }
}
impl From<RichBlockDocument> for RichBlock {
    fn from(value: RichBlockDocument) -> Self {
        Self::Document(Box::new(value))
    }
}
impl From<RichBlockPhoto> for RichBlock {
    fn from(value: RichBlockPhoto) -> Self {
        Self::Photo(Box::new(value))
    }
}
impl From<RichBlockVideo> for RichBlock {
    fn from(value: RichBlockVideo) -> Self {
        Self::Video(Box::new(value))
    }
}
impl From<RichBlockVoiceNote> for RichBlock {
    fn from(value: RichBlockVoiceNote) -> Self {
        Self::VoiceNote(Box::new(value))
    }
}
impl From<RichBlockThinking> for RichBlock {
    fn from(value: RichBlockThinking) -> Self {
        Self::Thinking(Box::new(value))
    }
}

/// A text paragraph, corresponding to the HTML tag `<p>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockparagraph).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockParagraph {
    /// Text of the block
    pub text: RichText,
}

impl RichBlockParagraph {
    pub fn new(text: RichText) -> Self {
        Self { text }
    }
}

/// A section heading, corresponding to the HTML tags `<h1>`, `<h2>`, `<h3>`,
/// `<h4>`, `<h5>`, or `<h6>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblocksectionheading).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockSectionHeading {
    /// Text of the block
    pub text: RichText,
    /// Relative size of the text font; 1-6, 1 is the largest, 6 is the smallest
    pub size: u32,
}

impl RichBlockSectionHeading {
    pub fn new(text: RichText, size: u32) -> Self {
        Self { text, size }
    }
}

/// A preformatted text block, corresponding to the nested HTML tags `<pre>` and
/// `<code>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockpreformatted).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockPreformatted {
    /// Text of the block
    pub text: RichText,
    /// Optional. The programming language of the text
    pub language: Option<String>,
}

impl RichBlockPreformatted {
    pub fn new(text: RichText) -> Self {
        Self { text, language: None }
    }
    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }
}

/// A footer, corresponding to the HTML tag `<footer>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockfooter).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockFooter {
    /// Text of the block
    pub text: RichText,
}

impl RichBlockFooter {
    pub fn new(text: RichText) -> Self {
        Self { text }
    }
}

/// A divider, corresponding to the HTML tag `<hr/>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockdivider).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockDivider {}

impl RichBlockDivider {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for RichBlockDivider {
    fn default() -> Self {
        Self::new()
    }
}

/// A block with a mathematical expression in LaTeX format, corresponding to the
/// custom HTML tag `<tg-math-block>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockmathematicalexpression).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockMathematicalExpression {
    /// The mathematical expression in LaTeX format
    pub expression: String,
}

impl RichBlockMathematicalExpression {
    pub fn new(expression: impl Into<String>) -> Self {
        Self { expression: expression.into() }
    }
}

/// A block with an anchor, corresponding to the HTML tag `<a>` with the
/// attribute name.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockanchor).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockAnchor {
    /// The name of the anchor
    pub name: String,
}

impl RichBlockAnchor {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// A list of blocks, corresponding to the HTML tag `<ul>` or `<ol>` with
/// multiple nested tags `<li>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblocklist).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockList {
    /// Items of the list
    pub items: Vec<RichBlockListItem>,
}

impl RichBlockList {
    pub fn new(items: Vec<RichBlockListItem>) -> Self {
        Self { items }
    }
}

/// A block quotation, corresponding to the HTML tag `<blockquote>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockblockquotation).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockBlockQuotation {
    /// Content of the block
    pub blocks: Vec<RichBlock>,
    /// Optional. Credit of the block
    pub credit: Option<RichText>,
}

impl RichBlockBlockQuotation {
    pub fn new(blocks: Vec<RichBlock>) -> Self {
        Self { blocks, credit: None }
    }
    pub fn credit(mut self, value: RichText) -> Self {
        self.credit = Some(value);
        self
    }
}

/// A block quotation, corresponding to the HTML tag `<blockquote>` with custom
/// attribute "expandable".
///
/// [The official docs](https://core.telegram.org/bots/api#richblockexpandableblockquotation).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockExpandableBlockQuotation {
    /// Content of the block
    pub text: RichText,
    /// Optional. Credit of the block
    pub credit: Option<RichText>,
}

impl RichBlockExpandableBlockQuotation {
    pub fn new(text: RichText) -> Self {
        Self { text, credit: None }
    }
    pub fn credit(mut self, value: RichText) -> Self {
        self.credit = Some(value);
        self
    }
}

/// A quotation with centered text, loosely corresponding to the HTML tag
/// `<aside>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockpullquotation).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockPullQuotation {
    /// Text of the block
    pub text: RichText,
    /// Optional. Credit of the block
    pub credit: Option<RichText>,
}

impl RichBlockPullQuotation {
    pub fn new(text: RichText) -> Self {
        Self { text, credit: None }
    }
    pub fn credit(mut self, value: RichText) -> Self {
        self.credit = Some(value);
        self
    }
}

/// A collage, corresponding to the custom HTML tag `<tg-collage>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockcollage).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockCollage {
    /// Elements of the collage
    pub blocks: Vec<RichBlock>,
    /// Optional. Caption of the block
    pub caption: Option<RichBlockCaption>,
}

impl RichBlockCollage {
    pub fn new(blocks: Vec<RichBlock>) -> Self {
        Self { blocks, caption: None }
    }
    pub fn caption(mut self, value: RichBlockCaption) -> Self {
        self.caption = Some(value);
        self
    }
}

/// A slideshow, corresponding to the custom HTML tag `<tg-slideshow>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockslideshow).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockSlideshow {
    /// Elements of the slideshow
    pub blocks: Vec<RichBlock>,
    /// Optional. Caption of the block
    pub caption: Option<RichBlockCaption>,
}

impl RichBlockSlideshow {
    pub fn new(blocks: Vec<RichBlock>) -> Self {
        Self { blocks, caption: None }
    }
    pub fn caption(mut self, value: RichBlockCaption) -> Self {
        self.caption = Some(value);
        self
    }
}

/// A table, corresponding to the HTML tag `<table>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblocktable).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockTable {
    /// Cells of the table
    pub cells: Vec<Vec<RichBlockTableCell>>,
    /// Optional. True, if the table has borders
    pub is_bordered: Option<True>,
    /// Optional. True, if the table is striped
    pub is_striped: Option<True>,
    /// Optional. True, if table cells have smaller indents
    pub is_compact: Option<True>,
    /// Optional. Caption of the table
    pub caption: Option<RichText>,
}

impl RichBlockTable {
    pub fn new(cells: Vec<Vec<RichBlockTableCell>>) -> Self {
        Self { cells, is_bordered: None, is_striped: None, is_compact: None, caption: None }
    }
    pub fn is_bordered(mut self, value: True) -> Self {
        self.is_bordered = Some(value);
        self
    }
    pub fn is_striped(mut self, value: True) -> Self {
        self.is_striped = Some(value);
        self
    }
    pub fn is_compact(mut self, value: True) -> Self {
        self.is_compact = Some(value);
        self
    }
    pub fn caption(mut self, value: RichText) -> Self {
        self.caption = Some(value);
        self
    }
}

/// An expandable block for details disclosure, corresponding to the HTML tag
/// `<details>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockdetails).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockDetails {
    /// Always shown summary of the block
    pub summary: RichText,
    /// Content of the block
    pub blocks: Vec<RichBlock>,
    /// Optional. True, if the content of the block is visible by default
    pub is_open: Option<True>,
}

impl RichBlockDetails {
    pub fn new(summary: RichText, blocks: Vec<RichBlock>) -> Self {
        Self { summary, blocks, is_open: None }
    }
    pub fn is_open(mut self, value: True) -> Self {
        self.is_open = Some(value);
        self
    }
}

/// A block with a map, corresponding to the custom HTML tag `<tg-map>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockmap).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockMap {
    /// Location of the center of the map
    pub location: Location,
    /// Map zoom level
    pub zoom: u32,
    /// Expected width of the map
    pub width: u32,
    /// Expected height of the map
    pub height: u32,
    /// Optional. Caption of the block
    pub caption: Option<RichBlockCaption>,
}

impl RichBlockMap {
    pub fn new(location: Location, zoom: u32, width: u32, height: u32) -> Self {
        Self { location, zoom, width, height, caption: None }
    }
    pub fn caption(mut self, value: RichBlockCaption) -> Self {
        self.caption = Some(value);
        self
    }
}

/// A block containing a list of buttons that are shown in one row,
/// corresponding to the custom HTML tag `<tg-button-row>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockbuttons).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockButtons {
    /// The buttons
    pub buttons: Vec<RichMessageButton>,
    /// Optional. Horizontal alignment of the buttons. Currently, must be one of
    /// “left”, “center”, or “right”.
    pub align: Option<String>,
}

impl RichBlockButtons {
    pub fn new(buttons: Vec<RichMessageButton>) -> Self {
        Self { buttons, align: None }
    }
    pub fn align(mut self, value: impl Into<String>) -> Self {
        self.align = Some(value.into());
        self
    }
}

/// A block with an animation, corresponding to the HTML tag `<video>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockanimation).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockAnimation {
    /// The animation
    pub animation: Animation,
    /// Optional. True, if the media preview is covered by a spoiler animation
    pub has_spoiler: Option<True>,
    /// Optional. Caption of the block
    pub caption: Option<RichBlockCaption>,
}

impl RichBlockAnimation {
    pub fn new(animation: Animation) -> Self {
        Self { animation, has_spoiler: None, caption: None }
    }
    pub fn has_spoiler(mut self, value: True) -> Self {
        self.has_spoiler = Some(value);
        self
    }
    pub fn caption(mut self, value: RichBlockCaption) -> Self {
        self.caption = Some(value);
        self
    }
}

/// A block with a music file, corresponding to the HTML tag `<audio>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockaudio).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockAudio {
    /// The audio
    pub audio: Audio,
    /// Optional. Caption of the block
    pub caption: Option<RichBlockCaption>,
}

impl RichBlockAudio {
    pub fn new(audio: Audio) -> Self {
        Self { audio, caption: None }
    }
    pub fn caption(mut self, value: RichBlockCaption) -> Self {
        self.caption = Some(value);
        self
    }
}

/// A block with a general file, corresponding to the custom HTML tag
/// `<tg-document>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockdocument).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockDocument {
    /// The document
    pub document: Document,
    /// Optional. Caption of the block
    pub caption: Option<RichBlockCaption>,
}

impl RichBlockDocument {
    pub fn new(document: Document) -> Self {
        Self { document, caption: None }
    }
    pub fn caption(mut self, value: RichBlockCaption) -> Self {
        self.caption = Some(value);
        self
    }
}

/// A block with a photo, corresponding to the HTML tag `<img>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockphoto).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockPhoto {
    /// Available sizes of the photo
    pub photo: Vec<PhotoSize>,
    /// Optional. True, if the media preview is covered by a spoiler animation
    pub has_spoiler: Option<True>,
    /// Optional. Caption of the block
    pub caption: Option<RichBlockCaption>,
}

impl RichBlockPhoto {
    pub fn new(photo: Vec<PhotoSize>) -> Self {
        Self { photo, has_spoiler: None, caption: None }
    }
    pub fn has_spoiler(mut self, value: True) -> Self {
        self.has_spoiler = Some(value);
        self
    }
    pub fn caption(mut self, value: RichBlockCaption) -> Self {
        self.caption = Some(value);
        self
    }
}

/// A block with a video, corresponding to the HTML tag `<video>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockvideo).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockVideo {
    /// The video
    pub video: Video,
    /// Optional. True, if the media preview is covered by a spoiler animation
    pub has_spoiler: Option<True>,
    /// Optional. Caption of the block
    pub caption: Option<RichBlockCaption>,
}

impl RichBlockVideo {
    pub fn new(video: Video) -> Self {
        Self { video, has_spoiler: None, caption: None }
    }
    pub fn has_spoiler(mut self, value: True) -> Self {
        self.has_spoiler = Some(value);
        self
    }
    pub fn caption(mut self, value: RichBlockCaption) -> Self {
        self.caption = Some(value);
        self
    }
}

/// A block with a voice note, corresponding to the HTML tag `<audio>`.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockvoicenote).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockVoiceNote {
    /// The voice note
    pub voice_note: Voice,
    /// Optional. Caption of the block
    pub caption: Option<RichBlockCaption>,
}

impl RichBlockVoiceNote {
    pub fn new(voice_note: Voice) -> Self {
        Self { voice_note, caption: None }
    }
    pub fn caption(mut self, value: RichBlockCaption) -> Self {
        self.caption = Some(value);
        self
    }
}

/// A block with a “Thinking…” placeholder, corresponding to the custom HTML tag
/// `<tg-thinking>`. The block may be used only in sendRichMessageDraft,
/// therefore it can't be received in messages. See https://t.me/addemoji/AIActions for examples of custom emoji that are recommended for usage
/// in the block.
///
/// [The official docs](https://core.telegram.org/bots/api#richblockthinking).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichBlockThinking {
    /// Text of the block. See https://t.me/addemoji/AIActions for examples of custom emoji that are
    /// recommended for usage in the block.
    pub text: RichText,
}

impl RichBlockThinking {
    pub fn new(text: RichText) -> Self {
        Self { text }
    }
}

impl RichText {
    /// Returns users mentioned anywhere in this rich text, including nested
    /// formatting.
    pub fn mentioned_users(&self) -> impl Iterator<Item = &User> {
        let mut users = Vec::new();
        self.collect_users(&mut users);
        users.into_iter()
    }
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        match self {
            Self::Plain(_) => {}
            Self::Concat(items) => {
                for item in items {
                    item.collect_users(users);
                }
            }
            Self::Formatted(item) => item.collect_users(users),
        }
    }
}
impl RichMessage {
    /// Returns users mentioned in all blocks, captions, tables and nested rich
    /// text.
    pub fn mentioned_users(&self) -> impl Iterator<Item = &User> {
        let mut users = Vec::new();
        self.collect_users(&mut users);
        users.into_iter()
    }
}
impl RichMessage {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        for value0 in &self.blocks {
            (value0).collect_users(users);
        }
    }
}
impl RichMessageButton {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichTextKind {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        match self {
            Self::Bold(v) => v.collect_users(users),
            Self::Italic(v) => v.collect_users(users),
            Self::Underline(v) => v.collect_users(users),
            Self::Strikethrough(v) => v.collect_users(users),
            Self::Spoiler(v) => v.collect_users(users),
            Self::DateTime(v) => v.collect_users(users),
            Self::TextMention(v) => v.collect_users(users),
            Self::Subscript(v) => v.collect_users(users),
            Self::Superscript(v) => v.collect_users(users),
            Self::Marked(v) => v.collect_users(users),
            Self::Code(v) => v.collect_users(users),
            Self::CustomEmoji(v) => v.collect_users(users),
            Self::MathematicalExpression(v) => v.collect_users(users),
            Self::Url(v) => v.collect_users(users),
            Self::EmailAddress(v) => v.collect_users(users),
            Self::PhoneNumber(v) => v.collect_users(users),
            Self::BankCardNumber(v) => v.collect_users(users),
            Self::Mention(v) => v.collect_users(users),
            Self::Hashtag(v) => v.collect_users(users),
            Self::Cashtag(v) => v.collect_users(users),
            Self::BotCommand(v) => v.collect_users(users),
            Self::Button(v) => v.collect_users(users),
            Self::Anchor(v) => v.collect_users(users),
            Self::AnchorLink(v) => v.collect_users(users),
            Self::Reference(v) => v.collect_users(users),
            Self::ReferenceLink(v) => v.collect_users(users),
        }
    }
}
impl RichTextBold {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichTextItalic {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichTextUnderline {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichTextStrikethrough {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichTextSpoiler {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichTextDateTime {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichTextTextMention {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
        users.push(&self.user);
    }
}
impl RichTextSubscript {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichTextSuperscript {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichTextMarked {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichTextCode {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichTextCustomEmoji {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        let _ = users;
    }
}
impl RichTextMathematicalExpression {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        let _ = users;
    }
}
impl RichTextUrl {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichTextEmailAddress {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichTextPhoneNumber {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichTextBankCardNumber {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichTextMention {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichTextHashtag {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichTextCashtag {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichTextBotCommand {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichTextButton {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.button.collect_users(users);
    }
}
impl RichTextAnchor {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        let _ = users;
    }
}
impl RichTextAnchorLink {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichTextReference {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichTextReferenceLink {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichBlockCaption {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
        if let Some(value0) = &self.credit {
            (value0).collect_users(users);
        }
    }
}
impl RichBlockTableCell {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        if let Some(value0) = &self.text {
            (value0).collect_users(users);
        }
    }
}
impl RichBlockListItem {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        for value0 in &self.blocks {
            (value0).collect_users(users);
        }
    }
}
impl RichBlock {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        match self {
            Self::Paragraph(v) => v.collect_users(users),
            Self::SectionHeading(v) => v.collect_users(users),
            Self::Preformatted(v) => v.collect_users(users),
            Self::Footer(v) => v.collect_users(users),
            Self::Divider(v) => v.collect_users(users),
            Self::MathematicalExpression(v) => v.collect_users(users),
            Self::Anchor(v) => v.collect_users(users),
            Self::List(v) => v.collect_users(users),
            Self::BlockQuotation(v) => v.collect_users(users),
            Self::ExpandableBlockQuotation(v) => v.collect_users(users),
            Self::PullQuotation(v) => v.collect_users(users),
            Self::Collage(v) => v.collect_users(users),
            Self::Slideshow(v) => v.collect_users(users),
            Self::Table(v) => v.collect_users(users),
            Self::Details(v) => v.collect_users(users),
            Self::Map(v) => v.collect_users(users),
            Self::Buttons(v) => v.collect_users(users),
            Self::Animation(v) => v.collect_users(users),
            Self::Audio(v) => v.collect_users(users),
            Self::Document(v) => v.collect_users(users),
            Self::Photo(v) => v.collect_users(users),
            Self::Video(v) => v.collect_users(users),
            Self::VoiceNote(v) => v.collect_users(users),
            Self::Thinking(v) => v.collect_users(users),
        }
    }
}
impl RichBlockParagraph {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichBlockSectionHeading {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichBlockPreformatted {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichBlockFooter {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
impl RichBlockDivider {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        let _ = users;
    }
}
impl RichBlockMathematicalExpression {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        let _ = users;
    }
}
impl RichBlockAnchor {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        let _ = users;
    }
}
impl RichBlockList {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        for value0 in &self.items {
            (value0).collect_users(users);
        }
    }
}
impl RichBlockBlockQuotation {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        for value0 in &self.blocks {
            (value0).collect_users(users);
        }
        if let Some(value0) = &self.credit {
            (value0).collect_users(users);
        }
    }
}
impl RichBlockExpandableBlockQuotation {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
        if let Some(value0) = &self.credit {
            (value0).collect_users(users);
        }
    }
}
impl RichBlockPullQuotation {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
        if let Some(value0) = &self.credit {
            (value0).collect_users(users);
        }
    }
}
impl RichBlockCollage {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        for value0 in &self.blocks {
            (value0).collect_users(users);
        }
        if let Some(value0) = &self.caption {
            (value0).collect_users(users);
        }
    }
}
impl RichBlockSlideshow {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        for value0 in &self.blocks {
            (value0).collect_users(users);
        }
        if let Some(value0) = &self.caption {
            (value0).collect_users(users);
        }
    }
}
impl RichBlockTable {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        for value0 in &self.cells {
            for value1 in value0 {
                (value1).collect_users(users);
            }
        }
        if let Some(value0) = &self.caption {
            (value0).collect_users(users);
        }
    }
}
impl RichBlockDetails {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.summary.collect_users(users);
        for value0 in &self.blocks {
            (value0).collect_users(users);
        }
    }
}
impl RichBlockMap {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        if let Some(value0) = &self.caption {
            (value0).collect_users(users);
        }
    }
}
impl RichBlockButtons {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        for value0 in &self.buttons {
            (value0).collect_users(users);
        }
    }
}
impl RichBlockAnimation {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        if let Some(value0) = &self.caption {
            (value0).collect_users(users);
        }
    }
}
impl RichBlockAudio {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        if let Some(value0) = &self.caption {
            (value0).collect_users(users);
        }
    }
}
impl RichBlockDocument {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        if let Some(value0) = &self.caption {
            (value0).collect_users(users);
        }
    }
}
impl RichBlockPhoto {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        if let Some(value0) = &self.caption {
            (value0).collect_users(users);
        }
    }
}
impl RichBlockVideo {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        if let Some(value0) = &self.caption {
            (value0).collect_users(users);
        }
    }
}
impl RichBlockVoiceNote {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        if let Some(value0) = &self.caption {
            (value0).collect_users(users);
        }
    }
}
impl RichBlockThinking {
    fn collect_users<'a>(&'a self, users: &mut Vec<&'a User>) {
        self.text.collect_users(users);
    }
}
