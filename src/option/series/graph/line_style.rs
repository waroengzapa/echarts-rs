pub mod type_;

use serde::Serialize;

use self::type_::Type;

#[derive(Serialize, Clone, Copy, Default, Debug)]
pub struct LineStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<u8>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    curveness: Option<f32>,
}
impl LineStyle {
    pub fn set_width(&mut self, width: u8) {
        self.width =  Some(width);
    }
    pub fn set_type(&mut self, type_: Type) {
        self.type_ = Some(type_);
    }
    pub fn set_curveness(&mut self, curveness: f32) {
        self.curveness = Some(curveness);
    }
}