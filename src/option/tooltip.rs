pub mod builder;
pub mod trigger;

use serde::Serialize;
use builder::TooltipBuilder;
use trigger::Trigger;   

#[derive(Default, Serialize)]
pub struct Tooltip {
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger: Option<Trigger>
}
impl Tooltip {
    pub fn builder() -> TooltipBuilder {
        TooltipBuilder::default()
    }
}

