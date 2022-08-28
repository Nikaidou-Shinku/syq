mod at_school;
mod other;

use std::{fs, io::Read};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

pub use at_school::FormAtSchool;
pub use other::FormOther;

#[derive(Deserialize, Serialize)]
pub struct Form {
  pub at_school: Option<FormAtSchool>,
  pub other: Option<FormOther>,
}

impl Form {
  pub fn new(path: &str) -> Result<Self> {
    let mut file = fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    toml::from_str(&content).map_err(|e| anyhow!(e))
  }
}
