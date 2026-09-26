#[allow(unused_imports)]
use super::OptionEnum;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OptionInputStruct {
    pub a: Option<u8>,
    pub b: Option<char>,
    pub c: Option<OptionEnum>,
}
