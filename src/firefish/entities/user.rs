use super::Emoji;
use chrono::Utc;
use serde::Deserialize;

use crate::entities as MegalodonEntities;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: String,
    name: Option<String>,
    username: String,
    host: Option<String>,
    avatar_url: Option<String>,
    avatar_blurhash: Option<String>,
    avatar_color: Option<String>,
    is_admin: Option<bool>,
    is_moderator: Option<bool>,
    is_bot: Option<bool>,
    is_cat: Option<bool>,
    is_indexable: Option<bool>,
    speak_as_cat: Option<bool>,
    emojis: Vec<Emoji>,
    online_status: Option<String>,
}

impl Into<MegalodonEntities::Account> for User {
    fn into(self) -> MegalodonEntities::Account {
        let mut acct = self.username.clone();
        if let Some(host) = self.host {
            acct = format!("{}@{}", self.username, host);
        }
        let mut display_name = "".to_string();
        if let Some(name) = self.name {
            display_name = name;
        }
        let mut avatar = "".to_string();
        if let Some(avatar_url) = self.avatar_url {
            avatar = avatar_url;
        }
        let mut avatar_static = "".to_string();
        if let Some(avatar_color) = self.avatar_color {
            avatar_static = avatar_color;
        }
        let mut bot = false;
        if let Some(is_bot) = self.is_bot {
            bot = is_bot;
        }

        MegalodonEntities::Account {
            id: self.id,
            username: self.username,
            acct: acct.clone(),
            display_name,
            locked: false,
            discoverable: None,
            group: None,
            noindex: self.is_indexable,
            moved: None,
            suspended: None,
            limited: None,
            created_at: Utc::now(),
            followers_count: 0,
            following_count: 0,
            statuses_count: 0,
            note: "".to_string(),
            url: acct,
            avatar,
            avatar_static,
            header: "".to_string(),
            header_static: "".to_string(),
            emojis: self.emojis.into_iter().map(|e| e.into()).collect(),
            fields: [].to_vec(),
            bot,
            source: None,
            role: None,
            mute_expires_at: None,
        }
    }
}
