use serde::Serialize;
                // use super::line_style::LineStyle;

// #[derive(Serialize, Clone, Debug)]
#[derive(Serialize, Clone, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct Edge {
    pub source: NumberOrString,
    pub target: NumberOrString,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // line_style: Option<LineStyle>
}
impl Edge {
    pub fn new(source: NumberOrString, target: NumberOrString) -> Self {
        Self { 
            source, 
            target, 
            // line_style: None 
        }
    }
    // pub fn set_line_style(&mut self, line_style: LineStyle) {
    //     self.line_style = Some(line_style);
    // }
}


#[derive(Clone, Debug)]
pub enum NumberOrString {
    Number(Number),
    String(String)
}
impl Serialize for NumberOrString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match self {
            NumberOrString::Number(number) => number.serialize(serializer),
            NumberOrString::String(string) => string.serialize(serializer),
        }
    }
}
#[derive(Clone, Debug)]
pub enum Number {
    F32(f32),
    F64(f64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize)
}
impl Serialize for Number {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        match self {
            Number::F32(number) => number.serialize(serializer),
            Number::F64(number) => number.serialize(serializer),
            Number::I8(number) => number.serialize(serializer),
            Number::I16(number) => number.serialize(serializer),
            Number::I32(number) => number.serialize(serializer),
            Number::I64(number) => number.serialize(serializer),
            Number::I128(number) => number.serialize(serializer),
            Number::ISize(number) => number.serialize(serializer),
            Number::U8(number) => number.serialize(serializer),
            Number::U16(number) => number.serialize(serializer),
            Number::U32(number) => number.serialize(serializer),
            Number::U64(number) => number.serialize(serializer),
            Number::U128(number) => number.serialize(serializer),
            Number::USize(number) => number.serialize(serializer)
        }
    }
}