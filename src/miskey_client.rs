use serde::Serialize;
use std::collections::HashMap;

pub struct Client {
    token: String,
    url: String,
    client: reqwest::Client,
}

#[derive(Default, Serialize, Debug)]
//For proper serialization
#[allow(non_camel_case_types)]
pub enum Visibility {
    #[default]
    public,
    home,
    followers,
    specified,
}

#[derive(Default, Serialize, Debug)]
//For proper serialization
#[allow(non_camel_case_types)]
pub enum ReactionAcceptance {
    #[default]
    null,
    likeOnly,
    nonSenseitiveOnly,
    nonSensitiveOnlyForLocalLikeOnlyForRemote,
}

#[derive(Default, Serialize, Debug)]
pub struct Poll {
    pub choices: Vec<String>,
    pub multiple: bool,
    pub expires_at: Option<i64>,
    pub expired_after: Option<i64>,
}

#[derive(Default, Serialize, Debug)]
pub struct Note {
    pub visibility: Visibility,
    pub visible_user_ids: Vec<String>,
    pub cw: Option<String>,
    pub local_only: bool,
    pub reaction_acceptance: ReactionAcceptance,
    pub no_extract_mentions: bool,
    pub no_extract_hashtags: bool,
    pub no_extract_emojis: bool,
    pub reply_id: Option<String>,
    pub renote_id: Option<String>,
    pub channel_id: Option<String>,
    pub text: Option<String>,
    pub file_ids: Option<String>,
    pub media_ids: Option<String>,
    pub poll: Option<Poll>,
}

impl Client {
    pub fn new(token: String, url: String) -> Self {
        let client = reqwest::Client::new();
        Self { token, url, client }
    }

    pub async fn create_note(&self, note: Note) -> Result<(), String> {
        let request = self
            .client
            .post(self.url.clone() + "/api/notes/create")
            .bearer_auth(self.token.clone())
            .json(&note);

        match request.send().await.unwrap().error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn me(&self) -> Result<String, String> {
        let request = self
            .client
            .post(self.url.clone() + "/api/i")
            .bearer_auth(self.token.clone())
            .json(&HashMap::<i32, i32>::new());

        match request.send().await.unwrap().error_for_status() {
            Ok(r) => {
                let text = r.text().await.unwrap();
                Ok(text)
            }
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn flush_notifications(&self) -> Result<(), String> {
        let request = self
            .client
            .post(self.url.clone() + "/api/notifications/flush")
            .bearer_auth(self.token.clone())
            .json(&HashMap::<i32, i32>::new());

        match request.send().await.unwrap().error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}
