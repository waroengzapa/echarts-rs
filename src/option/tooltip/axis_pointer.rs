pub mod type_;
pub mod builder;

use builder::AxisPointerBuilder;
use serde::Serialize;
use type_::Type;

#[derive(Clone)]
#[derive(Serialize)]
pub struct AxisPointer {
    type_: Option<Type>
}
impl AxisPointer {
    pub fn builder() -> AxisPointerBuilder {
        AxisPointerBuilder::default()
    }
}