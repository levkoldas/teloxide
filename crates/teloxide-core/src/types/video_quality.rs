use crate::types::{FileId, FileUniqueId};
use serde::{Deserialize, Serialize};

/// This object represents a video file of a specific quality.
///
/// [The official docs](https://core.telegram.org/bots/api#videoquality).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct VideoQuality {
    /// Identifier for this file, which can be used to download or reuse the
    /// file
    pub file_id: FileId,
    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: FileUniqueId,
    /// Video width
    pub width: u32,
    /// Video height
    pub height: u32,
    /// Codec that was used to encode the video, for example, “h264”, “h265”, or
    /// “av01”
    pub codec: String,
    /// Optional. File size in bytes. It can be bigger than 2^31 and some
    /// programming languages may have difficulty/silent defects in
    /// interpreting it. But it has at most 52 significant bits, so a signed
    /// 64-bit integer or double-precision float type are safe for storing this
    /// value.
    pub file_size: Option<u64>,
}

impl VideoQuality {
    pub fn new(
        file_id: FileId,
        file_unique_id: FileUniqueId,
        width: u32,
        height: u32,
        codec: impl Into<String>,
    ) -> Self {
        Self { file_id, file_unique_id, width, height, codec: codec.into(), file_size: None }
    }
    pub fn file_size(mut self, value: u64) -> Self {
        self.file_size = Some(value);
        self
    }
}
