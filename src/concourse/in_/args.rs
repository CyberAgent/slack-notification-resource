use std::fmt::Debug;
use std::result::Result as StdResult;
use structopt::StructOpt;
use failure::Error;


use core::*;
use error::Result;
use concourse::in_::request::*;
use concourse::Version;
use concourse::in_::validator::ArgsValidator;

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(name = "out")]
pub struct Args {
    #[structopt(long = "version.thread_ts")] pub thread_ts: Option<String>,
    #[structopt(long = "source.token")] pub token: String,
    #[structopt(name = "DIR", help = "destination dir")] pub dir: String,
}

impl From<Args> for Request {
    fn from(value: Args) -> Self {
        let version = Version {
            thread_ts: value.thread_ts,
        };
        Request { version }
    }
}

impl Validatable for Args {
    fn validate(&self) -> Result<()> {
        ArgsValidator::from_ref(self).validate()
    }
}

impl TryFromIterator<String> for Args {
    type Error = Error;

    fn try_from_iter<I>(iter: I) -> StdResult<Self, Self::Error>
    where
        I: IntoIterator<Item = String> + Debug,
    {
        let clap = Args::clap().get_matches_from_safe(iter.into_iter())?;
        let args = Args::from_clap(clap);
        match args.validate() {
            Ok(_) => Ok(args),
            Err(err) => bail!("Invalid option. args: {:?}, err: {}", args, err),
        }
    }
}
