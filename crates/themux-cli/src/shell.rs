// Shell quoting and word splitting (POSIX shell rules).
//
// Used by the tmux compatibility layer for constructing remote commands.

/// Quote a string for safe use in a POSIX shell command.
pub fn shell_quote(s: &str) -> String {
    if s.is_empty() {
        return "''".to_string();
    }
    // If the string is "safe" (no special chars), return as-is
    if s.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.' || c == '/') {
        return s.to_string();
    }
    // Otherwise, single-quote it, escaping any embedded single quotes
    format!("'{}'", s.replace('\'', "'\\''"))
}
