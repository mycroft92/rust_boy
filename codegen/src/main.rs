use serde;
use serde_yaml;
use std::error::Error;
mod fetch;
mod options;

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = ConsoleAppender::builder().build();
    let logfile = FileAppender::builder()
        .append(false)
        .encoder(Box::new(PatternEncoder::new(" {l} {t} - {m} {d} {n}")))
        .build("output.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(Appender::builder().build("stdout",Box::new(stdout)))
        .build(Root::builder()
                   .appender("logfile")
                   .appender("stdout")
                   .build(LevelFilter::Debug))?;

    log4rs::init_config(config)?;
    log::info!("Hello, world!");
    fetch::fetch(None, None)?;
    Ok(())
}
