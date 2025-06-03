use core::fmt;
use core::fmt::Write;

/// Indent all but the nonempty lines
pub(crate) fn indent_trimmed(s: impl fmt::Display, width: usize) -> Result<String, fmt::Error> {
    let mut indented = String::new();
    for _ in 0..width {
        indented.push(' ');
    }

    prefix_trimmed(s, &indented)
}

/// Apply a prefix to each line, trimming trailing whitespace
pub(crate) fn prefix_trimmed(s: impl fmt::Display, prefix: &str) -> Result<String, fmt::Error> {
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
