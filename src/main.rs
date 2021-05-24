// Based on https://stackoverflow.com/a/55134333/4812090
use clap::{crate_version, App, Arg, ArgMatches, SubCommand};
use rust_decimal::prelude::*;
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
    pub default_quote_asset: String,
}

fn default_sell_to_asset() -> String {
    "USD".to_string()
}

fn update_config(config: &mut Configuration, matches: &ArgMatches) {
    let name = "api-key";
    if let Some(value) = matches.value_of(name) {
        config.api_key = value.to_string();
    }

    let name = "secret-key";
    if let Some(value) = matches.value_of(name) {
        config.secret_key = value.to_string();
    }

    let name = "log-path";
    if let Some(value) = matches.value_of(name) {
        let path_buf = PathBuf::from(value.to_string());
        config.log_path = Some(path_buf);
    }

    let name = "default-quote-asset";
    if let Some(value) = matches.value_of(name) {
        config.default_quote_asset = value.to_string();
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
                .help("Sets a custom config file")
                .env("BINANCE_CONFIG")
                .default_value("config.toml")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("api-key")
                .short("a")
                .long("api-key")
                .value_name("API-KEY")
                .help("Define the api key")
                .env("BINANCE_US_API_KEY")
                //.default_value("api key")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("secret-key")
                .short("s")
                .long("secret-key")
                .value_name("SECRET-KEY")
                .help("Define the secret key")
                .env("BINANCE_US_SECRET_KEY")
                //.default_value("secret key")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("log-path")
                .short("l")
                .long("log-path")
                .value_name("PATH")
                .help("Define log path")
                //.default_value("data/log.txt")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("default-quote-asset")
                .short("d")
                .long("default-quote-asset")
                .value_name("ASSET")
                .help("The name of the asset that is used to buy or sell another asset")
                .default_value("USD")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("auto-sell")
                .about("Automatically sell assets as defined in the configuration keep section"),
        )
        .subcommand(
            SubCommand::with_name("buy-market")
                .about("Buy an asset")
                .arg(
                    Arg::with_name("SYMBOL")
                        .help("Name of aseet")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("QUANTITY")
                        .help("Amount of asset to buy")
                        .required(true)
                        .index(2),
                ),
        )
        .get_matches();

    let mut config_string = "".to_string();
    let mut config = Configuration {
        api_key: "".to_string(),
        secret_key: "".to_string(),
        log_path: None,
        default_quote_asset: "".to_string(),
    };

    let config_file_path = if let Some(value) = matches.value_of("config") {
        PathBuf::from(value.to_string())
    } else {
        unreachable!("SNH: There should always be a config file path");
    };
    let file = File::open(config_file_path);
    if let Ok(mut f) = file {
        f.read_to_string(&mut config_string)
            .expect("Error reading value");
        config = toml::from_str(&config_string)?;
    }
    println!("config from file:\n{:#?}", config);

    update_config(&mut config, &matches);
    println!("config after update_config:\n{:#?}", config);

    // Call subcommands
    if matches.subcommand_matches("auto-sell").is_some() {
        println!("auto_sell has no parameters");
    } else if let Some(matches) = matches.subcommand_matches("buy-market") {
        let symbol = if let Some(sym) = matches.value_of("SYMBOL") {
            sym.to_string()
        } else {
            unreachable!("SYMBOL is the requried first positional parameter");
        };
        let quantity: Decimal = if let Some(q) = matches.value_of("QUANTITY") {
            match Decimal::from_str(q) {
                Ok(q) => q,
                Err(e) => return Err(format!("QUANTITY {}", e).into()),
            }
        } else {
            unreachable!("QUANTITY is the requried first positional parameter");
        };
        println!("symbol: {:?} quantity: {:?}", symbol, quantity);
    }
    println!("done");
    Ok(())
}
