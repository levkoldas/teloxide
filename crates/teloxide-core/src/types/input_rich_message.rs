use crate::types::*;
use serde::{Deserialize, Serialize};

/// Describes a rich message to be sent. Exactly one of the fields html,
/// markdown, or blocks must be used.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichmessage).
///
/// ```
/// use teloxide_core::{
///     prelude::*,
///     types::{ChatId, InputRichBlockParagraph, InputRichMessage},
/// };
///
/// let bot = Bot::new("TOKEN");
/// let content = InputRichMessage::new()
///     .blocks(vec![InputRichBlockParagraph::new("Hello from a rich message".into()).into()]);
/// let request = bot.send_rich_message(ChatId(42), content);
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichMessage {
    /// Optional. Content of the rich message to send described as a list of
    /// blocks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<InputRichBlock>>,
    /// Optional. Content of the rich message to send described using HTML
    /// formatting. See rich message formatting options for more details.
    /// Use media field to specify the media used in the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// Optional. Content of the rich message to send described using Markdown
    /// formatting. See rich message formatting options for more details.
    /// Use media field to specify the media used in the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<String>,
    /// Optional. List of media that are specified in the markdown or html
    /// fields using tg://photo?id=, tg://video?id=, tg://document?id=, and
    /// tg://audio?id= links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Vec<InputRichMessageMedia>>,
    /// Optional. Pass True if the rich message must be shown right-to-left
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rtl: Option<bool>,
    /// Optional. Pass True to skip automatic detection of entities (e.g., URLs,
    /// email addresses, username mentions, hashtags, cashtags, bot
    /// commands, or phone numbers) in the text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_entity_detection: Option<bool>,
}
impl InputRichMessage {
    pub fn new() -> Self {
        Self {
            blocks: None,
            html: None,
            markdown: None,
            media: None,
            is_rtl: None,
            skip_entity_detection: None,
        }
    }
    pub fn blocks(mut self, value: Vec<InputRichBlock>) -> Self {
        self.blocks = Some(value);
        self
    }
    pub fn html(mut self, value: impl Into<String>) -> Self {
        self.html = Some(value.into());
        self
    }
    pub fn markdown(mut self, value: impl Into<String>) -> Self {
        self.markdown = Some(value.into());
        self
    }
    pub fn media(mut self, value: Vec<InputRichMessageMedia>) -> Self {
        self.media = Some(value);
        self
    }
    pub fn is_rtl(mut self, value: bool) -> Self {
        self.is_rtl = Some(value);
        self
    }
    pub fn skip_entity_detection(mut self, value: bool) -> Self {
        self.skip_entity_detection = Some(value);
        self
    }
}
impl Default for InputRichMessage {
    fn default() -> Self {
        Self::new()
    }
}

/// Describes a media element embedded in an outgoing rich message.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichmessagemedia).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichMessageMedia {
    /// Unique identifier of the media used in a tg://photo?id=, tg://video?id=,
    /// tg://document?id=, or tg://audio?id= link. 1-64 characters, only
    /// A-Z, a-z, 0-9, _ and - are allowed.
    pub id: String,
    /// The media to be sent. Everything except the media itself and its
    /// properties is ignored.
    pub media: InputRichMedia,
}
impl InputRichMessageMedia {
    pub fn new(id: impl Into<String>, media: InputRichMedia) -> Self {
        Self { id: id.into(), media }
    }
}

