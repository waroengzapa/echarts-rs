mod builder;
pub mod left_text;

use left_text::LeftText;
use serde::Serialize;
use builder::TitleBuilder;

#[derive(Serialize, Default, Clone, Debug)]
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
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        // self
    }
    pub fn set_subtext(&mut self, text: &str) {
        self.subtext = Some(text.to_string());
        // self
    }
}