use chrono::prelude::*;

pub mod check;
pub mod in_;
pub mod out;

#[derive(Serialize, Deserialize, Debug)]
pub struct KVals {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Version {
    pub thread_ts: Option<String>,
}

impl Version {
    pub fn new() -> Version {
        let thread_ts = Some(Utc::now().format("%s%.f").to_string());
        Version { thread_ts }
    }
}

pub type Metadata = Vec<KVals>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    pub token: String,
}
