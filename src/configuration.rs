// Based on https://stackoverflow.com/a/55134333/4812090
use clap::ArgMatches;
use log::trace;
use serde::Deserialize;
use std::{fs::read_to_string, path::PathBuf};

#[derive(Debug, Clone, Deserialize)]
pub struct Configuration {
    #[serde(rename = "SECRET_KEY")]
    #[serde(default)]
    pub secret_key: String,

    #[serde(rename = "API_KEY")]
    #[serde(default)]
    pub api_key: String,

    pub log_path: Option<PathBuf>,

    #[serde(default = "default_sell_to_asset")]
    pub default_quote_asset: String,
}

fn default_sell_to_asset() -> String {
    "USD".to_string()
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            api_key: "".to_string(),
            secret_key: "".to_string(),
            log_path: None,
            default_quote_asset: default_sell_to_asset(),
        }
    }
}

impl Configuration {
    pub fn new(matches: &ArgMatches) -> Self {
        let mut config = if let Some(path_str) = matches.value_of("config") {
            let config_file_path = PathBuf::from(path_str.to_string());
            let config: Configuration = match read_to_string(config_file_path) {
                Ok(str) => match toml::from_str(&str) {
                    Ok(cfg) => {
                        trace!("config from file:\n{:#?}", cfg);
                        cfg
                    }
                    Err(_) => Configuration::default(),
                },
                Err(_) => Configuration::default(),
            };
            config
        } else {
            Configuration::default()
        };

        config.update_config(&matches);
        trace!("config after update_config:\n{:#?}", config);

        config
    }

    fn update_config(&mut self, matches: &ArgMatches) {
        let name = "api-key";
        if let Some(value) = matches.value_of(name) {
            self.api_key = value.to_string();
        }

        let name = "secret-key";
        if let Some(value) = matches.value_of(name) {
            self.secret_key = value.to_string();
        }

        let name = "log-path";
        if let Some(value) = matches.value_of(name) {
            let path_buf = PathBuf::from(value.to_string());
            self.log_path = Some(path_buf);
        }

        let name = "default-quote-asset";
        if let Some(value) = matches.value_of(name) {
            self.default_quote_asset = value.to_string();
        }
    }
}
