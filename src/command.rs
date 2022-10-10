use anyhow::{anyhow, Result};

use crate::{Args, Config, login};
use crate::report::report;

pub async fn command_report(args: &Args, config: &Config) -> Result<()> {
  let (username, password): (&str, &str) = if args.tgc.is_none() {
    if let Some(cfg) = config.basic.as_ref() {
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
    }
  } else {
    ("", "")
  };

  if args.repeat {
    let tgc: String;
    if args.tgc.is_none() {
      loop {
        match login::sso_login(username, password).await {
          Ok(res) => {
            tgc = res;
            println!("Sign in to SSO successfully.");
            break;
          }
          Err(err) => {
            eprintln!("[Error] {}", err);
            eprintln!("Failed to sign in to SSO, retrying...");
          }
        }
      }
    } else {
      println!("Read TGC, Sign in to SSO successfully.");
      tgc = args.tgc.clone().unwrap();
    }

    let jsessionid: String;
    loop {
      match login::report_login(&tgc).await {
        Ok(res) => {
          jsessionid = res;
          println!("Sign in to report system successfully.");
          break;
        }
        Err(err) => {
          eprintln!("[Error] {}", err);
          eprintln!("Failed to sign in to report system, retrying...");
        }
      }
    }

    loop {
      match report(&jsessionid, &config.report, false).await {
        Ok(_) => break,
        Err(err) => {
          eprintln!("[Error] {}", err);
          eprintln!("Failed to report, retrying...");
        },
      }
    }

    Ok(())
  } else {
    let tgc = if args.tgc.is_none() {
      login::sso_login(username, password).await?
    } else {
      args.tgc.clone().unwrap()
    };
    let jsessionid = login::report_login(&tgc).await?;
    report(&jsessionid, &config.report, true).await
  }
}
