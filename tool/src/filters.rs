use core::fmt;
use core::fmt::Write;

use askama::Values;

/// Indent all but the nonempty lines
#[askama::filter_fn]
pub(crate) fn indent_trimmed(
    s: impl fmt::Display,
    values: &dyn Values,
    width: usize,
) -> Result<String, fmt::Error> {
    let mut indented = String::new();
    for _ in 0..width {
        indented.push(' ');
    }

    let p : prefix_trimmed::<'_, true> = prefix_trimmed::default().with_prefix(&indented);
    prefix_trimmed::execute(p, s, values)
}

/// Apply a prefix to each line, trimming trailing whitespace
#[askama::filter_fn]
pub(crate) fn prefix_trimmed(
    s: impl fmt::Display,
    _values: &dyn Values,
    prefix: &str,
) -> Result<String, fmt::Error> {
    let s = s.to_string();
    let mut result = String::new();
    let prefix_trimmed = prefix.trim_end();

    for line in s.lines() {
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            writeln!(&mut result, "{prefix_trimmed}")?;
        } else {
            writeln!(&mut result, "{prefix}{trimmed}")?;
        }
    }
    // The writelns will end up with a trailing \n
    if let Some(c) = result.chars().next_back() {
        if c == '\n' {
            result.pop();
        }
    }

    Ok(result)
}
