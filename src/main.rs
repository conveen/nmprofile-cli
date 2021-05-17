// Copyright 2021 Conveen

extern crate flexi_logger as logging;

mod cli;
mod error;
mod profile;
mod profiles;

use structopt::StructOpt;

/// Logging formatter
fn log_format(
    w: &mut dyn std::io::Write,
    now: &mut logging::DeferredNow,
    record: &log::Record,
) -> Result<(), std::io::Error> {
    write!(
        w,
        "{}\t{}\t{}",
        now.now().format("%Y-%m-%d %H:%M:%S%:z"),
        record.level(),
        &record.args(),
    )
}

fn main() {
    logging::Logger::with_env_or_str("info")
        .format(log_format)
        .start()
        .unwrap_or_else(|err| panic!("Failed to initialize logger ({})", err));

    cli::Cli::from_args().run();
}
