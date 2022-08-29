use anyhow::{anyhow, bail, Result};
use regex::Regex;
use serde::Serialize;
use reqwest::Client;

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct Params {
  sign: String,
  timeStamp: String,
}

pub async fn fetch_params(
  cookie: &str,
  client: &Client,
) -> Result<(bool, Params)> {
  let re = Regex::new(r"url:'ry_util\.jsp\?sign=(.+)&timeStamp=(\d+)',")?;

  let resp = client.get("https://yqtb.nwpu.edu.cn/wx/ry/jrsb_xs.jsp")
    .header("Cookie", format!("JSESSIONID={}", cookie))
    .send().await?;

  match resp.status().into() {
    200 => {
      let html = resp.text().await?;
      let already = html.find("您已提交今日填报，重新提交将覆盖上一次的信息。").is_some();
      let res = re.captures(&html).ok_or(anyhow!("Failed to parse the report page!"))?;

      let sign = res.get(1)
        .ok_or(anyhow!("Failed to get the param `sign` from the report page!"))?
        .as_str().to_string();
      #[allow(non_snake_case)]
      let timeStamp = res.get(2)
        .ok_or(anyhow!("Failed to get the param `timeStamp` from the report page!"))?
        .as_str().to_string();

      Ok((already, Params { sign, timeStamp }))
    }
    500 => {
      bail!("Failed to open the report page!\nMaybe check your cookie?");
    }
    _code => {
      bail!("Failed to open the report page!\nStatus code: {}.", _code);
    }
  }
}
