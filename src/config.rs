
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use log::LevelFilter;
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
    pub fn as_str(&self) -> &str {
        match self {
            LogLevel::Warn => "Warn",
            LogLevel::Info => "Info",
            LogLevel::Debug => "Debug",
            LogLevel::Error => "Error",
            LogLevel::Critical => "Critical",
        }
    }

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

impl Config {
    pub(crate) const ENABLE_CLI_COLOR: bool = true;
    pub(crate) const ENABLE_LOG: bool = true;

}
// let f = std::fs::File::open("./config.yaml").expect("file open err");
// let d: Config = serde_yaml::from_reader(f).expect("yaml error");
// debug!("Read YAML string: {:?}", d.log.level);
// impl From<String> for Config {
//     fn from(file_path: String) -> Self {
//
//         let do_steps = || -> Result<Config, MyError> {
//             let mut file = File::open(file_path)?;
//             //try yaml
//             let d: Config = serde_yaml::from_reader(file)?;
//             if d.conf.addr.len() != 0 && d.conf.port != 0 {
//                 Ok(d)
//             }
//             Ok(Config::default())
//         };
//
//         if let Err(_err) = do_steps() {
//             println!("Failed to perform necessary steps");
//         }
//
//
//
//         Config::default()
//     }
// }

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
}
