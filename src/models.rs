use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct SubmitSm {
    #[serde(default)]
    pub field1: String,

    #[serde(default = "default_field2")]
    pub field2: i32,
}

fn default_field2() -> i32 {
    42 // default value for field2
}