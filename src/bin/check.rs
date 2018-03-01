extern crate failure;
extern crate log;
extern crate snotif;

use std::process::exit;

use snotif::util::*;
use snotif::error::Result;
use snotif::concourse::check::app::run;

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
    run()
}
