use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::types::User;

/// Returned only in [`GetMe`].
///
/// [`GetMe`]: crate::payloads::GetMe
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Me {
    /// Optional. True, if the bot supports guest queries from chats it is not a
    /// member of. Returned only in getMe.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub supports_guest_queries: bool,

    /// Optional. True, if the bot has forum topic mode enabled in private
    /// chats. Returned only in getMe.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_topics_enabled: bool,

    /// Optional. True, if the bot allows users to create and delete topics in
    /// private chats. Returned only in getMe.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub allows_users_to_create_topics: bool,

    /// Optional. True, if other bots can be created to be controlled by the
    /// bot. Returned only in getMe.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub can_manage_bots: bool,

    /// Optional. True, if the bot supports join request queries and can be
    /// assigned to process them. Returned only in getMe.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub supports_join_request_queries: bool,

    #[serde(flatten)]
    pub user: User,

    /// `true`, if the bot can be invited to groups.
    pub can_join_groups: bool,

    /// `true`, if [privacy mode] is disabled for the bot.
    ///
    /// [privacy mode]: https://core.telegram.org/bots#privacy-mode
    pub can_read_all_group_messages: bool,

    /// `true`, if the bot supports inline queries.
    pub supports_inline_queries: bool,

    /// `true`, if the bot can be connected to a Telegram Business account to
    /// receive its messages.
    #[serde(default)]
    pub can_connect_to_business: bool,

    /// `true`, if the bot has a main Web App.
    pub has_main_web_app: bool,
}

impl Me {
    /// Returns the active username reported by Telegram, if present.
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        self.user.username.as_deref()
    }

    /// Returns a username mention of this bot.
    #[must_use]
    pub fn mention(&self) -> Option<String> {
        self.user.mention()
    }

    /// Returns an URL that links to this bot in the form of `t.me/<...>`.
    #[must_use]
    pub fn tme_url(&self) -> Option<reqwest::Url> {
        self.user.tme_url()
    }
}

impl Deref for Me {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.user
    }
}

impl Me {
    /// Sets `supports_guest_queries`.
    pub fn supports_guest_queries(mut self, value: bool) -> Self {
        self.supports_guest_queries = value;
        self
    }
    /// Sets `has_topics_enabled`.
    pub fn has_topics_enabled(mut self, value: bool) -> Self {
        self.has_topics_enabled = value;
        self
    }
    /// Sets `allows_users_to_create_topics`.
    pub fn allows_users_to_create_topics(mut self, value: bool) -> Self {
        self.allows_users_to_create_topics = value;
        self
    }
    /// Sets `can_manage_bots`.
    pub fn can_manage_bots(mut self, value: bool) -> Self {
        self.can_manage_bots = value;
        self
    }
    /// Sets `supports_join_request_queries`.
    pub fn supports_join_request_queries(mut self, value: bool) -> Self {
        self.supports_join_request_queries = value;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{Me, User, UserId};

    #[test]
    fn convenience_methods_work() {
        let me = Me {
            supports_guest_queries: false,
            has_topics_enabled: false,
            allows_users_to_create_topics: false,
            can_manage_bots: false,
            supports_join_request_queries: false,
            user: User {
                id: UserId(42),
                is_bot: true,
                first_name: "First".to_owned(),
                last_name: None,
                username: Some("SomethingSomethingBot".to_owned()),
                language_code: None,
                is_premium: false,
                added_to_attachment_menu: false,
            },
            can_join_groups: false,
            can_read_all_group_messages: false,
            supports_inline_queries: false,
            can_connect_to_business: false,
            has_main_web_app: false,
        };

        assert_eq!(me.username(), Some("SomethingSomethingBot"));
        assert_eq!(me.mention(), Some("@SomethingSomethingBot".into()));
        assert_eq!(me.tme_url(), Some("https://t.me/SomethingSomethingBot".parse().unwrap()));

        let mut without_username = me;
        without_username.user.username = None;
        assert_eq!(without_username.username(), None);
        assert_eq!(without_username.mention(), None);
        assert_eq!(without_username.tme_url(), None);
    }
}
