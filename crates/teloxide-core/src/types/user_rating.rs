use serde::{Deserialize, Serialize};

/// This object describes the rating of a user based on their Telegram Star
/// spendings.
///
/// [The official docs](https://core.telegram.org/bots/api#userrating).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UserRating {
    /// Current level of the user, indicating their reliability when purchasing
    /// digital goods and services. A higher level suggests a more
    /// trustworthy customer; a negative level is likely reason for concern.
    pub level: i64,
    /// Numerical value of the user's rating; the higher the rating, the better
    pub rating: i64,
    /// The rating value required to get the current level
    pub current_level_rating: i64,
    /// Optional. The rating value required to get to the next level; omitted if
    /// the maximum level was reached
    pub next_level_rating: Option<i64>,
}

impl UserRating {
    pub fn new(level: i64, rating: i64, current_level_rating: i64) -> Self {
        Self { level, rating, current_level_rating, next_level_rating: None }
    }
    pub fn next_level_rating(mut self, value: i64) -> Self {
        self.next_level_rating = Some(value);
        self
    }
}
