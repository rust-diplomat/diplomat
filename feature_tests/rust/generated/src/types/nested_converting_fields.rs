#[allow(unused_imports)]
use super::NestedCharField;
#[allow(unused_imports)]
use super::NestedOptionField;
use crate::ffi;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NestedConvertingFields {
    pub char_inner: NestedCharField,
    pub option_inner: NestedOptionField,
}
impl NestedConvertingFields {
    pub fn new() -> NestedConvertingFields {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::NestedConvertingFields_new() };
        NestedConvertingFields {
            char_inner: NestedCharField {
                ch: crate::private::char_from_u32(result.char_inner.ch),
            },
            option_inner: NestedOptionField {
                value: result.option_inner.value.into_option(),
            },
        }
    }
    pub fn round_trip(value: NestedConvertingFields) -> NestedConvertingFields {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::NestedConvertingFields_round_trip(ffi::NestedConvertingFields {
                char_inner: ffi::NestedCharField {
                    ch: value.char_inner.ch as u32,
                },
                option_inner: ffi::NestedOptionField {
                    value: ffi::DiplomatOption::from(value.option_inner.value),
                },
            })
        };
        NestedConvertingFields {
            char_inner: NestedCharField {
                ch: crate::private::char_from_u32(result.char_inner.ch),
            },
            option_inner: NestedOptionField {
                value: result.option_inner.value.into_option(),
            },
        }
    }
}
