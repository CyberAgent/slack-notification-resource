use std::env;
use std::fmt::Debug;
use std::result::Result as StdResult;
use std::path::PathBuf;
use structopt::StructOpt;
use failure::Error;

use error::*;
use io::*;
use core::*;
use util::*;
use concourse::*;
use concourse::out::request::*;
use concourse::out::validator::*;

// @todo field scope
#[derive(StructOpt, PartialEq, Debug)]
#[structopt(name = "out")]
pub struct Args {
    #[structopt(long = "source.token", env = "SNOTIF_TOKEN")] pub token: String,
    #[structopt(long = "params.channel")] pub channel: Option<String>,
    #[structopt(long = "params.channel_file")] pub channel_file: Option<String>,
    #[structopt(long = "params.attachments")] pub attachments: Option<String>,
    #[structopt(long = "params.attachments_file")] pub attachments_file: Option<String>,
    #[structopt(long = "params.username")] pub username: Option<String>,
    #[structopt(long = "params.icon_url")] pub icon_url: Option<String>,
    #[structopt(long = "params.icon_emoji")] pub icon_emoji: Option<String>,
    #[structopt(long = "params.thread_ts_file")] pub thread_ts_file: Option<String>,
    #[structopt(long = "params.reply_broadcast")] pub reply_broadcast: bool,
    #[structopt(name = "DIR", help = "destination dir")] pub dir: String,
}

impl TryFrom<Args> for Request {
    type Error = Error;
    fn try_from(value: Args) -> StdResult<Self, Self::Error> {
        let source = Source {
            token: value.token.clone(),
        };

        let params = Params {
            channel: value.channel()?,
            attachments: value.attachments()?,
            username: value.username.clone(),
            icon_url: value.icon_url.clone(),
            icon_emoji: value.icon_emoji.clone(),
            thread_ts: value.thread_ts()?,
            reply_broadcast: value.reply_broadcast,
        };
        Ok(Request { source, params })
    }
}

impl Args {
    fn attachments(&self) -> Result<String> {
        let attach = match (self.attachments.as_ref(), self.attachments_file.as_ref()) {
            (Some(buf), _) => Ok(buf.to_string()),
            (None, Some(path)) => read_file(self.path(path)),
            _ => Err(format_err!(
                "{}",
                "Specify either attachments or attachments_file"
            )),
        }?;
        // @todo DI
        pairs_extract(&attach, env::vars())
    }

    fn channel(&self) -> Result<String> {
        match (self.channel.as_ref(), self.channel_file.as_ref()) {
            (Some(buf), None) => Ok(buf.to_string()),
            (None, Some(path)) => read_file(self.path(path)),
            _ => Err(format_err!("{}", "Specify either channel or channel_file")),
        }
    }

    pub fn path(&self, path: &str) -> PathBuf {
        PathBuf::from(&format!("{}/{}", self.dir, path))
    }

    fn thread_ts(&self) -> Result<Option<String>> {
        match self.thread_ts_file.as_ref() {
            Some(path) => Ok(Some(read_file(self.path(path))?)),
            None => Ok(None),
        }
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

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs::File;
    use core::TryFromIterator;
    use mktemp::Temp;

    fn new(dir: &str) -> Args {
        Args {
            token: "test-token".to_string(),
            channel: Some("test-channel".to_string()),
            channel_file: None,
            attachments: Some("test-attachments".to_string()),
            attachments_file: None,
            username: Some("test-username".to_string()),
            icon_url: Some("test-icon-url".to_string()),
            icon_emoji: Some("test-icon-emoji".to_string()),
            thread_ts_file: None,
            reply_broadcast: false,
            dir: dir.to_string(),
        }
    }

    #[test]
    fn out_opt_suc() {
        let tmp = Temp::new_dir().unwrap();
        let buf = tmp.to_path_buf();
        let tmp_dir = buf.to_str().unwrap();

        let expect = new(tmp_dir);
        let row = vec![
            "out",
            tmp_dir,
            "--params.channel",
            "test-channel",
            "--params.attachments",
            "test-attachments",
            "--source.token",
            "test-token",
            "--params.username",
            "test-username",
            "--params.icon_url",
            "test-icon-url",
            "--params.icon_emoji",
            "test-icon-emoji",
        ];

        let options = row.into_iter().map(String::from).collect::<Vec<String>>();
        let actual = Args::try_from_iter(options).unwrap();
        assert_eq!(expect, actual);
    }

    #[test]
    fn valid_suc() {
        let tmp = Temp::new_dir().unwrap();
        let buf = tmp.to_path_buf();
        let tmp_dir = buf.to_str().unwrap();

        let row = vec![
            "out",
            tmp_dir,
            "--source.token",
            "test-token",
            "--params.channel",
            "test-channel",
            "--params.attachments",
            "test-attachments",
        ];
        let options = row.into_iter().map(String::from).collect::<Vec<String>>();
        let actual = Args::try_from_iter(options);
        assert!(actual.is_ok())
    }

    #[test]
    fn valid_fail() {
        let tmp = Temp::new_dir().unwrap();
        let buf = tmp.to_path_buf();
        let tmp_dir = buf.to_str().unwrap();

        let row = vec!["out", tmp_dir];
        let options = row.into_iter().map(String::from).collect::<Vec<String>>();
        let actual = Args::try_from_iter(options);
        assert!(actual.is_err())
    }

    #[test]
    fn suc_channel_path() {
        let channel_file = "test-channel.txt";

        let tmp = Temp::new_dir().unwrap();
        let buf = tmp.to_path_buf();
        let tmp_dir = buf.to_str().unwrap();

        let mut tmp_file = tmp.to_path_buf();
        tmp_file.push(channel_file);

        File::create(&tmp_file).unwrap();

        let row = vec![
            "out",
            tmp_dir,
            "--source.token",
            "test-token",
            "--params.channel_file",
            channel_file,
            "--params.attachments",
            "test-attachments",
        ];

        let options = row.into_iter().map(String::from).collect::<Vec<String>>();
        let actual = Args::try_from_iter(options);
        assert!(actual.is_ok());
    }

    #[test]
    fn fail_channel_path() {
        let tmp = Temp::new_dir().unwrap();
        let buf = tmp.to_path_buf();
        let tmp_dir = buf.to_str().unwrap();

        let row = vec![
            "out",
            tmp_dir,
            "--source.token",
            "test-token",
            "--params.channel_file",
            "not-found.txt",
            "--params.attachments",
            "test-attachments",
        ];
        let options = row.into_iter().map(String::from).collect::<Vec<String>>();
        let actual = Args::try_from_iter(options);
        assert!(actual.is_err())
    }

    #[test]
    fn webhook_ng() {
        let tmp = Temp::new_dir().unwrap();
        let buf = tmp.to_path_buf();
        let tmp_dir = buf.to_str().unwrap();

        let row = vec![
            "out",
            tmp_dir,
            "--source.token",
            "https://xxxx",
            "--params.channel",
            "test-channel",
            "--params.attachments",
            "test-attachments",
        ];
        let options = row.into_iter().map(String::from).collect::<Vec<String>>();
        let actual = Args::try_from_iter(options);
        assert!(actual.is_err())
    }
}
