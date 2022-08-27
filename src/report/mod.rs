mod params;

use anyhow::{bail, Result};
use serde_json::Value;
use reqwest::Client;

use crate::form::Form;
use params::fetch_params;

pub async fn report(cookie: &str, force: bool) -> Result<()> {
  let client = Client::new();

  let params = fetch_params(cookie, &client).await?;
  if params.0 && !force {
    return Ok(());
  }

  let form = Form::new();

  let resp = client.post("https://yqtb.nwpu.edu.cn/wx/ry/ry_util.jsp")
    .query(&params.1)
    .form(&form)
    .header("Cookie", format!("JSESSIONED={}", cookie))
    .header("Referer", "https://yqtb.nwpu.edu.cn/wx/ry/jrsb_xs.jsp")
    .send().await?;
  let res = resp.text().await?.replace("－", "-"); // 😅
  let res: Value = serde_json::from_str(&res)?;

  if let Some(code) = res["state"].as_str() {
    if code == "1" {
      Ok(())
    } else {
      bail!(res)
    }
  } else {
    bail!(res);
  }
}
