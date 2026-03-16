#[askama::filter_fn]
pub(crate) fn checked(value : &bool, _env : &dyn askama::Values) -> askama::Result<String> {
    if *value {
        Ok(format!("X"))
    } else {
        Ok(format!(" "))
    }
}