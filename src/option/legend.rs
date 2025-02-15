mod builder;
pub mod top_text;

use builder::LegendBuilder;
use serde::Serialize;
use top_text::TopText;

#[derive(Serialize, Default)]
pub struct Legend {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "top")]
    top_text: Option<TopText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<String>>
}
impl Legend {
    pub fn new(data: Vec<&str>) -> Self {
        Self {
            data: Some(data.iter().map(|item| item.to_string()).collect()),
            ..Self::default()
        }
    }
    pub fn builder() -> LegendBuilder {
        LegendBuilder::default()
    }
}