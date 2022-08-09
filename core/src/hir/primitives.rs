//! Primitives types that can cross the FFI boundary.

/// 8, 16, 32, and 64-bit signed and unsigned integers.
#[derive(Copy, Clone)]
pub enum IntType {
    I8,
    I16,
    I32,
    I64,
    Isize,
    U8,
    U16,
    U32,
    U64,
    Usize,
}

/// 128-bit signed and unsigned integers.
#[derive(Copy, Clone)]
pub enum Int128Type {
    I128,
    U128,
}

/// 32 and 64-bit floating point numbers.
#[derive(Copy, Clone)]
pub enum FloatType {
    F32,
    F64,
}

/// Primitive types.
#[derive(Copy, Clone)]
pub enum PrimitiveType {
    Bool,
    Char,
    Int(IntType),
    Int128(Int128Type),
    Float(FloatType),
    Unit,
}

impl IntType {
    pub fn as_str(&self) -> &'static str {
        match self {
            IntType::I8 => "i8",
            IntType::I16 => "i16",
            IntType::I32 => "i32",
            IntType::I64 => "i64",
            IntType::Isize => "isize",
            IntType::U8 => "u8",
            IntType::U16 => "u16",
            IntType::U32 => "u32",
            IntType::U64 => "u64",
            IntType::Usize => "usize",
        }
    }
}

impl Int128Type {
    pub fn as_str(&self) -> &'static str {
        match self {
            Int128Type::I128 => "i128",
            Int128Type::U128 => "u128",
        }
    }
}

impl FloatType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FloatType::F32 => "f32",
            FloatType::F64 => "f64",
        }
    }
}

impl PrimitiveType {
    pub fn as_str(&self) -> &'static str {
        match self {
            PrimitiveType::Bool => "bool",
            PrimitiveType::Char => "char",
            PrimitiveType::Int(ty) => ty.as_str(),
            PrimitiveType::Int128(ty) => ty.as_str(),
            PrimitiveType::Float(ty) => ty.as_str(),
            PrimitiveType::Unit => "()",
        }
    }
}
