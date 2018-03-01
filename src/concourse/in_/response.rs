use concourse::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Response {
    pub version: Version,
    pub metadata: Metadata,
}
