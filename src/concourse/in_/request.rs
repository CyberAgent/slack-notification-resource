use concourse::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub version: Version,
}
