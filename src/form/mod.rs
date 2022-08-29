mod at_school;
mod other;

use serde::{Deserialize, Serialize};

pub use at_school::FormAtSchool;
pub use other::FormOther;

#[derive(Deserialize, Serialize)]
pub struct Form {
  pub at_school: Option<FormAtSchool>,
  pub other: Option<FormOther>,
}
