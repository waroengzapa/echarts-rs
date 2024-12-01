mod builder;
pub mod title;
pub mod legend;
pub mod x_axis;
pub mod y_axis;
pub mod tooltip;
pub mod series;

use std::option;
use legend::Legend;
use builder::OptionBuilder;
use serde::Serialize;
use series::Series;
use title::Title;
use tooltip::Tooltip;
use x_axis::XAxis;
use y_axis::YAxis;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Option {
    #[serde(skip_serializing_if = "option::Option::is_none")]
    title: option::Option<Title>,
    #[serde(skip_serializing_if = "option::Option::is_none")]
    legend: option::Option<Legend>,
    #[serde(skip_serializing_if = "option::Option::is_none")]
    x_axis: option::Option<XAxis>,
    #[serde(skip_serializing_if = "option::Option::is_none")]
    y_axis: option::Option<Vec<YAxis>>,
    #[serde(skip_serializing_if = "option::Option::is_none")]
    tooltip: option::Option<Tooltip>,
    series: Vec<Series>
}

impl Option {
    pub fn new(series: Vec<Series>) -> Self {
        Self {
            title: None,
            legend: None,
            x_axis: None,
            y_axis: None,
            tooltip: None,
            series
        }
    }
    pub fn builder() -> OptionBuilder {
        OptionBuilder::default()
    }
}