use std::fs::OpenOptions;

use simplelog::{Config, LevelFilter, WriteLogger};

use crate::err;

pub fn init(file: &str) {
    let logfile = match OpenOptions::new().append(true).create(true).open(file)
    {
        Err(e) => {
            err::notify_then_exit(&format!("Could not open log file: {}", e));
            // rustc complains about incompatible branches even though this is unreachable
            panic!();
        }
        Ok(f) => f,
    };

    if let Err(e) =
        WriteLogger::init(LevelFilter::Info, Config::default(), logfile)
    {
        err::notify_then_exit(&format!("Could not initiate logging: {}", e));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_good_file() {
        init("/tmp/laika.log");
    }
}
