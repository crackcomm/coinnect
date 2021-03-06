//! Use this module to create a generic API.


#![allow(new_ret_no_self)]

use std::path::PathBuf;

use exchange::{Exchange, ExchangeApi};
use bitstamp::{BitstampApi, BitstampCreds};
use kraken::{KrakenApi, KrakenCreds};
use poloniex::{PoloniexApi, PoloniexCreds};
use error::*;

pub trait Credentials {
    /// Get an element from the credentials.
    fn get(&self, cred: &str) -> Option<String>;
    /// Return the targeted `Exchange`.
    fn exchange(&self) -> Exchange;
    /// Return the client name.
    fn name(&self) -> String;
}

#[derive(Debug)]
pub struct Coinnect;

impl Coinnect {
    /// Create a new CoinnectApi by providing an API key & API secret
    pub fn new<C: Credentials>(exchange: Exchange, creds: C) -> Result<Box<ExchangeApi>> {
        match exchange {
            Exchange::Bitstamp => Ok(Box::new(BitstampApi::new(creds)?)),
            Exchange::Kraken => Ok(Box::new(KrakenApi::new(creds)?)),
            Exchange::Poloniex => Ok(Box::new(PoloniexApi::new(creds)?)),
        }
    }

    /// Create a new CoinnectApi from a json configuration file. This file must follow this
    /// structure:
    ///
    /// For this example, you could use load your Bitstamp account with
    /// `new_from_file(Exchange::Bitstamp, "account_bitstamp", Path::new("/keys.json"))`
    pub fn new_from_file(exchange: Exchange,
                         name: &str,
                         path: PathBuf)
                         -> Result<Box<ExchangeApi>> {
        match exchange {
            Exchange::Bitstamp => {
                Ok(Box::new(BitstampApi::new(BitstampCreds::new_from_file(name, path)?)?))
            }
            Exchange::Kraken => {
                Ok(Box::new(KrakenApi::new(KrakenCreds::new_from_file(name, path)?)?))
            }
            Exchange::Poloniex => {
                Ok(Box::new(PoloniexApi::new(PoloniexCreds::new_from_file(name, path)?)?))
            }
        }
    }
}