/// An item of a list to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblocklistitem).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockListItem {
    /// The content of the item
    pub blocks: Vec<InputRichBlock>,
    /// Optional. Pass True if the item has a checkbox
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_checkbox: Option<True>,
    /// Optional. Pass True if the item has a checked checkbox
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_checked: Option<True>,
    /// Optional. For ordered lists, the numeric value of the item label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    /// Optional. For ordered lists, the type of the item label; must be one of
    /// “a” for lowercase letters, “A” for uppercase letters, “i” for
    /// lowercase Roman numerals, “I” for uppercase Roman numerals, or “1”
    /// for decimal numbers
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
impl InputRichBlockListItem {
    pub fn new(blocks: Vec<InputRichBlock>) -> Self {
        Self { blocks, has_checkbox: None, is_checked: None, value: None, kind: None }
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

/// This object represents a block in a rich formatted message to be sent.
/// Currently, it can be any of the following types:
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblock).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type")]
pub enum InputRichBlock {
    #[serde(rename = "paragraph")]
    Paragraph(Box<InputRichBlockParagraph>),
    #[serde(rename = "heading")]
    SectionHeading(Box<InputRichBlockSectionHeading>),
    #[serde(rename = "pre")]
    Preformatted(Box<InputRichBlockPreformatted>),
    #[serde(rename = "footer")]
    Footer(Box<InputRichBlockFooter>),
    #[serde(rename = "divider")]
    Divider(Box<InputRichBlockDivider>),
    #[serde(rename = "mathematical_expression")]
    MathematicalExpression(Box<InputRichBlockMathematicalExpression>),
    #[serde(rename = "anchor")]
    Anchor(Box<InputRichBlockAnchor>),
    #[serde(rename = "list")]
    List(Box<InputRichBlockList>),
    #[serde(rename = "blockquote")]
    BlockQuotation(Box<InputRichBlockBlockQuotation>),
    #[serde(rename = "expandable_blockquote")]
    ExpandableBlockQuotation(Box<InputRichBlockExpandableBlockQuotation>),
    #[serde(rename = "pullquote")]
    PullQuotation(Box<InputRichBlockPullQuotation>),
    #[serde(rename = "collage")]
    Collage(Box<InputRichBlockCollage>),
    #[serde(rename = "slideshow")]
    Slideshow(Box<InputRichBlockSlideshow>),
    #[serde(rename = "table")]
    Table(Box<InputRichBlockTable>),
    #[serde(rename = "details")]
    Details(Box<InputRichBlockDetails>),
    #[serde(rename = "map")]
    Map(Box<InputRichBlockMap>),
    #[serde(rename = "buttons")]
    Buttons(Box<InputRichBlockButtons>),
    #[serde(rename = "animation")]
    Animation(Box<InputRichBlockAnimation>),
    #[serde(rename = "audio")]
    Audio(Box<InputRichBlockAudio>),
    #[serde(rename = "document")]
    Document(Box<InputRichBlockDocument>),
    #[serde(rename = "photo")]
    Photo(Box<InputRichBlockPhoto>),
    #[serde(rename = "video")]
    Video(Box<InputRichBlockVideo>),
    #[serde(rename = "voice_note")]
    VoiceNote(Box<InputRichBlockVoiceNote>),
    #[serde(rename = "thinking")]
    Thinking(Box<InputRichBlockThinking>),
}
impl From<InputRichBlockParagraph> for InputRichBlock {
    fn from(value: InputRichBlockParagraph) -> Self {
        Self::Paragraph(Box::new(value))
    }
}
impl From<InputRichBlockSectionHeading> for InputRichBlock {
    fn from(value: InputRichBlockSectionHeading) -> Self {
        Self::SectionHeading(Box::new(value))
    }
}
impl From<InputRichBlockPreformatted> for InputRichBlock {
    fn from(value: InputRichBlockPreformatted) -> Self {
        Self::Preformatted(Box::new(value))
    }
}
impl From<InputRichBlockFooter> for InputRichBlock {
    fn from(value: InputRichBlockFooter) -> Self {
        Self::Footer(Box::new(value))
    }
}
impl From<InputRichBlockDivider> for InputRichBlock {
    fn from(value: InputRichBlockDivider) -> Self {
        Self::Divider(Box::new(value))
    }
}
impl From<InputRichBlockMathematicalExpression> for InputRichBlock {
    fn from(value: InputRichBlockMathematicalExpression) -> Self {
        Self::MathematicalExpression(Box::new(value))
    }
}
impl From<InputRichBlockAnchor> for InputRichBlock {
    fn from(value: InputRichBlockAnchor) -> Self {
        Self::Anchor(Box::new(value))
    }
}
impl From<InputRichBlockList> for InputRichBlock {
    fn from(value: InputRichBlockList) -> Self {
        Self::List(Box::new(value))
    }
}
impl From<InputRichBlockBlockQuotation> for InputRichBlock {
    fn from(value: InputRichBlockBlockQuotation) -> Self {
        Self::BlockQuotation(Box::new(value))
    }
}
impl From<InputRichBlockExpandableBlockQuotation> for InputRichBlock {
    fn from(value: InputRichBlockExpandableBlockQuotation) -> Self {
        Self::ExpandableBlockQuotation(Box::new(value))
    }
}
impl From<InputRichBlockPullQuotation> for InputRichBlock {
    fn from(value: InputRichBlockPullQuotation) -> Self {
        Self::PullQuotation(Box::new(value))
    }
}
impl From<InputRichBlockCollage> for InputRichBlock {
    fn from(value: InputRichBlockCollage) -> Self {
        Self::Collage(Box::new(value))
    }
}
impl From<InputRichBlockSlideshow> for InputRichBlock {
    fn from(value: InputRichBlockSlideshow) -> Self {
        Self::Slideshow(Box::new(value))
    }
}
impl From<InputRichBlockTable> for InputRichBlock {
    fn from(value: InputRichBlockTable) -> Self {
        Self::Table(Box::new(value))
    }
}
impl From<InputRichBlockDetails> for InputRichBlock {
    fn from(value: InputRichBlockDetails) -> Self {
        Self::Details(Box::new(value))
    }
}
impl From<InputRichBlockMap> for InputRichBlock {
    fn from(value: InputRichBlockMap) -> Self {
        Self::Map(Box::new(value))
    }
}
impl From<InputRichBlockButtons> for InputRichBlock {
    fn from(value: InputRichBlockButtons) -> Self {
        Self::Buttons(Box::new(value))
    }
}
impl From<InputRichBlockAnimation> for InputRichBlock {
    fn from(value: InputRichBlockAnimation) -> Self {
        Self::Animation(Box::new(value))
    }
}
impl From<InputRichBlockAudio> for InputRichBlock {
    fn from(value: InputRichBlockAudio) -> Self {
        Self::Audio(Box::new(value))
    }
}
impl From<InputRichBlockDocument> for InputRichBlock {
    fn from(value: InputRichBlockDocument) -> Self {
        Self::Document(Box::new(value))
    }
}
impl From<InputRichBlockPhoto> for InputRichBlock {
    fn from(value: InputRichBlockPhoto) -> Self {
        Self::Photo(Box::new(value))
    }
}
impl From<InputRichBlockVideo> for InputRichBlock {
    fn from(value: InputRichBlockVideo) -> Self {
        Self::Video(Box::new(value))
    }
}
impl From<InputRichBlockVoiceNote> for InputRichBlock {
    fn from(value: InputRichBlockVoiceNote) -> Self {
        Self::VoiceNote(Box::new(value))
    }
}
impl From<InputRichBlockThinking> for InputRichBlock {
    fn from(value: InputRichBlockThinking) -> Self {
        Self::Thinking(Box::new(value))
    }
}

/// A text paragraph, corresponding to the HTML tag `<p>`.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockparagraph).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockParagraph {
    /// Text of the block
    pub text: RichText,
}
impl InputRichBlockParagraph {
    pub fn new(text: RichText) -> Self {
        Self { text }
    }
}

