use futures_util::TryStreamExt;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use tokio::io::AsyncBufReadExt;
use tokio_util::io::StreamReader;

pub struct Client {
    token: String,
    url: String,
    client: reqwest::Client,
}

#[derive(Default, Serialize, Deserialize, Debug)]
//For proper serialization
#[allow(non_camel_case_types)]
pub enum Visibility {
    #[default]
    public,
    unlisted,
    private,
    direct,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Poll {
    pub options: Vec<String>,
    pub multiple: bool,
    pub expires_in: i32,
    pub hide_totals: bool,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Post {
    pub status: String,
    pub media_ids: Vec<String>,
    pub poll: Option<Poll>,
    pub in_reply_to_id: Option<String>,
    pub sensitive: bool,
    pub spoiler_text: Option<String>,
    pub visibility: Option<Visibility>,
    pub language: Option<String>,
    pub scheduled_at: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct CustomEmoji {
    pub shortcode: String,
    pub url: String,
    pub static_url: String,
    pub visible_in_picker: bool,
    pub category: Option<String>,
}
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Field {
    pub name: String,
    pub value: String,
    pub verified_at: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: String,
    pub username: String,
    pub acct: String,
    pub url: String,
    pub uri: String,
    pub display_name: String,
    pub note: String,
    pub avatar: String,
    pub avatar_static: String,
    pub header: String,
    pub locked: bool,
    pub fields: Vec<Field>,
    pub emojis: Vec<CustomEmoji>,
    pub bot: bool,
    pub group: Option<bool>,
    pub discoverable: Option<bool>,
    pub noindex: Option<bool>,
    pub moved: Option<Box<Account>>,
    pub suspended: Option<bool>,
    pub limited: Option<bool>,
    pub created_at: String,
    pub last_status_at: Option<String>,
    pub statuses_count: i32,
    pub followers_count: i32,
    pub following_count: i32,
    pub hide_collections: Option<bool>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
//For proper serialization
#[allow(non_camel_case_types)]
pub enum MediaType {
    #[default]
    unknown,
    image,
    gifv,
    video,
    audio,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Focus {
    pub x: f32,
    pub y: f32,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Meta {
    pub focus: Option<Focus>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct MediaAttachment {
    pub id: String,
    pub r#type: MediaType,
    pub url: String,
    pub preview_url: Option<String>,
    pub remote_url: Option<String>,
    pub meta: Meta,
    pub description: Option<String>,
    pub blurhash: Option<String>,
    //This was removed but probably good to put this in here
    pub text_rul: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Application {
    pub name: String,
    pub website: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Mention {
    pub id: String,
    pub username: String,
    pub url: String,
    pub accr: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Tag {
    pub name: String,
    pub url: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct PreviewCardAuthor {
    pub name: String,
    pub url: String,
    pub account: Option<Account>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum PreviewCardType {
    #[default]
    link,
    photo,
    video,
    rich,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct PreviewCard {
    pub url: String,
    pub title: String,
    pub desctiption: String,
    pub r#type: PreviewCardType,
    pub authors: Vec<PreviewCardAuthor>,
    //Deprecated in 4.3.0
    pub author_name: Option<String>,
    //Deprecated in 4.3.0
    pub author_url: Option<String>,
    pub provider_name: String,
    pub provider_url: String,
    pub html: String,
    pub width: i32,
    pub height: i32,
    pub image: Option<String>,
    pub embed_url: String,
    pub blurhash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum QuoteState {
    pending,
    accepted,
    rejected,
    revoked,
    deleted,
    unauthorized,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum Quote {
    Quote {
        state: QuoteState,
        status: Option<Box<Status>>,
    },
    ShallowQuote {
        state: QuoteState,
        status_id: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum FilterContext {
    home,
    notifications,
    public,
    thread,
    account,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum FilterAction {
    #[default]
    warn,
    hide,
    blur,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct FilterKeyword {
    pub id: String,
    pub keyword: String,
    pub whole_word: bool,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct FilterStatus {
    pub id: String,
    pub status_id: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Filter {
    pub id: String,
    pub title: String,
    pub context: Vec<FilterContext>,
    pub expires_at: Option<String>,
    pub filter_action: FilterAction,
    pub keywords: Option<Vec<FilterKeyword>>,
    pub statuses: Option<Vec<FilterStatus>>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct FilterResult {
    pub filter: Filter,
    pub keyword_matches: Option<Vec<String>>,
    pub status_matches: Option<Vec<String>>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct StatusPollOption {
    pub title: String,
    pub votes_count: Option<i32>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct StatusPoll {
    pub id: String,
    pub expires_at: Option<String>,
    pub expired: bool,
    pub multiple: bool,
    pub votes_count: i32,
    pub voters_count: Option<i32>,
    pub options: Vec<StatusPollOption>,
    pub emojis: Vec<CustomEmoji>,
    pub voted: Option<bool>,
    pub own_votes: Option<Vec<i32>>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Status {
    pub id: String,
    pub uri: String,
    pub created_at: String,
    pub account: Account,
    pub content: String,
    pub visibility: Visibility,
    pub sensetive: Option<bool>,
    pub spoiler_text: String,
    pub media_attachments: Vec<MediaAttachment>,
    pub application: Option<Application>,
    pub mentions: Vec<Mention>,
    pub tags: Vec<Tag>,
    pub emojis: Vec<CustomEmoji>,
    pub reblog_count: Option<i32>,
    pub favourites_count: i32,
    pub replied_count: Option<i32>,
    pub url: Option<String>,
    pub in_reply_to_id: Option<String>,
    pub in_reply_to_account_id: Option<String>,
    pub reblog: Option<Box<Status>>,
    pub poll: Option<StatusPoll>,
    pub card: Option<PreviewCard>,
    pub language: Option<String>,
    pub text: Option<String>,
    pub edited_at: Option<String>,
    pub quote: Option<Quote>,
    pub favourited: Option<bool>,
    pub reblogged: Option<bool>,
    pub muted: Option<bool>,
    pub bookmarked: Option<bool>,
    pub pinned: Option<bool>,
    pub filtered: Option<Vec<FilterResult>>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
//For proper serialization
#[allow(non_camel_case_types)]
pub enum ReportCategory {
    spam,
    violation,
    #[default]
    other,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Report {
    pub id: String,
    pub action_taken: bool,
    pub action_taken_at: Option<String>,
    pub category: ReportCategory,
    pub comment: String,
    pub forwarded: bool,
    pub created_at: String,
    pub status_ids: Option<Vec<String>>,
    pub rule_ids: Option<Vec<String>>,
    pub target_account: Account,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq)]
//For proper serialization
#[allow(non_camel_case_types)]
pub enum NotificationType {
    #[default]
    mention,
    status,
    reblog,
    reaction,
    follow,
    follow_request,
    favourite,
    poll,
    update,
    severed_relationships,
    moderation_warning,
}
#[derive(Default, Serialize, Deserialize, Debug)]
//For proper serialization
#[allow(non_camel_case_types)]
pub enum RelationShipSeveranceEventType {
    #[default]
    domaint_block,
    user_domain_block,
    account_suspension,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct RelationshipSeveranceEvent {
    pub id: String,
    pub r#type: RelationShipSeveranceEventType,
    pub purged: bool,
    pub target_name: String,
    pub followers_count: i32,
    pub following_couint: i32,
    pub created_at: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
//For proper serialization
#[allow(non_camel_case_types)]
pub enum AppealState {
    approved,
    rejected,
    #[default]
    pending,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Appeal {
    pub test: String,
    pub state: AppealState,
}

#[derive(Default, Serialize, Deserialize, Debug)]
//For proper serialization
#[allow(non_camel_case_types)]
pub enum AccountWarningAction {
    #[default]
    none,
    disable,
    mark_statuses_as_sensitive,
    delete_statuses,
    silence,
    suspend,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AccountWarning {
    pub id: String,
    pub action: AccountWarningAction,
    pub text: String,
    pub status_id: Option<Vec<String>>,
    pub target_account: Account,
    pub appeal: Option<Appeal>,
    pub created_at: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Notification {
    pub id: String,
    pub r#type: NotificationType,
    pub group_key: Option<String>,
    pub created_at: String,
    pub account: Account,
    pub status: Option<Status>,
    pub report: Option<Report>,
    pub event: Option<RelationshipSeveranceEvent>,
    pub moderation_warning: Option<AccountWarning>,
}

impl Client {
    pub fn new(token: String, url: String) -> Self {
        let client = reqwest::Client::new();
        Self { token, url, client }
    }

    pub async fn create_post(&self, post: Post) -> Result<(), String> {
        let request = self
            .client
            .post(self.url.clone() + "/api/v1/statuses")
            .bearer_auth(self.token.clone())
            .json(&post);

        match request.send().await.unwrap().error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn me(&self) -> Result<String, String> {
        let request = self
            .client
            .get(self.url.clone() + "/api/v1/accounts/verify_credentials")
            .bearer_auth(self.token.clone());

        match request.send().await.unwrap().error_for_status() {
            Ok(r) => {
                let text = r.text().await.unwrap();
                Ok(text)
            }
            Err(e) => Err(e.to_string()),
        }
    }

    //NOT SUPPORTED BY SHARKEY
    pub async fn stream_notifications(&self) {
        let request = self
            .client
            .get(self.url.clone() + "/api/v1/streaming/user/notification")
            .bearer_auth(self.token.clone());

        let response = request.send().await.unwrap();

        let stream = StreamReader::new(response.bytes_stream().map_err(|e| {
            println!("{e}");
            std::io::Error::new(std::io::ErrorKind::BrokenPipe, format!("{e}"))
        }));

        let mut stream = stream.lines();

        for _ in 0..4 {
            let l = stream.next_line().await; //.unwrap().unwrap();
            println!("Got an item");
            if let Err(e) = l {
                println!("ERROR {e}");
            } else {
                let l = l.unwrap().unwrap();

                println!("Got line {l}");
            }
        }
    }

    pub async fn dismiss_all_notification(&self) {
        let request = self
            .client
            .post(self.url.clone() + "/api/v1/notifications/clear")
            .bearer_auth(self.token.clone());

        let r = request.send().await.unwrap().text().await.unwrap();

        println!("DISMIS: {r}");
    }

    pub async fn dismis_notification(&self, id: String) {
        let request = self
            .client
            .post(self.url.clone() + &format!("/api/v1/notifications/dismiss?id={id}"))
            .bearer_auth(self.token.clone());

        let r = request.send().await.unwrap().text().await.unwrap();

        println!("DISMIS: {r}");
    }

    pub async fn get_notifications(&self) -> Vec<Notification> {
        let request = self
            .client
            .get(self.url.clone() + "/api/v1/notifications")
            .bearer_auth(self.token.clone());

        let response = request.send().await.unwrap().text().await.unwrap();

        from_str(&response).unwrap()
    }
}
