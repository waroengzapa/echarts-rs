pub mod builder;
pub mod trigger;
pub mod axis_pointer;

use axis_pointer::AxisPointer;
use serde::Serialize;
use builder::TooltipBuilder;
use trigger::Trigger;   

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tooltip {
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    axis_pointer: Option<AxisPointer>
}
impl Tooltip {
    pub fn builder() -> TooltipBuilder {
        TooltipBuilder::default()
    }
}

