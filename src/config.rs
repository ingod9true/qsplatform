use serde::{Deserialize, Serialize};
use log::debug;
use std::path::Path;
use std::fs::File;

#[derive(Eq, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub(crate) struct Config {
    pub(crate) conf: Conf,
    pub(crate) log: Log
}

#[derive(Eq, Copy, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub(crate)  struct Log {
    pub(crate) level: LogLevel,
    pub(crate) disk: bool,
    pub(crate) console: bool
}


#[derive(Eq,  Debug, Clone, PartialEq, Deserialize, Serialize)]
pub(crate)  struct Conf {
    pub(crate) addr: String,
    pub(crate) port: u32,
    pub(crate) shutdown: bool
}

#[derive(Eq, Copy, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub(crate)  enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Critical,
}

impl LogLevel {

    // pub fn as_str(&self) -> &str {
    //     match self {
    //         LogLevel::Warn => "Warn",
    //         LogLevel::Info => "Info",
    //         LogLevel::Debug => "Debug",
    //         LogLevel::Error => "Error",
    //         LogLevel::Critical => "Critical",
    //     }
    // }

    #[inline(always)]
    pub fn to_level_filter(self) -> log::LevelFilter {
        match self {
            LogLevel::Warn => log::LevelFilter::Warn,
            LogLevel::Info => log::LevelFilter::Info,
            LogLevel::Debug => log::LevelFilter::Debug,
            LogLevel::Error => log::LevelFilter::Error,
            LogLevel::Critical => log::LevelFilter::Trace,
        }
    }
}

impl From<String> for Config {
    fn from(file_path: String) -> Self {
        let path = Path::new(&file_path);
        if !path.is_file() {
          return   Config::default()
        }
        let file = File::open(path);
        if let Ok(fi) = file {
            let d: Result<Config, serde_yaml::Error> = serde_yaml::from_reader(fi);
            match d {
                Ok(c) => {
                    if c.conf.addr.len() != 0 && c.conf.port != 0 {
                        return  c
                    }
                },
                Err(e) => debug!("{}", e),
            }
        }
       Config::default()

    }
}

impl Default for Config {
    fn default() -> Self {
        Config{
            log: Log::default(),
            conf: Conf::default()
        }
    }
}

impl Default for Log {
    fn default() -> Self {
        Log {
            level: LogLevel::default(),
            disk: false,
            console: true
        }
    }
}

impl Default for Conf {
    fn default() -> Self {
        Conf{
            addr: String::from("127.0.0.1"),
            port: 3000,
            shutdown: true
        }
    }
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Error
    }
}


#[test]
fn  test_model() {
    let model =  Config::default();
    assert_eq!("127.0.0.1", model.conf.addr);
    assert_eq!(true, model.conf.shutdown);
    assert_eq!(3000, model.conf.port);
    assert_eq!(LogLevel::Error, model.log.level);
    assert_eq!(true, model.log.console);
    assert_eq!(false, model.log.disk);
    let conf = Config::from(String::from("config.yaml"));
    assert_eq!(conf.conf.port,6000);
    let conf = Config::from(String::from("xxxxx"));
    assert_eq!(conf.conf.port,3000);


}
