pub mod option;
pub mod theme;

use std::{fs::File, path::Path};
use handlebars::Handlebars;
use option::EChartOption;
use serde::Serialize;
use serde_json::json;
use theme::Theme;

#[derive(Serialize)]
pub struct EChart {
    title: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    theme: Option<String>,
    option: String
}
impl EChart {
    pub fn init(title: &'static str, theme: Option<Theme>, option: EChartOption) -> Self {
        Self {
            title,
            theme: theme.map(|theme| theme.to_string()),
            option: serde_json::to_string(&option).unwrap(),
        }
    }
    pub fn render_to_html(&self, path: &Path) -> Result<(), handlebars::RenderError>{
        Handlebars::new()
            .render_template_to_write(
                include_str!("template/template.hbs"), 
                &json!(self), 
                File::create(path).unwrap()
            )
    }
}