/// A section heading, corresponding to the HTML tags `<h1>`, `<h2>`, `<h3>`,
/// `<h4>`, `<h5>`, or `<h6>`.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblocksectionheading).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockSectionHeading {
    /// Text of the block
    pub text: RichText,
    /// Relative size of the text font; 1-6, 1 is the largest, 6 is the smallest
    pub size: u32,
}
impl InputRichBlockSectionHeading {
    pub fn new(text: RichText, size: u32) -> Self {
        Self { text, size }
    }
}

/// A preformatted text block, corresponding to the nested HTML tags `<pre>` and
/// `<code>`.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockpreformatted).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockPreformatted {
    /// Text of the block
    pub text: RichText,
    /// Optional. The programming language of the text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}
impl InputRichBlockPreformatted {
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
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockfooter).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockFooter {
    /// Text of the block
    pub text: RichText,
}
impl InputRichBlockFooter {
    pub fn new(text: RichText) -> Self {
        Self { text }
    }
}

/// A divider, corresponding to the HTML tag `<hr/>`.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockdivider).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockDivider {}
impl InputRichBlockDivider {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for InputRichBlockDivider {
    fn default() -> Self {
        Self::new()
    }
}

/// A block with a mathematical expression in LaTeX format, corresponding to the
/// custom HTML tag `<tg-math-block>`.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockmathematicalexpression).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockMathematicalExpression {
    /// The mathematical expression in LaTeX format
    pub expression: String,
}
impl InputRichBlockMathematicalExpression {
    pub fn new(expression: impl Into<String>) -> Self {
        Self { expression: expression.into() }
    }
}

/// A block with an anchor, corresponding to the HTML tag `<a>` with the
/// attribute name.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockanchor).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockAnchor {
    /// The name of the anchor
    pub name: String,
}
impl InputRichBlockAnchor {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// A list of blocks, corresponding to the HTML tag `<ul>` or `<ol>` with
/// multiple nested tags `<li>`.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblocklist).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockList {
    /// Items of the list
    pub items: Vec<InputRichBlockListItem>,
}
impl InputRichBlockList {
    pub fn new(items: Vec<InputRichBlockListItem>) -> Self {
        Self { items }
    }
}

