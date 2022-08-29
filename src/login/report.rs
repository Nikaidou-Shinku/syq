use anyhow::{bail, Result};
use regex::Regex;
use reqwest::{ClientBuilder, redirect::Policy};

use crate::constants::{
  AUTH_URL,
  USER_AGENT,
  REPORT_LOGIN_URL,
  SERVICE_REPORT,
};

pub async fn report_login(tgc: &str) -> Result<String> {
  let client = ClientBuilder::new()
    .redirect(Policy::none())
    .build()?;

  // 1st request: get JSESSIONID
  let resp = client.get(REPORT_LOGIN_URL).send().await?;

  let jsessionid_raw = resp.headers()
    .get("set-cookie").unwrap()
    .to_str()?;
  let jsessionid = &jsessionid_raw.split(';')
    .next().unwrap()[11..];

  // 2nd request: get ticket (for auth)
  let resp = client.get(AUTH_URL)
    .query(&[("service", SERVICE_REPORT)])
    .header("user-agent", USER_AGENT)
    .header("cookie", format!("TGC={}", tgc))
    .send().await?;

  let re = Regex::new("ticket=(.+)")?;
  let ticket_raw = if let Some(raw) = resp.headers().get("location") {
    raw.to_str()?
  } else {
    bail!("Token is invalid, please try again!");
  };
  let ticket = re.captures(ticket_raw).unwrap()
    .get(1).unwrap()
    .as_str();

  // 3rd request: sign in with ticket & JSESSIONID
  client.get(REPORT_LOGIN_URL)
    .query(&[("ticket", ticket)])
    .header("Cookie", format!("JSESSIONID={}", jsessionid))
    .send().await?;

  Ok(jsessionid.to_string())
}
