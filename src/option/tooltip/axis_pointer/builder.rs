use super::{type_::Type, AxisPointer};

#[derive(Default)]
pub struct AxisPointerBuilder {
    type_: Option<Type>
}

impl AxisPointerBuilder {
    pub fn build(&self) -> AxisPointer {
        AxisPointer { 
            type_: self.type_.clone()
        }
    }
    pub fn type_(mut self, type_: Type) -> AxisPointerBuilder {
        self.type_ = Some(type_);
        self
    }
}