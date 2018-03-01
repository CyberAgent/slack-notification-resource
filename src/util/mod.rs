extern crate core;

use error::Result;
use std::env;
use env_logger::{LogBuilder, LogTarget};
use std::fmt::Debug;
use regex::Regex;
use chrono::prelude::*;
use chrono::*;
use std::result::Result as StdResult;
use failure::Error;
use core::*;

pub fn logger_init() -> Result<()> {
    let mut builder = LogBuilder::new();
    builder.target(LogTarget::Stderr);

    if env::var("RUST_LOG").is_ok() {
        builder.parse(&env::var("RUST_LOG")?);
    }

    builder.init()?;
    Ok(())
}

pub fn pairs_extract<I>(s: &str, pairs: I) -> Result<String>
where
    I: IntoIterator<Item = (String, String)> + Debug,
{
    pairs.into_iter().fold(Ok(s.to_string()), |acc, (k, v)| {
        // @todo performance??
        acc.and_then(|a| {
            let re = Regex::new(format!(r"(\$\{{?{}\}}?)", &k).as_str())?;
            let r = re.replace_all(&a, v.as_str()).to_string();
            Ok(r)
        })
    })
}

pub fn args_extend<I>(iter: I) -> Vec<String>
where
    I: IntoIterator<Item = String> + Debug,
{
    let mut args = env::args().collect::<Vec<String>>();
    args.extend(iter.into_iter());
    args
}

impl<'a> TryFrom<&'a str> for UnixTimestamp {
    type Error = Error;

    // @todo simple
    fn try_from(value: &'a str) -> StdResult<Self, Self::Error> {
        let tss: Vec<&str> = value.split_terminator('.').collect();
        let ts_opt = match (tss.get(0), tss.get(1)) {
            (Some(sec), Some(mills)) => Ok((sec, mills)),
            _ => Err(format_err!("{}", "Failure on parts of thread_ts")),
        };
        ts_opt
            .and_then(|(sec, mills)| {
                debug!("sec:{}, mills: {}", sec, mills);
                let sec_result = sec.parse::<i64>();
                let mills_result = mills.parse::<u32>();
                match (sec_result, mills_result) {
                    (Ok(s), Ok(m)) => Ok((s, m)),
                    _ => Err(format_err!("{}", "Failure conversion of thread_ts")),
                }
            })
            .and_then(|(time, ms)| match Utc.timestamp_opt(time, ms) {
                LocalResult::Single(s) => Ok(s),
                _ => Err(format_err!("{}", "Failure to convert thread_ts")),
            })
    }
}

#[cfg(test)]
mod tests {}
