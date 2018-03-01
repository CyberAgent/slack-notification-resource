use serde_json;

use error::Result;
use concourse::*;
use concourse::check::response::Response;

pub fn run() -> Result<String> {
    let check = vec![Version::new()] as Response;
    let res = serde_json::to_string(&check)?;
    Ok(res)
}
