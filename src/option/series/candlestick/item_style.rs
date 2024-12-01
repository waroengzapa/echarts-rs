use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color0: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_color0: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_width: Option<u8>
}
impl ItemStyle {
    pub fn builder() -> ItemStyleBuilder {
        ItemStyleBuilder::default()
    }
}

#[derive(Default)]

pub struct ItemStyleBuilder {
    color: Option<String>,
    color0: Option<String>,
    border_color: Option<String>,
    border_color0: Option<String>,
    border_width: Option<u8>
}   
impl <'a>ItemStyleBuilder { 
    pub fn build(self) -> ItemStyle {
        ItemStyle {
            color: self.color,
            color0: self.color0,
            border_color: self.border_color,
            border_color0: self.border_color0,
            border_width: self.border_width
        }
    }
    pub fn color(mut self, color: &'a str) -> Self {
        self.color = Some(color.to_owned());
        self
    }
    pub fn color0(mut self, color: &'a str) -> Self {
        self.color0 = Some(color.to_owned());
        self
    }
    pub fn border_color(mut self, color: &'a str) -> Self {
        self.border_color = Some(color.to_owned());
        self
    }
    pub fn border_color0(mut self, color: &'a str) -> Self {
        self.border_color0 = Some(color.to_owned());
        self
    }
}