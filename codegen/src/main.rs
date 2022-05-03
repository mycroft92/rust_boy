use serde;
use serde_yaml;
use std::error::Error;
use clap::Parser; // seems to be needed for arg parsing
mod fetch;
mod options;

//Setting up logging with log4rs
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use log4rs::filter::threshold::ThresholdFilter;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = crate::options::CMDArgs::parse();
    println!("Args: {:?}",cli);

    let stdout = ConsoleAppender::builder().build();
    let logfile = FileAppender::builder()
        .append(false)
        .encoder(Box::new(PatternEncoder::new(" {l} {t} - {m} {d} {n}")))
        .build(cli.log.as_str())?;

    //logs debug level to log and stdout only sees info level
    //Reference: https://github.com/estk/log4rs/blob/master/examples/log_to_file.rs
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(Appender::builder().filter(Box::new(ThresholdFilter::new(LevelFilter::Info))).build("stdout",Box::new(stdout)))
        .build(Root::builder()
                   .appender("logfile")
                   .appender("stdout")
                   .build(LevelFilter::Debug))?;

    log4rs::init_config(config)?;
    //Copy paste this stuff when you need to enable both console and file logging

    log::info!("Hello, world!");
    fetch::fetch(cli.url, cli.out)?;
    Ok(())
}
