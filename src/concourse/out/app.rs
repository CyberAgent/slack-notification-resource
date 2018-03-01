use std::fmt::Debug;
use serde_json;

use slack_client;
use error::Result;
use core::TryFromIterator;
use core::TryFrom;
use concourse::out::args::Args;
use concourse::out::request::Request;
use concourse::out::response::Response;

// @todo only reference
pub fn run<I>(row_args: I) -> Result<String>
    where
        I: IntoIterator<Item = String> + Debug,
{
    let args = Args::try_from_iter(row_args)?;
    let request = Request::try_from(args)?;

    let client = slack_client::new(&request.token());
    info!("params: {:?}", &request.params);
    let result = client.post_message(&request.to_request())?;
    let response = Response::from(result);

    let view = serde_json::to_string(&response)?;
    Ok(view)
}
