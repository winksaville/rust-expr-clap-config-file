// Based on https://stackoverflow.com/a/55134333/4812090
use clap::ArgMatches;
use log::trace;
use rust_decimal::prelude::*;
use std::error::Error;

mod arg_matches;
mod configuration;

use crate::{arg_matches::arg_matches, configuration::Configuration};

#[allow(clippy::unnecessary_wraps)]
fn cmd_auto_sell(config: &Configuration, _matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    println!("auto_sell config:\n{:#?}", config);

    Ok(())
}

fn cmd_buy_market(config: &Configuration, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    println!("cmd_buy_market config:\n{:#?}", config);

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

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    trace!("main:+");

    let matches = arg_matches()?;

    let config = Configuration::new(&matches);

    // Call subcommands
    if matches.subcommand_matches("auto-sell").is_some() {
        cmd_auto_sell(&config, &matches)?;
    } else if let Some(matches) = matches.subcommand_matches("buy-market") {
        cmd_buy_market(&config, &matches)?;
    }

    println!("done");
    Ok(())
}
