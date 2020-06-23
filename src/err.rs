use std::process;

pub fn notify_then_exit(msg: &str) {
    log::error!("Fatal :: {}", msg);
    eprintln!();
    eprintln!("Fatal :: {}", msg);
    process::exit(1);
}
