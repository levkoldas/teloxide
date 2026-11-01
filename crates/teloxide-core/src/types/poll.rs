use crate::types::{MessageEntity, PollType, Seconds, User};

use chrono::{DateTime, Utc};
use derive_more::derive::From;
use serde::{Deserialize, Serialize};

/// Unique poll identifier.
#[derive(
    Default,
    Clone,
    Debug,
    derive_more::Display,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    From
)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(transparent)]
#[from(&'static str, String)]
pub struct PollId(pub String);

/// This object contains information about a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#poll).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Poll {
    /// True, if the poll allows to change the chosen answer options
    #[serde(default)]
    pub allows_revoting: bool,

    /// True if voting is limited to users who have been members of the chat
    /// where the poll was originally sent for more than 24 hours
    #[serde(default)]
    pub members_only: bool,

    /// Optional. A list of two-letter ISO 3166-1 alpha-2 country codes
    /// indicating the countries from which users can vote in the poll. The
    /// country code “FT” is used for users with anonymous numbers. If
    /// omitted, then users from any country can participate in the poll.
    pub country_codes: Option<Vec<String>>,

    /// Optional. Array of 0-based identifiers of the correct answer options.
    /// Available only for polls in quiz mode which are closed or were sent
    /// (not forwarded) by the bot or to the private chat with the bot.
    pub correct_option_ids: Option<Vec<u32>>,

    /// Optional. Media added to the quiz explanation
    pub explanation_media: Option<crate::types::PollMedia>,

    /// Optional. Description of the poll; for polls inside the Message object
    /// only
    pub description: Option<String>,

    /// Optional. Special entities like usernames, URLs, bot commands, etc. that
    /// appear in the description
    pub description_entities: Option<Vec<crate::types::MessageEntity>>,

    /// Optional. Media added to the poll description; for polls inside the
    /// Message object only
    pub media: Option<crate::types::PollMedia>,

    /// Unique poll identifier.
    pub id: PollId,

    /// Poll question, 1-300 characters.
    pub question: String,

    /// Special entities that appear in the question. Currently, only custom
    /// emoji entities are allowed in poll questions
    pub question_entities: Option<Vec<MessageEntity>>,

    /// List of poll options.
    pub options: Vec<PollOption>,

    /// `true`, if the poll is closed.
    pub is_closed: bool,

    /// Total number of users that voted in the poll
    pub total_voter_count: u32,

    /// True, if the poll is anonymous
    pub is_anonymous: bool,

    /// Poll type, currently can be “regular” or “quiz”
    #[serde(rename = "type")]
    pub poll_type: PollType,

    /// True, if the poll allows multiple answers
    pub allows_multiple_answers: bool,

    /// Text that is shown when a user chooses an incorrect answer or taps on
    /// the lamp icon in a quiz-style poll, 0-200 characters.
    pub explanation: Option<String>,

    /// Special entities like usernames, URLs, bot commands, etc. that appear in
    /// the explanation.
    pub explanation_entities: Option<Vec<MessageEntity>>,

    /// Amount of time in seconds the poll will be active after creation.
    pub open_period: Option<Seconds>,

    /// Point in time when the poll will be automatically closed.
    #[serde(default, with = "crate::types::serde_opt_date_from_unix_timestamp")]
    #[cfg_attr(test, schemars(with = "Option<i64>"))]
    pub close_date: Option<DateTime<Utc>>,
}

/// This object contains information about one answer option in a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#polloption).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PollOption {
    /// Unique identifier of the option, persistent on option addition and
    /// deletion
    #[serde(default)]
    pub persistent_id: String,

    /// Optional. Media added to the poll option
    pub media: Option<crate::types::PollMedia>,

    /// Optional. User who added the option; omitted if the option wasn't added
    /// by a user after poll creation
    pub added_by_user: Option<crate::types::User>,

    /// Optional. Chat that added the option; omitted if the option wasn't added
    /// by a chat after poll creation
    pub added_by_chat: Option<crate::types::Chat>,

    /// Optional. Point in time (Unix timestamp) when the option was added;
    /// omitted if the option existed in the original poll
    #[serde(default, with = "crate::types::serde_opt_date_from_unix_timestamp")]
    #[cfg_attr(test, schemars(with = "Option<i64>"))]
    pub addition_date: Option<chrono::DateTime<chrono::Utc>>,

    /// Option text, 1-100 characters.
    pub text: String,

    /// Special entities that appear in the option text. Currently, only custom
    /// emoji entities are allowed in poll option texts
    pub text_entities: Option<Vec<MessageEntity>>,

    /// Number of users that voted for this option.
    pub voter_count: u32,
}

