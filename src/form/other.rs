use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct FormOther {
  /// 近 48 小时内是否进行过核酸检测。
  /// 0: 否，1: 是。
  /// 目前无需填写，缺省为 1。
  pub hsjc: Option<i32>,

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
  pub ycqksm: Option<String>,

  /// 不知道什么玩意儿，看名字像是用户类型(?)，缺省为 2。
  pub userType: Option<i32>,

  /// 姓名。
  pub userName: Option<String>,

  /// 不知道什么玩意儿，缺省为空。
  pub sfczbcqca: Option<String>,

  /// 不知道什么玩意儿，缺省为空。
  pub czbcqcasjd: Option<String>,

  /// 不知道什么玩意儿，缺省为空。
  pub sfczbcfhyy: Option<String>,

  /// 不知道什么玩意儿，缺省为空。
  pub czbcfhyysjd: Option<String>,

  /// 当前所在位置具体地址。
  /// 我特么也不知道为什么有两个…
  pub szcsmc1: Option<String>,

  /// 近 15 天是否前往或经停中高风险地区，或其它有新冠患者病例报告的社区。
  /// 0: 否，1: 中高风险地区，3: 其它有病例报告的社区。
  pub sfjt: Option<i32>,

  /// 简单描述前往或经停的具体情况。
  pub sfjtsm: Option<String>,

  /// 近 15 天是否接触过出入或居住在中高风险地区的人员，以及其他有新冠患者病例报告的发热或呼吸道症状患者。
  /// 0: 否，1: 中高风险地区，3: 其他有病例社区的发热或呼吸道症状患者。
  pub sfjcry: Option<i32>,

  /// 简单描述此类接触的具体情况。
  pub sfjcrysm: Option<String>,

  /// 近 15 天您或家属是否接触过疑似或确诊患者，或无症状感染患者（核酸检测阳性者）。
  /// 0: 否，1: 是。
  pub sfjcqz: Option<i32>,

  /// 简单描述此类接触的具体情况。
  pub sfjcqzsm: Option<String>,

  /// 您或家属是否正根据上级单位、医院相关要求进行居家或封闭性隔离。
  /// 0: 正常学习生活，1: 居家（宿舍）隔离，2: 在校医院隔离，3: 定点医院隔离，9: 指定酒店/宿舍/特定区域隔离。
  pub glqk: Option<i32>,

  /// 隔离开始日期。
  pub glksrq: Option<String>,
  
  /// 隔离结束日期。
  pub gljsrq: Option<String>,
  
  /// 隔离原因。
  pub glyy: Option<String>,

  /// 您或家属当前健康状态。
  /// 0: 正常，2: 疑似，3: 确诊，4: 已治愈，5: 疑似转排除（留观）。
  pub sfjkqk: Option<i32>,

  /// 简单描述一下您或家属的健康情况。
  pub jkqksm: Option<String>,

  /// 不知道什么玩意儿，缺省为空。
  pub sfmtbg: Option<String>,

  /// 不知道什么玩意儿，缺省为 3。
  pub qrlxzt: Option<i32>,

  /// 学院名称，如“信息类”。
  pub xymc: Option<String>,

  /// 学生手机号码。
  pub xssjhm: Option<String>,
}
