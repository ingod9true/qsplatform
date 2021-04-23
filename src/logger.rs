
#[derive(Debug)]
struct QSLogger;
use crate::config::LogLevel;
use log;
use log::Record;
use yansi::Paint;
use std::time::SystemTime;
use chrono::{DateTime, Utc};

pub(crate) fn init()  {
   let _ = log::set_boxed_logger(Box::new(QSLogger))
      .map(|()| log::set_max_level(LogLevel::Debug.to_level_filter()));
}

impl log::Log for QSLogger {
    fn enabled(&self, record: &log::Metadata<'_>) -> bool {
        match log::max_level().to_level() {
            Some(max) => record.level() <= max,
            None => false
        }
    }

    fn log(&self, record: &Record<'_>) {
        if !self.enabled(record.metadata()){ return }
        let now = SystemTime::now();
        let now: DateTime<Utc> = now.into();
        let now = now.to_rfc3339();
        match record.level() {
            log::Level::Error => {
                let result:String =  format!(">>> Error | {} | {:?}",now,record.args());
                println!("{}",Paint::red(result).bold());
            },
            log::Level::Warn => {
                let result:String =  format!(">>> Warn  | {} | {:?}",now,record.args());
                println!("{}",Paint::yellow(result).bold());
            }
            log::Level::Info =>{
                let result:String =  format!(">>> Info  | {} | {:?}",now,record.args());
                println!("{}",Paint::green(result).bold());
            },
            log::Level::Debug =>{
                let result:String =  format!(">>> Debug | {} | {:?}",now,record.args());
                println!("{}",result);
            },
            log::Level::Trace => {
                let result:String =  format!(">>> Trace | {} | {:?}",now,record.args());
                println!("{}",Paint::red(result).bold());
            }
        }
    }
    fn flush(&self) {
       //
    }
}

