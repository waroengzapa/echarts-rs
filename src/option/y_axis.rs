mod builder;
pub mod position;
mod type_;
pub mod axis_line;
pub mod axis_label;

use axis_label::AxisLabel;
use axis_line::AxisLine;
use position::Position;
use serde::Serialize;
use type_::Type;
use builder::YAxisBuilder;

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct YAxis {
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    type_: Option<Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    axis_line: Option<AxisLine>,
    #[serde(skip_serializing_if = "Option::is_none")]
    axis_label: Option<AxisLabel>
}

impl YAxis {
    pub fn builder() -> YAxisBuilder {
        YAxisBuilder::default()
    }
}
