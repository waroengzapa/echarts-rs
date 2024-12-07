pub mod builder;
pub mod data;
pub mod end_label;

use data::Data;
use end_label::EndLabel;
use builder::LineBuilder;
use serde::Serialize;

#[derive(Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    #[serde(rename = "type")]
    type_: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y_axis_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_label: Option<EndLabel>,
    data: Vec<Data>
}
impl Line {
    pub fn new(data: Vec<Data>) -> Self {
        Self { 
            type_: "line", 
            data,
            ..Self::default()
        }
    }
    pub fn builder() -> LineBuilder {
        LineBuilder::default()
    }
}