/// A block quotation, corresponding to the HTML tag `<blockquote>`.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockblockquotation).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockBlockQuotation {
    /// Content of the block
    pub blocks: Vec<InputRichBlock>,
    /// Optional. Credit of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<RichText>,
}
impl InputRichBlockBlockQuotation {
    pub fn new(blocks: Vec<InputRichBlock>) -> Self {
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
/// [The official
/// docs](https://core.telegram.org/bots/api#inputrichblockexpandableblockquotation).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockExpandableBlockQuotation {
    /// Content of the block
    pub text: RichText,
    /// Optional. Credit of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<RichText>,
}
impl InputRichBlockExpandableBlockQuotation {
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
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockpullquotation).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockPullQuotation {
    /// Text of the block
    pub text: RichText,
    /// Optional. Credit of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<RichText>,
}
impl InputRichBlockPullQuotation {
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
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockcollage).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockCollage {
    /// Elements of the collage
    pub blocks: Vec<InputRichBlock>,
    /// Optional. Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}
impl InputRichBlockCollage {
    pub fn new(blocks: Vec<InputRichBlock>) -> Self {
        Self { blocks, caption: None }
    }
    pub fn caption(mut self, value: RichBlockCaption) -> Self {
        self.caption = Some(value);
        self
    }
}

/// A slideshow, corresponding to the custom HTML tag `<tg-slideshow>`.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockslideshow).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockSlideshow {
    /// Elements of the slideshow
    pub blocks: Vec<InputRichBlock>,
    /// Optional. Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}
impl InputRichBlockSlideshow {
    pub fn new(blocks: Vec<InputRichBlock>) -> Self {
        Self { blocks, caption: None }
    }
    pub fn caption(mut self, value: RichBlockCaption) -> Self {
        self.caption = Some(value);
        self
    }
}

/// A table, corresponding to the HTML tag `<table>`.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblocktable).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockTable {
    /// Cells of the table
    pub cells: Vec<Vec<RichBlockTableCell>>,
    /// Optional. Pass True if the table has borders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bordered: Option<True>,
    /// Optional. Pass True if the table is striped
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_striped: Option<True>,
    /// Optional. Pass True if table cells must have smaller indents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_compact: Option<True>,
    /// Optional. Caption of the table
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichText>,
}
impl InputRichBlockTable {
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
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockdetails).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockDetails {
    /// Always shown summary of the block
    pub summary: RichText,
    /// Content of the block
    pub blocks: Vec<InputRichBlock>,
    /// Optional. Pass True if the content of the block is visible by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_open: Option<True>,
}
impl InputRichBlockDetails {
    pub fn new(summary: RichText, blocks: Vec<InputRichBlock>) -> Self {
        Self { summary, blocks, is_open: None }
    }
    pub fn is_open(mut self, value: True) -> Self {
        self.is_open = Some(value);
        self
    }
}

/// A block with a map, corresponding to the custom HTML tag `<tg-map>`. The
/// map's width and height must not exceed 10000 in total. The width and height
/// ratio must be at most 20.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockmap).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockMap {
    /// Location of the center of the map
    pub location: Location,
    /// Optional. Map zoom level; 0-24
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom: Option<u32>,
    /// Optional. Map width; 0-10000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    /// Optional. Map height; 0-10000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    /// Optional. Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}
impl InputRichBlockMap {
    pub fn new(location: Location) -> Self {
        Self { location, zoom: None, width: None, height: None, caption: None }
    }
    pub fn zoom(mut self, value: u32) -> Self {
        self.zoom = Some(value);
        self
    }
    pub fn width(mut self, value: u32) -> Self {
        self.width = Some(value);
        self
    }
    pub fn height(mut self, value: u32) -> Self {
        self.height = Some(value);
        self
    }
    pub fn caption(mut self, value: RichBlockCaption) -> Self {
        self.caption = Some(value);
        self
    }
}

/// A block containing a list of buttons that are shown in one row,
/// corresponding to the custom HTML tag `<tg-button-row>`.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockbuttons).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockButtons {
    /// List of 1-8 buttons to send
    pub buttons: Vec<RichMessageButton>,
    /// Optional. Horizontal alignment of the buttons. Currently, must be one of
    /// “left”, “center”, or “right”.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<String>,
}
impl InputRichBlockButtons {
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
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockanimation).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockAnimation {
    /// The animation. Caption is ignored.
    #[serde(with = "input_media_animation_tagged")]
    #[cfg_attr(test, schemars(with = "input_media_animation_tagged::Schema"))]
    pub animation: InputMediaAnimation,
    /// Optional. Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}
impl InputRichBlockAnimation {
    pub fn new(animation: InputMediaAnimation) -> Self {
        Self { animation, caption: None }
    }
    pub fn caption(mut self, value: RichBlockCaption) -> Self {
        self.caption = Some(value);
        self
    }
}

/// A block with a music file, corresponding to the HTML tag `<audio>`.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockaudio).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockAudio {
    /// The audio. Caption is ignored.
    #[serde(with = "input_media_audio_tagged")]
    #[cfg_attr(test, schemars(with = "input_media_audio_tagged::Schema"))]
    pub audio: InputMediaAudio,
    /// Optional. Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}
impl InputRichBlockAudio {
    pub fn new(audio: InputMediaAudio) -> Self {
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
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockdocument).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockDocument {
    /// The document. Caption is ignored.
    #[serde(with = "input_media_document_tagged")]
    #[cfg_attr(test, schemars(with = "input_media_document_tagged::Schema"))]
    pub document: InputMediaDocument,
    /// Optional. Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}
impl InputRichBlockDocument {
    pub fn new(document: InputMediaDocument) -> Self {
        Self { document, caption: None }
    }
    pub fn caption(mut self, value: RichBlockCaption) -> Self {
        self.caption = Some(value);
        self
    }
}

/// A block with a photo, corresponding to the HTML tag `<img>`.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockphoto).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockPhoto {
    /// The photo. Caption is ignored.
    #[serde(with = "input_media_photo_tagged")]
    #[cfg_attr(test, schemars(with = "input_media_photo_tagged::Schema"))]
    pub photo: InputMediaPhoto,
    /// Optional. Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}
impl InputRichBlockPhoto {
    pub fn new(photo: InputMediaPhoto) -> Self {
        Self { photo, caption: None }
    }
    pub fn caption(mut self, value: RichBlockCaption) -> Self {
        self.caption = Some(value);
        self
    }
}

/// A block with a video, corresponding to the HTML tag `<video>`.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockvideo).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockVideo {
    /// The video. Caption is ignored.
    #[serde(with = "input_media_video_tagged")]
    #[cfg_attr(test, schemars(with = "input_media_video_tagged::Schema"))]
    pub video: InputMediaVideo,
    /// Optional. Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}
impl InputRichBlockVideo {
    pub fn new(video: InputMediaVideo) -> Self {
        Self { video, caption: None }
    }
    pub fn caption(mut self, value: RichBlockCaption) -> Self {
        self.caption = Some(value);
        self
    }
}

/// A block with a voice note, corresponding to the HTML tag `<audio>`.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockvoicenote).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockVoiceNote {
    /// The voice note. Caption is ignored.
    #[serde(with = "input_media_voice_note_tagged")]
    #[cfg_attr(test, schemars(with = "input_media_voice_note_tagged::Schema"))]
    pub voice_note: InputMediaVoiceNote,
    /// Optional. Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}
impl InputRichBlockVoiceNote {
    pub fn new(voice_note: InputMediaVoiceNote) -> Self {
        Self { voice_note, caption: None }
    }
    pub fn caption(mut self, value: RichBlockCaption) -> Self {
        self.caption = Some(value);
        self
    }
}

/// A block with a “Thinking…” placeholder, corresponding to the custom HTML tag
/// `<tg-thinking>`. The block may be used only in sendRichMessageDraft,
/// therefore it can't be received in messages. See https://t.me/addemoji/AIActions for examples of custom emoji that are
/// recommended for usage in the block.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichblockthinking).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichBlockThinking {
    /// Text of the block. See https://t.me/addemoji/AIActions for examples of custom emoji that are
    /// recommended for usage in the block.
    pub text: RichText,
}
impl InputRichBlockThinking {
    pub fn new(text: RichText) -> Self {
        Self { text }
    }
}

/// Represents the content of a rich message to be sent as the result of an
/// inline query.
///
/// [The official docs](https://core.telegram.org/bots/api#inputrichmessagecontent).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputRichMessageContent {
    /// The message to be sent. Only previously uploaded files may be used in
    /// the message.
    pub rich_message: InputRichMessage,
}
impl InputRichMessageContent {
    pub fn new(rich_message: InputRichMessage) -> Self {
        Self { rich_message }
    }
}

/// Represents an HTTP link to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmedialink).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputMediaLink {
    /// HTTP URL of the link
    pub url: String,
}
impl InputMediaLink {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}

