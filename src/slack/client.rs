use slack_api as slack;
use slack::request::PostMessageRequest;
use slack::response::PostMessageResponse;
use error::Result;
use serde_json;
use std::result;

pub struct Client {
    token: String,
    client: slack::requests::Client,
}

pub fn new(token: &str) -> Client {
    Client {
        token: token.to_string(),
        client: slack::requests::default_client().unwrap(),
    }
}

impl Client {
    pub fn post_message(&self, req: &PostMessageRequest) -> Result<PostMessageResponse> {
        let r = self._post_message(&self.client, self.token.as_ref(), req)?;
        Ok(r)
    }
    fn _post_message<R>(
        &self,
        client: &R,
        token: &str,
        request: &PostMessageRequest,
    ) -> result::Result<PostMessageResponse, slack::chat::PostMessageError<R::Error>>
    where
        R: slack::requests::SlackWebRequestSender,
    {
        let params = vec![
            Some(("token", token)),
            Some(("channel", request.channel)),
            Some(("text", request.text)),
            request.parse.map(|parse| ("parse", parse)),
            request
                .link_names
                .map(|link_names| ("link_names", if link_names { "1" } else { "0" })),
            request
                .attachments
                .map(|attachments| ("attachments", attachments)),
            request
                .unfurl_links
                .map(|unfurl_links| ("unfurl_links", if unfurl_links { "1" } else { "0" })),
            request
                .unfurl_media
                .map(|unfurl_media| ("unfurl_media", if unfurl_media { "1" } else { "0" })),
            request.username.map(|username| ("username", username)),
            request
                .as_user
                .map(|as_user| ("as_user", if as_user { "1" } else { "0" })),
            request.icon_url.map(|icon_url| ("icon_url", icon_url)),
            request
                .icon_emoji
                .map(|icon_emoji| ("icon_emoji", icon_emoji)),
            request.thread_ts.map(|thread_ts| ("thread_ts", thread_ts)),
            request.reply_broadcast.map(|reply_broadcast| {
                ("reply_broadcast", if reply_broadcast { "1" } else { "0" })
            }),
        ];
        let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
        let url = self.get_slack_url_for_method("chat.postMessage");
        client
            .send(&url, &params[..])
            .map_err(slack::chat::PostMessageError::Client)
            .and_then(|result| {
                serde_json::from_str::<PostMessageResponse>(&result)
                    .map_err(slack::chat::PostMessageError::MalformedResponse)
            })
            .and_then(|o| o.into())
    }

    fn get_slack_url_for_method(&self, method: &str) -> String {
        format!("https://slack.com/api/{}", method)
    }
}
