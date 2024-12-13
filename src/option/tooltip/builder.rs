use super::{axis_pointer::AxisPointer, trigger::Trigger, Tooltip};

#[derive(Default)]
pub struct TooltipBuilder {
    trigger: Option<Trigger>,
    axis_pointer: Option<AxisPointer>
}

impl TooltipBuilder {
    pub fn build(&self) -> Tooltip {
        Tooltip { 
            trigger: self.trigger.clone(),
            axis_pointer: self.axis_pointer.clone()
        }
    }
    pub fn trigger(mut self, trigger: Trigger) -> Self {
        self.trigger = Some(trigger);
        self
    }
    pub fn axis_pointer(mut self, axis_pointer: AxisPointer) -> Self {
        self.axis_pointer = Some(axis_pointer);
        self
    }
}