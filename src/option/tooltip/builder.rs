use super::{trigger::Trigger, Tooltip};

#[derive(Default)]
pub struct TooltipBuilder {
    trigger: Option<Trigger>
}

impl TooltipBuilder {
    pub fn build(&self) -> Tooltip {
        Tooltip { 
            trigger: self.trigger.clone()
        }
    }
    pub fn trigger(mut self, trigger: Trigger) -> Self {
        self.trigger = Some(trigger);
        self
    }
}