/// Represents a live photo to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmedialivephoto).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputMediaLivePhoto {
    /// Video of the live photo to send. Pass a file_id to send a file that
    /// exists on the Telegram servers (recommended) or pass
    /// “attach://`<file_attach_name>`” to upload a new one using
    /// multipart/form-data under `<file_attach_name>` name. More information on
    /// Sending Files ». Sending live photos by a URL is currently
    /// unsupported.
    pub media: InputFile,
    /// The static photo to send. Pass a file_id to send a file that exists on
    /// the Telegram servers (recommended) or pass
    /// “attach://`<file_attach_name>`” to upload a new one using
    /// multipart/form- data under `<file_attach_name>` name. More
    /// information on Sending Files ». Sending live photos by a URL is
    /// currently unsupported.
    pub photo: InputFile,
    /// Optional. Caption of the live photo to be sent, 0-1024 characters after
    /// entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the live photo caption. See
    /// formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Optional. List of special entities that appear in the caption, which can
    /// be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Pass True if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Optional. Pass True if the live photo needs to be covered with a spoiler
    /// animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
}
impl InputMediaLivePhoto {
    pub fn new(media: InputFile, photo: InputFile) -> Self {
        Self {
            media,
            photo,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: None,
            has_spoiler: None,
        }
    }
    pub fn caption(mut self, value: impl Into<String>) -> Self {
        self.caption = Some(value.into());
        self
    }
    pub fn parse_mode(mut self, value: ParseMode) -> Self {
        self.parse_mode = Some(value);
        self
    }
    pub fn caption_entities(mut self, value: Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(value);
        self
    }
    pub fn show_caption_above_media(mut self, value: bool) -> Self {
        self.show_caption_above_media = Some(value);
        self
    }
    pub fn has_spoiler(mut self, value: bool) -> Self {
        self.has_spoiler = Some(value);
        self
    }
}

/// Represents a location to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmedialocation).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputMediaLocation {
    /// Latitude of the location
    pub latitude: f64,
    /// Longitude of the location
    pub longitude: f64,
    /// Optional. The radius of uncertainty for the location, measured in
    /// meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
}
impl InputMediaLocation {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self { latitude, longitude, horizontal_accuracy: None }
    }
    pub fn horizontal_accuracy(mut self, value: f64) -> Self {
        self.horizontal_accuracy = Some(value);
        self
    }
}

/// Represents a sticker file to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmediasticker).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputMediaSticker {
    /// File to send. Pass a file_id to send a file that exists on the Telegram
    /// servers (recommended), pass an HTTP URL for Telegram to get a .WEBP
    /// sticker from the Internet, or pass “attach://`<file_attach_name>`” to
    /// upload a new .WEBP, .TGS, or .WEBM sticker using multipart/form-data
    /// under `<file_attach_name>` name. More information on Sending Files »
    pub media: InputFile,
    /// Optional. Emoji associated with the sticker; only for just uploaded
    /// stickers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
}
impl InputMediaSticker {
    pub fn new(media: InputFile) -> Self {
        Self { media, emoji: None }
    }
    pub fn emoji(mut self, value: impl Into<String>) -> Self {
        self.emoji = Some(value.into());
        self
    }
}

