extern crate failure;
extern crate log;
extern crate serde_json;
extern crate snotif;

use std::process::exit;
use snotif::concourse::out::app::run;
use snotif::util::*;
use snotif::io::*;
use snotif::core::WrapValue;
use snotif::error::Result;
use std::io;
use serde_json::Value;

enum ExitStatus {
    Success,
    Failure,
}

fn main() {
    let code = match _main() {
        Ok(view) => {
            println!("{}", view);
            ExitStatus::Success
        }
        Err(err) => {
            eprintln!("{}", err);
            ExitStatus::Failure
        }
    };
    exit(code as i32)
}

fn _main() -> Result<String> {
    logger_init()?;
    let stdin = io::stdin();
    let handle = stdin.lock();

    let json = read(handle)?;
    let value = serde_json::from_str::<Value>(&json)?;
    let inputs: Vec<String> = WrapValue(value).into();
    let row_args = args_extend(inputs);
    run(row_args)
}