impl Poll {
    /// Returns all users that are "contained" in this `Poll`
    /// structure.
    ///
    /// This might be useful to track information about users.
    ///
    /// Note that this function can return duplicate users.
    pub fn mentioned_users(&self) -> impl Iterator<Item = &User> {
        use crate::util::{flatten, mentioned_users_from_entities};

        flatten(self.explanation_entities.as_deref().map(mentioned_users_from_entities))
            .chain(flatten(self.question_entities.as_deref().map(mentioned_users_from_entities)))
            .chain(flatten(self.description_entities.as_deref().map(mentioned_users_from_entities)))
            .chain(self.options.iter().flat_map(|option| {
                option.added_by_user.iter().chain(mentioned_users_from_entities(
                    option.text_entities.as_deref().unwrap_or_default(),
                ))
            }))
    }
}

impl Poll {
    /// Sets `allows_revoting`.
    pub fn allows_revoting(mut self, value: bool) -> Self {
        self.allows_revoting = value;
        self
    }
    /// Sets `members_only`.
    pub fn members_only(mut self, value: bool) -> Self {
        self.members_only = value;
        self
    }
    /// Sets `country_codes`.
    pub fn country_codes(mut self, value: Vec<String>) -> Self {
        self.country_codes = Some(value);
        self
    }
    /// Sets `correct_option_ids`.
    pub fn correct_option_ids(mut self, value: Vec<u32>) -> Self {
        self.correct_option_ids = Some(value);
        self
    }
    /// Sets `explanation_media`.
    pub fn explanation_media(mut self, value: crate::types::PollMedia) -> Self {
        self.explanation_media = Some(value);
        self
    }
    /// Sets `description`.
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    /// Sets `description_entities`.
    pub fn description_entities(mut self, value: Vec<crate::types::MessageEntity>) -> Self {
        self.description_entities = Some(value);
        self
    }
    /// Sets `media`.
    pub fn media(mut self, value: crate::types::PollMedia) -> Self {
        self.media = Some(value);
        self
    }
}

impl PollOption {
    /// Sets `persistent_id`.
    pub fn persistent_id(mut self, value: String) -> Self {
        self.persistent_id = value;
        self
    }
    /// Sets `media`.
    pub fn media(mut self, value: crate::types::PollMedia) -> Self {
        self.media = Some(value);
        self
    }
    /// Sets `added_by_user`.
    pub fn added_by_user(mut self, value: crate::types::User) -> Self {
        self.added_by_user = Some(value);
        self
    }
    /// Sets `added_by_chat`.
    pub fn added_by_chat(mut self, value: crate::types::Chat) -> Self {
        self.added_by_chat = Some(value);
        self
    }
    /// Sets `addition_date`.
    pub fn addition_date(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.addition_date = Some(value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"
        {
            "allows_multiple_answers": false,
            "id": "5377643193141559299",
            "is_anonymous": true,
            "is_closed": false,
            "options": [
                {
                    "text": "1",
                    "voter_count": 1
                },
                {
                    "text": "2",
                    "voter_count": 0
                },
                {
                    "text": "3",
                    "voter_count": 0
                },
                {
                    "text": "4",
                    "voter_count": 0
                },
                {
                    "text": "5",
                    "voter_count": 0
                }
            ],
            "question": "Rate me from 1 to 5.",
            "total_voter_count": 1,
            "type": "regular"
        }
        "#;
        serde_json::from_str::<Poll>(data).unwrap();
    }
}
