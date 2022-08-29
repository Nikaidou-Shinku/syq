use std::{fs, io::Read};
use anyhow::{anyhow, Result};
use serde::Deserialize;

use crate::form::Form;

#[derive(Deserialize)]
pub struct BasicConfig {
  pub username: Option<String>,
  pub password: Option<String>,
}

#[derive(Deserialize)]
pub struct Config {
  pub basic: Option<BasicConfig>,
  pub report: Form,
}

impl Config {
  pub fn from_file(path: &str) -> Result<Self> {
    let mut file = fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    toml::from_str(&content).map_err(|e| anyhow!(e))
  }
}
