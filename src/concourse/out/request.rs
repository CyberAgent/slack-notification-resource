use slack_request::PostMessageRequest;
use concourse::Source;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub attachments: String,
    pub channel: String,
    pub username: Option<String>,
    pub icon_url: Option<String>,
    pub icon_emoji: Option<String>,
    pub thread_ts: Option<String>,
    pub reply_broadcast: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub source: Source,
    pub params: Params,
}

// slack-rs api interface to fit
const DUMMY_TEXT: &str = "";

impl Request {
    pub fn token(&self) -> String {
        self.source.token.clone()
    }

    // @todo naming. with?
    pub fn to_request(&self) -> PostMessageRequest {
        PostMessageRequest {
            channel: &self.params.channel,
            text: DUMMY_TEXT,
            attachments: Some(self.params.attachments.as_str()),
            username: self.params.username.as_ref().map(String::as_str),
            icon_url: self.params.icon_url.as_ref().map(String::as_str),
            icon_emoji: self.params.icon_emoji.as_ref().map(String::as_str),
            thread_ts: self.params.thread_ts.as_ref().map(String::as_str),
            reply_broadcast: Some(self.params.reply_broadcast),
            ..PostMessageRequest::default()
        }
    }
}
