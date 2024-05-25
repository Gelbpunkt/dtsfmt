use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum IndentationType {
    #[serde(rename = "tabs")]
    Tabs,
    #[serde(rename = "two-spaces")]
    TwoSpaces,
    #[serde(rename = "four-spaces")]
    FourSpaces,
}

impl IndentationType {
    pub fn indent(&self) -> &'static str {
        match self {
            Self::Tabs => "\t",
            Self::TwoSpaces => "  ",
            Self::FourSpaces => "    ",
        }
    }
}
