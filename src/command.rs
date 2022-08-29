use anyhow::{anyhow, Result};

use crate::{Args, Config, login};
use crate::report::report;

pub async fn command_report(args: &Args, config: &Config) -> Result<()> {
  let (username, password) = if let Some(cfg) = config.basic.as_ref() {
    let username = args.username.as_ref()
      .or(cfg.username.as_ref())
      .ok_or_else(|| anyhow!("Unable to get username"))?;
  
    let password = args.password.as_ref()
      .or(cfg.password.as_ref())
      .ok_or_else(|| anyhow!("Unable to get password"))?;

    (username, password)
  } else {
    let username = args.username.as_ref()
      .ok_or_else(|| anyhow!("Unable to get username"))?;
    let password = args.password.as_ref()
      .ok_or_else(|| anyhow!("Unable to get password"))?;

    (username, password)
  };

  if args.repeat {
    let tgc: String;
    loop {
      let tgc_raw = login::sso_login(username, password).await;
      if tgc_raw.is_ok() {
        tgc = tgc_raw.unwrap();
        break;
      }
      eprintln!("Failed to sign in to SSO, retrying...");
    }

    let jsessionid: String;
    loop {
      let jsessionid_raw = login::report_login(&tgc).await;
      if jsessionid_raw.is_ok() {
        jsessionid = jsessionid_raw.unwrap();
        break;
      }
      eprintln!("Failed to sign in to report system, retrying...");
    }

    loop {
      let result = report(&jsessionid, &config.report, false).await;
      if result.is_ok() {
        break;
      }
      eprintln!("Failed to report, retrying...");
    }

    Ok(())
  } else {
    let tgc = login::sso_login(username, password).await?;
    let jsessionid = login::report_login(&tgc).await?;
    report(&jsessionid, &config.report, true).await
  }
}
