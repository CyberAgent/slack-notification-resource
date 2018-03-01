extern crate chrono;
extern crate env_logger;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate mktemp;
extern crate regex;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate slack_api;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

pub mod util;
mod slack;
pub mod concourse;
pub mod error;
pub mod io;
pub mod core;

pub use slack::client as slack_client;
pub use slack::request as slack_request;
pub use slack::response as slack_response;
