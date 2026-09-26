#[allow(unused_imports)]
use super::FloatField;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NestedFloatField {
    pub inner: FloatField,
}
