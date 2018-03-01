use std::path::PathBuf;

use error::Result;
use core::Validator;
use concourse::out::args::Args;

pub struct ArgsValidator<'a>(&'a Args);

impl<'a> ArgsValidator<'a> {
    pub fn from_ref(args: &'a Args) -> ArgsValidator<'a> {
        ArgsValidator(args)
    }

    fn valid_token(&self) -> Result<()> {
        let token = self.0.token.as_str();
        if let Some(_) = token.find("https://") {
            let msg = "Are you registering webhook url?\n\
            It is a token that starts with a string of xoxp-.:{}\n\
            https://api.slack.com/apps";
            bail!("{}", msg)
        }

        Ok(())
    }

    fn valid_attachments_file(&self) -> Result<()> {
        match (
            self.0.attachments.as_ref(),
            self.0.attachments_file.as_ref(),
        ) {
            (Some(_), _) => Ok(()),
            (None, Some(path)) => Self::exists(&self.0.path(path)),
            _ => Err(format_err!(
                "{}",
                "Specify either attachments or attachments_file"
            )),
        }
    }

    fn valid_channel(&self) -> Result<()> {
        match (self.0.channel.as_ref(), self.0.channel_file.as_ref()) {
            (Some(_), None) => Ok(()),
            (None, Some(path)) => Self::exists(&self.0.path(path)),
            _ => Err(format_err!("{}", "Specify either channel or channel_file")),
        }
    }

    fn exists(path: &PathBuf) -> Result<()> {
        if path.exists() && path.is_file() {
            Ok(())
        } else {
            bail!("Not fond attachments_file. path: {:?}", path)
        }
    }

    fn valid_dir(&self) -> Result<()> {
        let dir = self.0.dir.as_str();
        if !PathBuf::from(dir).is_dir() {
            bail!("Dir is not exist. dir:{}", dir)
        }
        Ok(())
    }
}

impl<'a> Validator for ArgsValidator<'a> {
    fn validate(&self) -> Result<()> {
        self.valid_attachments_file()?;
        self.valid_channel()?;
        self.valid_token()?;
        self.valid_dir()
    }
}
