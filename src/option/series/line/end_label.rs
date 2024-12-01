mod builder;

use builder::EndLabelBuilder;
use serde::Serialize;

#[derive(Serialize, Clone, Debug, Default)]
pub struct EndLabel {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    formatter: Option<String>}
impl EndLabel {
    pub fn new() -> Self {
        Self {
            show: Some(true),
            ..Self::default()
            // formatter: Some("{@[1]}\n{@[0]}".to_string())
        }
    }
}
impl EndLabel {
    pub fn builder() -> EndLabelBuilder {
        EndLabelBuilder::default()
    }
}