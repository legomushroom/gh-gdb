/// Adds a new line to a string.
#[cfg(test)]
pub fn with_end_new_line<S: AsRef<str>>(string: S) -> String {
    return format!("{}\n", string.as_ref());
}
