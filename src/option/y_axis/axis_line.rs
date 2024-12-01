mod builder;

use builder::AxisLineBuilder;
use serde::Serialize;

#[derive(Serialize, Clone, Debug, Default)]
pub struct AxisLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>
}

impl AxisLine {
    pub fn new() -> Self {
        Self {
            show: Some(true)
        }
    }
}
impl AxisLine {
    pub fn builder() -> AxisLineBuilder {
        AxisLineBuilder::default()
    }
}