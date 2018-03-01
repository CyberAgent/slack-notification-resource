use slack_api::chat::PostMessageError;
use std::error::Error;

#[derive(Clone, Debug, Deserialize)]
pub struct PostMessageResponse {
    pub channel: Option<String>,
    error: Option<String>,
    #[serde(default)] ok: bool,
    pub ts: Option<String>,
}

impl<E: Error> Into<Result<PostMessageResponse, PostMessageError<E>>> for PostMessageResponse {
    fn into(self) -> Result<PostMessageResponse, PostMessageError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
