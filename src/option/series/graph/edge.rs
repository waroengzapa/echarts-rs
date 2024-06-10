use serde::Serialize;

use super::line_style::LineStyle;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    source: String,
    target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_style: Option<LineStyle>
}
impl Edge {
    pub fn new(source: String, target: String) -> Self {
        Self { 
            source, 
            target, 
            line_style: None 
        }
    }
    pub fn set_line_style(&mut self, line_style: LineStyle) {
        self.line_style = Some(line_style);
    }
}