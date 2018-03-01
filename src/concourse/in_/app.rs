use std::fmt::Debug;
use serde_json;

use core::TryFromIterator;
use error::Result;
use io::*;
use concourse::*;
use concourse::in_::response::Response;
use concourse::in_::request::Request;
use concourse::in_::args::*;

const THREAD_TS_FILE_NAME: &str = ".thread_ts";

// @todo only reference
pub fn run<I>(row_args: I) -> Result<String>
    where
        I: IntoIterator<Item = String> + Debug,
{
    debug!("row_args: {:?}", &row_args);

    let args = Args::try_from_iter(row_args)?;
    let dir = args.dir.clone();
    let request = Request::from(args);

    let thread_ts_path = thread_path(&dir);
    let version = request.version;
    if let Some(ref ts) = version.thread_ts {
        write_file(thread_ts_path, ts)?;
    }

    let metadata = vec![] as Metadata;
    let in_ = Response { version, metadata };
    let resp = serde_json::to_string(&in_)?;
    Ok(resp)
}

fn thread_path(dir: &str) -> String {
    format!("{}/{}", dir, THREAD_TS_FILE_NAME)
}

#[cfg(test)]
mod tests {

    use concourse::in_::app::*;

    fn base_args() -> Vec<String> {
        let base = vec![
            "target/x86_64-apple-darwin/debug/in",
            "./tmp",
            "--source.token",
            "xoxp-9999999999-99999999999-999999999999-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
            "--version.thread_ts",
            "1519169532.000296",
        ];
        base.into_iter().map(String::from).collect::<Vec<_>>()
    }

    #[test]
    fn run_ok() {
        let expt = r#"{"version":{"thread_ts":"1519169532.000296"},"metadata":[]}"#;
        let args = base_args();
        let actl = run(args);
        assert!(actl.is_ok());
        assert_eq!(actl.unwrap(), expt)
    }

    #[test]
    fn run_ts_format_error() {
        let invalid = (vec![
            "target/x86_64-apple-darwin/debug/in",
            "./tmp",
            "--source.token",
            "xoxp-test",
            "--version.thread_ts",
            "1519169532",
        ]).into_iter()
            .map(String::from)
            .collect::<Vec<_>>();

        let actl = run(invalid);
        assert!(actl.is_err());
    }

    #[test]
    fn run_ts_none() {
        let invalid = (vec![
            "target/x86_64-apple-darwin/debug/in",
            "./tmp",
            "--source.token",
            "xoxp-test",
        ]).into_iter()
            .map(String::from)
            .collect::<Vec<_>>();

        let actl = run(invalid);
        assert!(actl.is_ok());
    }

    // @todo concourse format check
    // http://concourse.ci/implementing-resources.html
    #[test]
    fn run_ts_empty() {
        let invalid = (vec![
            "target/x86_64-apple-darwin/debug/in",
            "./tmp",
            "--source.token",
            "xoxp-test",
            "--version.thread_ts",
            "",
        ]).into_iter()
            .map(String::from)
            .collect::<Vec<_>>();

        let actl = run(invalid);
        assert!(actl.is_err());
    }

    #[test]
    fn run_invalid_token() {
        let invalid = (vec![
            "target/x86_64-apple-darwin/debug/in",
            "./tmp",
            "--source.token",
            "https://hooks.slack.com/services/TXXXXXXXX/XXXXXXXXX/XXXXXXXXXXXXXXXXXXXXXXXX",
            "--version.thread_ts",
            "1519169532.000296",
        ]).into_iter()
            .map(String::from)
            .collect::<Vec<_>>();

        let actl = run(invalid);
        assert!(actl.is_err());
    }

}
