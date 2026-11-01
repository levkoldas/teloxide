use crate::types::Audio;
use serde::{Deserialize, Serialize};

/// This object represents the audios displayed on a user's profile.
///
/// [The official docs](https://core.telegram.org/bots/api#userprofileaudios).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UserProfileAudios {
    /// Total number of profile audios for the target user
    pub total_count: u32,
    /// Requested profile audios
    pub audios: Vec<Audio>,
}

impl UserProfileAudios {
    pub fn new(total_count: u32, audios: Vec<Audio>) -> Self {
        Self { total_count, audios }
    }
}
