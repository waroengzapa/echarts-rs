mod builder;
pub mod left_text;

use left_text::LeftText;
use serde::Serialize;
use builder::TitleBuilder;

#[derive(Serialize, Default)]
pub struct Title {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    subtext: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "left")]
    left_text: Option<LeftText>
}
impl Title {
    pub fn new(text: &str) -> Self {
        Self { 
            text: text.to_string(),
            ..Self::default()
        }
    }
    pub fn builder() -> TitleBuilder {
        TitleBuilder::default()
    }
}