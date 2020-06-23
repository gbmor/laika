use std::fs::OpenOptions;

use simplelog::{Config, LevelFilter, WriteLogger};

pub fn init(file: &str) {
    let logfile = match OpenOptions::new().append(true).create(true).open(file)
    {
        Err(e) => {
            panic!("Could not open log file: {}", e);
        }
        Ok(f) => f,
    };

    if let Err(e) =
        WriteLogger::init(LevelFilter::Info, Config::default(), logfile)
    {
        panic!("Could not initiate logging: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn init_bad_file() {
        init("/var/log/wrongfile_laika.log");
    }

    #[test]
    fn init_good_file() {
        init("/tmp/laika.log");
    }
}
