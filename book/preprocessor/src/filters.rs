#[askama::filter_fn]
pub(crate) fn checked(value: &bool, _env: &dyn askama::Values) -> askama::Result<String> {
    if *value {
        Ok("X".to_string())
    } else {
        Ok(" ".to_string())
    }
}
