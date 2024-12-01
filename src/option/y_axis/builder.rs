use super::{axis_label::AxisLabel, axis_line::AxisLine, position::Position, type_::Type, YAxis};

#[derive(Default)]
pub struct YAxisBuilder {
    position: Option<Position>,
    offset: Option<usize>,
    type_: Option<Type>,
    name: Option<String>,
    scale: Option<bool>,
    axis_line: Option<AxisLine>,
    axis_label: Option<AxisLabel>
}
impl YAxisBuilder {
    pub fn build(self) -> YAxis {
        YAxis { 
            position: self.position,
            offset: self.offset,
            type_: self.type_,
            name: self.name,
            scale: self.scale,
            axis_line: self.axis_line,
            axis_label: self.axis_label
        }
    }
    pub fn position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn type_(mut self, type_: Type) -> Self {
        self.type_ = Some(type_);
        self
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
    pub fn scale(mut self, scale: bool) -> Self {
        self.scale = Some(scale);
        self
    }
    pub fn axis_line(mut self, axis_line: AxisLine) -> Self {
        self.axis_line = Some(axis_line);
        self
    }
    pub fn axis_label(mut self, axis_label: AxisLabel) -> Self {
        self.axis_label = Some(axis_label);
        self
    }
}