use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct FormAtSchool {
  /// 近 48 小时内是否进行过核酸检测。
  /// 0: 否，1: 是。
  /// 目前无需填写，缺省为 1。
  pub hsjc: Option<i32>,

  /// 西安市一码通颜色。
  /// 1: 绿码，2: 黄码，3: 红码。
  pub xasymt: Option<i32>,

  /// 不知道什么玩意儿，缺省为 addRbxx。
  pub actionType: Option<String>,

  /// 学号。
  pub userLoginId: Option<String>,

  /// 当前所在位置。
  /// 1: 在学校，2: 在西安，3: 在国内，4: 在国外。
  /// （该项除 `在学校` 外疑似有误）
  pub szcsbm: Option<i32>,

  /// 不知道什么玩意儿，缺省为 1。
  pub bdzt: Option<i32>,

  /// 当前所在位置具体地址。
  pub szcsmc: Option<String>,

  /// 今天的体温范围。
  /// 0: 37.3 度以下，1: 37.3 度 ~ 37.8 度，2: 37.8 ~ 39.0 度，3: 39.0 度以上。
  pub sfyzz: Option<i32>,

  /// 疑似症状。
  /// 用半角逗号分隔的列表。
  /// 0: 无，1: 发热，6: 胸闷，7: 咳嗽，8: 其他症状（咽痛、呼吸困难、乏力、恶心呕吐、腹泻、结膜炎、肌肉酸痛等症状）。
  pub sfqz: Option<String>,

  /// 不知道什么玩意儿，看名字像是填报来源(?)，缺省为 sso。
  pub tbly: Option<String>,

  /// 其它情况说明。
  pub qtqksm: Option<String>,

  /// 疑似症状情况说明。
  /// 选无疑似症状不会出现这个框，但是不知道为啥抓包抓到了，不知道他们前端写的什么锤子。
  pub ycqksm: Option<String>,

  /// 不知道什么玩意儿，看名字像是用户类型(?)，缺省为 2。
  pub userType: Option<i32>,

  /// 姓名。
  pub userName: Option<String>,
}
