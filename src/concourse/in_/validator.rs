use chrono::prelude::*;

use error::Result;
use core::*;
use concourse::in_::args::Args;

pub struct ArgsValidator<'a>(&'a Args);

impl<'a> ArgsValidator<'a> {
    // @todo more generic
    pub fn from_ref(args: &'a Args) -> ArgsValidator<'a> {
        ArgsValidator(args)
    }

    fn valid_thread_ts(&self) -> Result<()> {
        let r = match self.0.thread_ts {
            Some(ref ts) => UnixTimestamp::try_from(ts),
            None => Ok(Utc::now()),
        };
        r.map(|_| ())
    }

    fn valid_token(&self) -> Result<()> {
        let token = self.0.token.as_str();
        match token.find("https://") {
            Some(_) => Err(format_err!(
                "Invalid token format\n Ref https://api.slack.com/apps. \ntoken: {}",
                token
            )),
            None => Ok(()),
        }
    }
}

impl<'a> Validator for ArgsValidator<'a> {
    // @todo more better way
    fn validate(&self) -> Result<()> {
        self.valid_thread_ts()?;
        self.valid_token()
    }
}
