pub mod option;
pub mod theme;

pub extern crate handlebars;

use std::{fs::File, path::Path};
use handlebars::{Handlebars, RenderError};
use serde::Serialize;
use serde_json::json;
use theme::Theme;

#[derive(Serialize)]
pub struct EChart {
    pub id: &'static str,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub width: u16,
    pub height: u16,
    pub theme: Theme,
    // pub theme: String,
    pub option: option::Option
    // pub option: String,
}


impl EChart {
    // #[allow(dead_code)]
    // pub fn init(id: &'static str, theme: Theme, option: option::Option) -> Self {
    //     Self {
    //         id,
    //         theme: theme.to_string(),
    //         option: serde_json::to_string(&option).unwrap(),
    //     }
    // }
    pub fn render_template_to_write(&self, path: &Path) -> Result<(), RenderError> {
        let sad = EChartHandlebars {
            id: self.id,
            width: self.width,
            height: self.height,
            theme: self.theme.to_string(),
            // option: serde_json::to_string_pretty(&self.option).unwrap()
            option: serde_json::to_string(&self.option).unwrap()
        };
        Handlebars::new().render_template_to_write(include_str!("template/template.hbs"), &json!(&sad), File::create(path).unwrap())
        // Handlebars::new().render_templ(include_str!("template/template.hbs"), &json!(&sad), File::create(path).unwrap())
    }
    pub fn render_template(&self) -> Result<String, RenderError> {
        let sad = EChartHandlebars {
            id: self.id,
            width: self.width,
            height: self.height,
            theme: self.theme.to_string(),
            // option: serde_json::to_string_pretty(&self.option).unwrap()
            option: serde_json::to_string(&self.option).unwrap()
        };
        Handlebars::new().render_template(include_str!("template/template.hbs"), &json!(&sad))
        // Handlebars::new().render_templ(include_str!("template/template.hbs"), &json!(&sad), File::create(path).unwrap())
    }
}

#[derive(Serialize)]
struct EChartHandlebars {
    id: &'static str,
    width: u16,
    height: u16,
    theme: String,
    option: String
}