use serde::Serialize;

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