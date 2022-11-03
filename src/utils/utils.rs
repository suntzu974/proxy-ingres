use config::ConfigError;
use serde::{Deserialize };

#[derive(Debug, Deserialize,Clone)]
pub struct Config {
    pub proxy_host:String,
    pub proxy_port:u16,
    pub ip_tomcat:String,
    pub ip_tomcat_port:u16,
    pub ip_tomcat_app:String,
    pub ip_tomcat_app_dev:String,
}

impl Config {
    #[cfg(target_os = "windows")]
    pub fn from_env() -> Result<Self, ConfigError> {
          config::Config::builder()
              .add_source(config::File::with_name("c:\\pitaya\\ingres-stock.toml"))
              .build()
              .unwrap()
              .try_deserialize()
      }
      #[cfg(not(target_os = "windows"))]
      pub fn from_env() -> Result<Self, ConfigError> {
            config::Config::builder()
                .add_source(config::File::with_name("/usr/local/ingres-stock.toml"))
                .build()
                .unwrap()
                .try_deserialize()
    }
}
