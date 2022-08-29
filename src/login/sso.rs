use anyhow::{bail, Result};
use regex::Regex;
use reqwest::Client;

use crate::constants::{AUTH_URL, USER_AGENT};

pub async fn sso_login(
  username: &str,
  password: &str,
) -> Result<String> {
  let re = Regex::new("<input type=\"hidden\" name=\"execution\" value=\"(.+?)\"/>").unwrap();
  let client = Client::new();

  let execution_raw = client.get(AUTH_URL)
    .header("user-agent", USER_AGENT)
    .send().await?
    .text().await?;

  let execution = re.captures(&execution_raw).unwrap()
    .get(1).unwrap()
    .as_str();

  let login_resp = client.post(AUTH_URL)
    .form(&[
      ("username", username),
      ("password", password),
      ("execution", execution),
      ("_eventId", "submit"),
    ]).send().await?;

  if login_resp.status() == 401 {
    bail!("Please check your username and password.");
  }

  let tgc_raw = login_resp.headers()
    .get("set-cookie").unwrap()
    .to_str().unwrap();
  let tgc = &tgc_raw.split(';')
    .next().unwrap()[4..];

  Ok(tgc.to_string())
}
