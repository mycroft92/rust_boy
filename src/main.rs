// fn main() {
//     println!("Hello, world!");
// }

use std::error::Error;
use log::{LevelFilter,info};
use clap::Parser; // Needed for parse function here
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::filter::threshold::ThresholdFilter;

mod mmu;
mod cpu;
mod options;

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        return a;
    } else {
        b
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    let cli = crate::options::CMDArgs::parse();
    info!("Args: {:?}",cli);

    let stdout = ConsoleAppender::builder().build();
    let logfile = FileAppender::builder()
        .append(false)
        .encoder(Box::new(PatternEncoder::new(" {l} {t} - {m} {d} {n}")))
        .build(cli.log.as_str())?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(Appender::builder().filter(Box::new(ThresholdFilter::new(LevelFilter::Debug))).build("stdout",Box::new(stdout)))
        .logger(Logger::builder().build("scraper", LevelFilter::Warn))
        .logger(Logger::builder().build("curl", LevelFilter::Warn))
        .build(Root::builder()
                    .appender("logfile")
                    .appender("stdout")
                    .build(LevelFilter::Info))?;

    log4rs::init_config(config)?;

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    };

    Ok (())
}
