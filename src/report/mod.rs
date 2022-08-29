mod params;

use anyhow::{bail, Result};
use serde_json::Value;
use reqwest::Client;

use params::fetch_params;

use crate::form::Form;

pub async fn report(
  cookie: &str,
  form: &Form,
  force: bool,
) -> Result<()> {
  let client = Client::new();

  let params = fetch_params(cookie, &client).await?;
  if params.0 && !force {
    return Ok(());
  }

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
      .header("Cookie", format!("JSESSIONID={}", cookie))
      .header("Referer", "https://yqtb.nwpu.edu.cn/wx/ry/jrsb_xs.jsp")
      .send().await?
  } else {
    client.post("https://yqtb.nwpu.edu.cn/wx/ry/ry_util.jsp")
      .query(&params.1)
      .form(&form.other)
      .header("Cookie", format!("JSESSIONID={}", cookie))
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
