use lazy_static::lazy_static;
use serde::Deserialize;
use std::sync::RwLock;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
  pub llllll_base_url: String,
  pub llllll_port: u16,
  pub llllll_url: String,
}

lazy_static! {
  pub static ref CONFIG: RwLock<Config> = RwLock::new(Config::new(String::from("localhost"), 2999));
}

impl Config {
  pub fn new(base_url: String, port: u16) -> Config {
    Config {
      llllll_base_url: base_url.clone(),
      llllll_port: port,
      llllll_url: format!("https://{}:{}", base_url, port),
    }
  }
}
