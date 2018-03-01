use slack_response::PostMessageResponse;
use concourse::{Version, Metadata, KVals};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    version: Version,
    metadata: Metadata,
}

impl Response {
    pub fn from(resp: PostMessageResponse) -> Response {
        debug!("{:?}", resp);
        let ver = Version { thread_ts: resp.ts };
        let meta: Metadata = vec![
            KVals {
                name: "channel".to_string(),
                value: resp.channel.unwrap_or_else(String::new),
            },
        ];
        Response {
            version: ver,
            metadata: meta,
        }
    }
}
