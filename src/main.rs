// Based on https://stackoverflow.com/a/55134333/4812090
use clap::{crate_version, App, Arg, ArgMatches};
use serde::Deserialize;
use std::io::prelude::*;
use std::{error::Error, fs::File, path::PathBuf};

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
    pub default_sell_to_asset: String,
}

fn default_sell_to_asset() -> String {
    "USD".to_string()
}

#[derive(PartialEq)]
enum UseCliDefaults {
    Yes,
    No,
}

fn update_config(
    config: &mut Configuration,
    matches: ArgMatches,
    use_cli_defaults: UseCliDefaults,
) {
    let name = "api-key";
    if use_cli_defaults == UseCliDefaults::Yes || matches.occurrences_of(name) > 0 {
        println!("using defaults");
        config.api_key = matches.value_of(name).unwrap().to_string();
    }
    let name = "secret-key";
    if use_cli_defaults == UseCliDefaults::Yes || matches.occurrences_of(name) > 0 {
        config.secret_key = matches.value_of(name).unwrap().to_string();
    }
    let name = "log-path";
    if use_cli_defaults == UseCliDefaults::Yes || matches.occurrences_of(name) > 0 {
        config.log_path = Some(PathBuf::from(matches.value_of(name).unwrap().to_string()));
    }
    let name = "default-sell-to-asset";
    if use_cli_defaults == UseCliDefaults::Yes || matches.occurrences_of(name) > 0 {
        config.default_sell_to_asset = matches.value_of(name).unwrap().to_string();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Exper clap config")
        .version(crate_version!())
        .about("Experiment using a config file")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file"),
        )
        .arg(
            Arg::with_name("api-key")
                .short("a")
                .long("api-key")
                .help("Define the api key")
                .default_value("default api key")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("secret-key")
                .short("s")
                .long("secret-key")
                .help("Define the secret key")
                .default_value("default secret key")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("log-path")
                .short("l")
                .long("log-path")
                .help("Define log path")
                .default_value("data/log.txt")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("default-sell-to-asset")
                .short("d")
                .long("default-sell-to-asset")
                .help("The asset to sell to by default")
                .default_value("USD")
                .takes_value(true),
        )
        .get_matches();

    let mut config_string = "".to_string();
    let mut config = Configuration {
        api_key: "".to_string(),
        secret_key: "".to_string(),
        log_path: None,
        default_sell_to_asset: "".to_string(),
    };

    if let Some(c) = matches.value_of("config") {
        // Get defaults from config file
        let file = File::open(c);
        match file {
            Ok(mut f) => {
                f.read_to_string(&mut config_string)
                    .expect("Error reading value");
                config = toml::from_str(&config_string)?;
            }
            Err(_) => println!("Error reading config file"),
        }
        println!("config from file: {:#?}", config);
        update_config(&mut config, matches, UseCliDefaults::No);
    } else {
        update_config(&mut config, matches, UseCliDefaults::Yes);
    }
    println!("config after update: {:#?}", config);

    Ok(())
}