/// Represents a venue to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmediavenue).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputMediaVenue {
    /// Latitude of the location
    pub latitude: f64,
    /// Longitude of the location
    pub longitude: f64,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Optional. Foursquare identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Optional. Foursquare type of the venue, if known. (For example,
    /// “arts_entertainment/default”, “arts_entertainment/aquarium” or
    /// “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Optional. Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Optional. Google Places type of the venue. (See supported types.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
}
impl InputMediaVenue {
    pub fn new(
        latitude: f64,
        longitude: f64,
        title: impl Into<String>,
        address: impl Into<String>,
    ) -> Self {
        Self {
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
        }
    }
    pub fn foursquare_id(mut self, value: impl Into<String>) -> Self {
        self.foursquare_id = Some(value.into());
        self
    }
    pub fn foursquare_type(mut self, value: impl Into<String>) -> Self {
        self.foursquare_type = Some(value.into());
        self
    }
    pub fn google_place_id(mut self, value: impl Into<String>) -> Self {
        self.google_place_id = Some(value.into());
        self
    }
    pub fn google_place_type(mut self, value: impl Into<String>) -> Self {
        self.google_place_type = Some(value.into());
        self
    }
}

/// Represents a voice message file to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmediavoicenote).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputMediaVoiceNote {
    /// File to send. Pass a file_id to send a file that exists on the Telegram
    /// servers (recommended), pass an HTTP URL for Telegram to get a file
    /// from the Internet, or pass "attach://`<file_attach_name>`" to upload a
    /// new one using multipart/form-data under `<file_attach_name>` name.
    /// More information on Sending Files »
    pub media: InputFile,
    /// Optional. Caption of the voice message to be sent, 0-1024 characters
    /// after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the voice message caption. See
    /// formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Optional. List of special entities that appear in the caption, which can
    /// be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Duration of the voice message in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Seconds>,
}
impl InputMediaVoiceNote {
    pub fn new(media: InputFile) -> Self {
        Self { media, caption: None, parse_mode: None, caption_entities: None, duration: None }
    }
    pub fn caption(mut self, value: impl Into<String>) -> Self {
        self.caption = Some(value.into());
        self
    }
    pub fn parse_mode(mut self, value: ParseMode) -> Self {
        self.parse_mode = Some(value);
        self
    }
    pub fn caption_entities(mut self, value: Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(value);
        self
    }
    pub fn duration(mut self, value: Seconds) -> Self {
        self.duration = Some(value);
        self
    }
}

/// The paid media to send is a live photo.
///
/// [The official docs](https://core.telegram.org/bots/api#inputpaidmedialivephoto).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputPaidMediaLivePhoto {
    /// Video of the live photo to send. Pass a file_id to send a file that
    /// exists on the Telegram servers (recommended) or pass
    /// “attach://`<file_attach_name>`” to upload a new one using
    /// multipart/form-data under `<file_attach_name>` name. More information on
    /// Sending Files ». Sending live photos by a URL is currently
    /// unsupported.
    pub media: InputFile,
    /// The static photo to send. Pass a file_id to send a file that exists on
    /// the Telegram servers (recommended) or pass
    /// “attach://`<file_attach_name>`” to upload a new one using
    /// multipart/form- data under `<file_attach_name>` name. More
    /// information on Sending Files ». Sending live photos by a URL is
    /// currently unsupported.
    pub photo: InputFile,
}
impl InputPaidMediaLivePhoto {
    pub fn new(media: InputFile, photo: InputFile) -> Self {
        Self { media, photo }
    }
}

/// This object represents the content of a poll description or a quiz
/// explanation to be sent. It should be one of
///
/// [The official docs](https://core.telegram.org/bots/api#inputpollmedia).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type")]
pub enum InputPollMedia {
    #[serde(rename = "animation")]
    Animation(Box<InputMediaAnimation>),
    #[serde(rename = "audio")]
    Audio(Box<InputMediaAudio>),
    #[serde(rename = "document")]
    Document(Box<InputMediaDocument>),
    #[serde(rename = "live_photo")]
    LivePhoto(Box<InputMediaLivePhoto>),
    #[serde(rename = "location")]
    Location(Box<InputMediaLocation>),
    #[serde(rename = "photo")]
    Photo(Box<InputMediaPhoto>),
    #[serde(rename = "venue")]
    Venue(Box<InputMediaVenue>),
    #[serde(rename = "video")]
    Video(Box<InputMediaVideo>),
}
impl From<InputMediaAnimation> for InputPollMedia {
    fn from(value: InputMediaAnimation) -> Self {
        Self::Animation(Box::new(value))
    }
}
impl From<InputMediaAudio> for InputPollMedia {
    fn from(value: InputMediaAudio) -> Self {
        Self::Audio(Box::new(value))
    }
}
impl From<InputMediaDocument> for InputPollMedia {
    fn from(value: InputMediaDocument) -> Self {
        Self::Document(Box::new(value))
    }
}
impl From<InputMediaLivePhoto> for InputPollMedia {
    fn from(value: InputMediaLivePhoto) -> Self {
        Self::LivePhoto(Box::new(value))
    }
}
impl From<InputMediaLocation> for InputPollMedia {
    fn from(value: InputMediaLocation) -> Self {
        Self::Location(Box::new(value))
    }
}
impl From<InputMediaPhoto> for InputPollMedia {
    fn from(value: InputMediaPhoto) -> Self {
        Self::Photo(Box::new(value))
    }
}
impl From<InputMediaVenue> for InputPollMedia {
    fn from(value: InputMediaVenue) -> Self {
        Self::Venue(Box::new(value))
    }
}
impl From<InputMediaVideo> for InputPollMedia {
    fn from(value: InputMediaVideo) -> Self {
        Self::Video(Box::new(value))
    }
}

