extern crate failure;
extern crate log;
extern crate serde_json;
extern crate snotif;

use std::process::exit;
use std::io;
use serde_json::Value;

use snotif::error::Result;
use snotif::util::*;
use snotif::io::*;
use snotif::core::WrapValue;
use snotif::concourse::in_::app::*;

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
    let stdio = io::stdin();
    let handle = stdio.lock();

    let json = read(handle)?;
    let value = serde_json::from_str::<Value>(&json)?;
    let inputs: Vec<String> = WrapValue(value).into();
    let row_args = args_extend(inputs);
    run(row_args)
}
