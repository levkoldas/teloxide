use crate::types::{FileId, FileUniqueId, PhotoSize};
use serde::{Deserialize, Serialize};

/// This object represents a live photo.
///
/// [The official docs](https://core.telegram.org/bots/api#livephoto).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct LivePhoto {
    /// Optional. Available sizes of the corresponding static photo
    pub photo: Option<Vec<PhotoSize>>,
    /// Identifier for the video file which can be used to download or reuse the
    /// file
    pub file_id: FileId,
    /// Unique identifier for the video file which is supposed to be the same
    /// over time and for different bots. Can't be used to download or reuse
    /// the file.
    pub file_unique_id: FileUniqueId,
    /// Video width as defined by the sender
    pub width: u32,
    /// Video height as defined by the sender
    pub height: u32,
    /// Duration of the video in seconds as defined by the sender
    pub duration: u32,
    /// Optional. MIME type of the file as defined by the sender
    pub mime_type: Option<String>,
    /// Optional. File size in bytes. It can be bigger than 2^31 and some
    /// programming languages may have difficulty/silent defects in
    /// interpreting it. But it has at most 52 significant bits, so a signed
    /// 64-bit integer or double-precision float type are safe for storing this
    /// value.
    pub file_size: Option<u64>,
}

impl LivePhoto {
    pub fn new(
        file_id: FileId,
        file_unique_id: FileUniqueId,
        width: u32,
        height: u32,
        duration: u32,
    ) -> Self {
        Self {
            photo: None,
            file_id,
            file_unique_id,
            width,
            height,
            duration,
            mime_type: None,
            file_size: None,
        }
    }
    pub fn photo(mut self, value: Vec<PhotoSize>) -> Self {
        self.photo = Some(value);
        self
    }
    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }
    pub fn file_size(mut self, value: u64) -> Self {
        self.file_size = Some(value);
        self
    }
}