/// This object represents the content of a poll option to be sent. It should be
/// one of
///
/// [The official docs](https://core.telegram.org/bots/api#inputpolloptionmedia).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type")]
pub enum InputPollOptionMedia {
    #[serde(rename = "animation")]
    Animation(Box<InputMediaAnimation>),
    #[serde(rename = "link")]
    Link(Box<InputMediaLink>),
    #[serde(rename = "live_photo")]
    LivePhoto(Box<InputMediaLivePhoto>),
    #[serde(rename = "location")]
    Location(Box<InputMediaLocation>),
    #[serde(rename = "photo")]
    Photo(Box<InputMediaPhoto>),
    #[serde(rename = "sticker")]
    Sticker(Box<InputMediaSticker>),
    #[serde(rename = "venue")]
    Venue(Box<InputMediaVenue>),
    #[serde(rename = "video")]
    Video(Box<InputMediaVideo>),
}
impl From<InputMediaAnimation> for InputPollOptionMedia {
    fn from(value: InputMediaAnimation) -> Self {
        Self::Animation(Box::new(value))
    }
}
impl From<InputMediaLink> for InputPollOptionMedia {
    fn from(value: InputMediaLink) -> Self {
        Self::Link(Box::new(value))
    }
}
impl From<InputMediaLivePhoto> for InputPollOptionMedia {
    fn from(value: InputMediaLivePhoto) -> Self {
        Self::LivePhoto(Box::new(value))
    }
}
impl From<InputMediaLocation> for InputPollOptionMedia {
    fn from(value: InputMediaLocation) -> Self {
        Self::Location(Box::new(value))
    }
}
impl From<InputMediaPhoto> for InputPollOptionMedia {
    fn from(value: InputMediaPhoto) -> Self {
        Self::Photo(Box::new(value))
    }
}
impl From<InputMediaSticker> for InputPollOptionMedia {
    fn from(value: InputMediaSticker) -> Self {
        Self::Sticker(Box::new(value))
    }
}
impl From<InputMediaVenue> for InputPollOptionMedia {
    fn from(value: InputMediaVenue) -> Self {
        Self::Venue(Box::new(value))
    }
}
impl From<InputMediaVideo> for InputPollOptionMedia {
    fn from(value: InputMediaVideo) -> Self {
        Self::Video(Box::new(value))
    }
}

/// Media that can be attached to an input rich message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type")]
pub enum InputRichMedia {
    #[serde(rename = "animation")]
    Animation(Box<InputMediaAnimation>),
    #[serde(rename = "audio")]
    Audio(Box<InputMediaAudio>),
    #[serde(rename = "document")]
    Document(Box<InputMediaDocument>),
    #[serde(rename = "photo")]
    Photo(Box<InputMediaPhoto>),
    #[serde(rename = "video")]
    Video(Box<InputMediaVideo>),
    #[serde(rename = "voice_note")]
    VoiceNote(Box<InputMediaVoiceNote>),
}
impl From<InputMediaAnimation> for InputRichMedia {
    fn from(value: InputMediaAnimation) -> Self {
        Self::Animation(Box::new(value))
    }
}
impl From<InputMediaAudio> for InputRichMedia {
    fn from(value: InputMediaAudio) -> Self {
        Self::Audio(Box::new(value))
    }
}
impl From<InputMediaDocument> for InputRichMedia {
    fn from(value: InputMediaDocument) -> Self {
        Self::Document(Box::new(value))
    }
}
impl From<InputMediaPhoto> for InputRichMedia {
    fn from(value: InputMediaPhoto) -> Self {
        Self::Photo(Box::new(value))
    }
}
impl From<InputMediaVideo> for InputRichMedia {
    fn from(value: InputMediaVideo) -> Self {
        Self::Video(Box::new(value))
    }
}
impl From<InputMediaVoiceNote> for InputRichMedia {
    fn from(value: InputMediaVoiceNote) -> Self {
        Self::VoiceNote(Box::new(value))
    }
}

mod input_media_animation_tagged {
    #[cfg(test)]
    #[derive(serde::Deserialize, schemars::JsonSchema)]
    #[serde(tag = "type")]
    #[allow(dead_code)]
    pub enum Schema {
        #[serde(rename = "animation")]
        Media(crate::types::InputMediaAnimation),
    }

