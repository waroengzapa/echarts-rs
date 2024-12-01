use crate::option::x_axis::type_::Type;
use super::XAxis;

#[derive(Default)]
pub struct XAxisBuilder {
    type_: Option<Type>,
    // name: Option<String>,
    // data: Option<Vec<String>>
}

impl XAxisBuilder {
    pub fn build(&self) -> XAxis {
        XAxis { 
            type_: self.type_.clone(),
            // name: self.name.clone(),
            // data: self.data.clone()
        }
    }
    pub fn type_(mut self, type_: Type) -> Self {
        self.type_ = Some(type_);
        self
    }
    // pub fn name(mut self, data: &str) -> Self {
    //     self.name = Some(data.to_owned());
    //     self
    // }
    // pub fn data(mut self, data: Vec<String>) -> Self {
    //     self.data = Some(data);
    //     self
    // }
}