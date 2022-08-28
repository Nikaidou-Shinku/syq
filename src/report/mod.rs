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

  let form = Form::new("config.toml")?;
  if form.other.is_none() && form.at_school.is_none() {
    bail!("No form to report");
  }
  if form.other.is_some() && form.at_school.is_some() {
    bail!("Can not report both at school and other");
  }

  let resp = if form.at_school.is_some() {
    client.post("https://yqtb.nwpu.edu.cn/wx/ry/ry_util.jsp")
      .query(&params.1)
      .form(&form.at_school)
      .header("Cookie", format!("JSESSIONED={}", cookie))
      .header("Referer", "https://yqtb.nwpu.edu.cn/wx/ry/jrsb_xs.jsp")
      .send().await?
  } else {
    client.post("https://yqtb.nwpu.edu.cn/wx/ry/ry_util.jsp")
      .query(&params.1)
      .form(&form.other)
      .header("Cookie", format!("JSESSIONED={}", cookie))
      .header("Referer", "https://yqtb.nwpu.edu.cn/wx/ry/jrsb_xs.jsp")
      .send().await?
  };
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
