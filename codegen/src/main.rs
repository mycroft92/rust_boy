#[macro_use]
extern crate lazy_static;


use std::error::Error;
use clap::Parser; // seems to be needed for arg parsing
use log::{LevelFilter,info};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::filter::threshold::ThresholdFilter;

mod fetch;
mod options;
mod inst_parser;
//Setting up logging with log4rs


fn main() -> Result<(), Box<dyn Error>> {
    let cli = crate::options::CMDArgs::parse();
    info!("Args: {:?}",cli);

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
        .logger(Logger::builder().build("scraper", LevelFilter::Warn))
        .logger(Logger::builder().build("curl", LevelFilter::Warn))
        .build(Root::builder()
                    .appender("logfile")
                    .appender("stdout")
                    .build(LevelFilter::Info))?;

    log4rs::init_config(config)?;
    //Copy paste this stuff when you need to enable both console and file logging

    log::info!("Hello, world!");
    //Run the fetch
    fetch::fetch(cli.url, cli.out)?;
    Ok(())
}
