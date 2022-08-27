use anyhow::Result;

use crate::report::report;

pub async fn command_report(cookie: &str, repeat: bool) -> Result<()> {
  if repeat {
    loop {
      let result = report(cookie, false).await;
      if let Err(e) = result {
        eprintln!("{}", e);
      } else {
        break;
      }
    }
    Ok(())
  } else {
    report(cookie, true).await
  }
}
