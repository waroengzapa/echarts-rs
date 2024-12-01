use super::legend::Legend;
use super::series::Series;
use super::title::Title;
use super::tooltip::Tooltip;
use super::x_axis::XAxis;
use super::y_axis::YAxis;
use crate::option::option;

#[derive(Default)]
pub struct OptionBuilder {
    title: option::Option<Title>,
    legend: option::Option<Legend>,
    x_axis: option::Option<XAxis>,
    y_axis: option::Option<Vec<YAxis>>,
    tooltip: option::Option<Tooltip>,
    series: Vec<Series>
}
impl OptionBuilder {
    pub fn build(self) -> super::Option {
        super::Option { 
            title: self.title,
            legend: self.legend,
            x_axis: self.x_axis, 
            y_axis: self.y_axis,
            tooltip: self.tooltip,
            series: self.series
        }
    }
    pub fn title(mut self, title: Title) -> Self {
        self.title = Some(title);
        self
    }
    pub fn legend(mut self, legend: Legend) -> Self {
        self.legend = Some(legend);
        self
    }
    pub fn x_axis(mut self, x_axis: XAxis) -> Self {
        self.x_axis = Some(x_axis);
        self
    }
    pub fn y_axis(mut self, y_axis: Vec<YAxis>) -> Self {
        self.y_axis = Some(y_axis);
        self
    }
    pub fn tooltip(mut self, tooltip: Tooltip) -> Self {
        self.tooltip = Some(tooltip);
        self
    }
    pub fn series(mut self, series: Vec<Series>) -> Self {
        self.series = series;
        self
    }
}