    use super::*;
    pub fn serialize<S: serde::Serializer>(
        value: &InputMediaAnimation,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(tag = "type")]
        enum Tagged<'a> {
            #[serde(rename = "animation")]
            Media(&'a InputMediaAnimation),
        }
        Tagged::Media(value).serialize(serializer)
    }
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<InputMediaAnimation, D::Error> {
        #[derive(Deserialize)]
        #[serde(tag = "type")]
        enum Tagged {
            #[serde(rename = "animation")]
            Media(InputMediaAnimation),
        }
        let Tagged::Media(value) = Tagged::deserialize(deserializer)?;
        Ok(value)
    }
}

mod input_media_audio_tagged {
    #[cfg(test)]
    #[derive(serde::Deserialize, schemars::JsonSchema)]
    #[serde(tag = "type")]
    #[allow(dead_code)]
    pub enum Schema {
        #[serde(rename = "audio")]
        Media(crate::types::InputMediaAudio),
    }

    use super::*;
    pub fn serialize<S: serde::Serializer>(
        value: &InputMediaAudio,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(tag = "type")]
        enum Tagged<'a> {
            #[serde(rename = "audio")]
            Media(&'a InputMediaAudio),
        }
        Tagged::Media(value).serialize(serializer)
    }
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<InputMediaAudio, D::Error> {
        #[derive(Deserialize)]
        #[serde(tag = "type")]
        enum Tagged {
            #[serde(rename = "audio")]
            Media(InputMediaAudio),
        }
        let Tagged::Media(value) = Tagged::deserialize(deserializer)?;
        Ok(value)
    }
}

mod input_media_document_tagged {
    #[cfg(test)]
    #[derive(serde::Deserialize, schemars::JsonSchema)]
    #[serde(tag = "type")]
    #[allow(dead_code)]
    pub enum Schema {
        #[serde(rename = "document")]
        Media(crate::types::InputMediaDocument),
    }

    use super::*;
    pub fn serialize<S: serde::Serializer>(
        value: &InputMediaDocument,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(tag = "type")]
        enum Tagged<'a> {
            #[serde(rename = "document")]
            Media(&'a InputMediaDocument),
        }
        Tagged::Media(value).serialize(serializer)
    }
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<InputMediaDocument, D::Error> {
        #[derive(Deserialize)]
        #[serde(tag = "type")]
        enum Tagged {
            #[serde(rename = "document")]
            Media(InputMediaDocument),
        }
        let Tagged::Media(value) = Tagged::deserialize(deserializer)?;
        Ok(value)
    }
}

mod input_media_photo_tagged {
    #[cfg(test)]
    #[derive(serde::Deserialize, schemars::JsonSchema)]
    #[serde(tag = "type")]
    #[allow(dead_code)]
    pub enum Schema {
        #[serde(rename = "photo")]
        Media(crate::types::InputMediaPhoto),
    }

    use super::*;
    pub fn serialize<S: serde::Serializer>(
        value: &InputMediaPhoto,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(tag = "type")]
        enum Tagged<'a> {
            #[serde(rename = "photo")]
            Media(&'a InputMediaPhoto),
        }
        Tagged::Media(value).serialize(serializer)
    }
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<InputMediaPhoto, D::Error> {
        #[derive(Deserialize)]
        #[serde(tag = "type")]
        enum Tagged {
            #[serde(rename = "photo")]
            Media(InputMediaPhoto),
        }
        let Tagged::Media(value) = Tagged::deserialize(deserializer)?;
        Ok(value)
    }
}

mod input_media_video_tagged {
    #[cfg(test)]
    #[derive(serde::Deserialize, schemars::JsonSchema)]
    #[serde(tag = "type")]
    #[allow(dead_code)]
    pub enum Schema {
        #[serde(rename = "video")]
        Media(crate::types::InputMediaVideo),
    }

    use super::*;
    pub fn serialize<S: serde::Serializer>(
        value: &InputMediaVideo,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(tag = "type")]
        enum Tagged<'a> {
            #[serde(rename = "video")]
            Media(&'a InputMediaVideo),
        }
        Tagged::Media(value).serialize(serializer)
    }
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<InputMediaVideo, D::Error> {
        #[derive(Deserialize)]
        #[serde(tag = "type")]
        enum Tagged {
            #[serde(rename = "video")]
            Media(InputMediaVideo),
        }
        let Tagged::Media(value) = Tagged::deserialize(deserializer)?;
        Ok(value)
    }
}

mod input_media_voice_note_tagged {
    #[cfg(test)]
    #[derive(serde::Deserialize, schemars::JsonSchema)]
    #[serde(tag = "type")]
    #[allow(dead_code)]
    pub enum Schema {
        #[serde(rename = "voice_note")]
        Media(crate::types::InputMediaVoiceNote),
    }

    use super::*;
    pub fn serialize<S: serde::Serializer>(
        value: &InputMediaVoiceNote,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(tag = "type")]
        enum Tagged<'a> {
            #[serde(rename = "voice_note")]
            Media(&'a InputMediaVoiceNote),
        }
        Tagged::Media(value).serialize(serializer)
    }
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<InputMediaVoiceNote, D::Error> {
        #[derive(Deserialize)]
        #[serde(tag = "type")]
        enum Tagged {
            #[serde(rename = "voice_note")]
            Media(InputMediaVoiceNote),
        }
        let Tagged::Media(value) = Tagged::deserialize(deserializer)?;
        Ok(value)
    }
}
