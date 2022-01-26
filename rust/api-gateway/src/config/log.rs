use std::env;
use log::{LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

pub fn config_logging() -> Config {
    let storage = log_storage();
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)}/{l}/{t}/{m} \n")))
        .build(storage)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Info),
        ).expect("Log config..."); 
    config
}

fn log_storage() -> String {
    env::var("LOG_STORAGE").unwrap_or_else(|_| "D:/logs/logs_staff_service.log".into())
}