use crate::types::{
    Animation, Audio, Document, Link, LivePhoto, Location, PhotoSize, Sticker, Venue, Video,
};
use serde::{Deserialize, Serialize};

/// Media attached to a poll, quiz explanation or poll option.
/// At most one media kind can be present.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PollMedia {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub kind: Option<PollMediaKind>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum PollMediaKind {
    Animation(Box<Animation>),
    Audio(Box<Audio>),
    Document(Box<Document>),
    Link(Box<Link>),
    LivePhoto(Box<LivePhoto>),
    Location(Box<Location>),
    Photo(Vec<PhotoSize>),
    Sticker(Box<Sticker>),
    Venue(Box<Venue>),
    Video(Box<Video>),
}
impl PollMedia {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn animation(mut self, value: Animation) -> Self {
        self.kind = Some(PollMediaKind::Animation(Box::new(value)));
        self
    }
    pub fn audio(mut self, value: Audio) -> Self {
        self.kind = Some(PollMediaKind::Audio(Box::new(value)));
        self
    }
    pub fn document(mut self, value: Document) -> Self {
        self.kind = Some(PollMediaKind::Document(Box::new(value)));
        self
    }
    pub fn link(mut self, value: Link) -> Self {
        self.kind = Some(PollMediaKind::Link(Box::new(value)));
        self
    }
    pub fn live_photo(mut self, value: LivePhoto) -> Self {
        self.kind = Some(PollMediaKind::LivePhoto(Box::new(value)));
        self
    }
    pub fn location(mut self, value: Location) -> Self {
        self.kind = Some(PollMediaKind::Location(Box::new(value)));
        self
    }
    pub fn photo(mut self, value: Vec<PhotoSize>) -> Self {
        self.kind = Some(PollMediaKind::Photo(value));
        self
    }
    pub fn sticker(mut self, value: Sticker) -> Self {
        self.kind = Some(PollMediaKind::Sticker(Box::new(value)));
        self
    }
    pub fn venue(mut self, value: Venue) -> Self {
        self.kind = Some(PollMediaKind::Venue(Box::new(value)));
        self
    }
    pub fn video(mut self, value: Video) -> Self {
        self.kind = Some(PollMediaKind::Video(Box::new(value)));
        self
    }
}
