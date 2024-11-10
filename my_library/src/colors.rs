//! Terminal color formatting utilities

/// Returns string wrapped in ANSI red color codes
///
/// # Examples
/// ```
/// use my_library::colors::red;
/// assert_ne!(red("error"), "error");
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

/// Returns string wrapped in ANSI green color codes
///
/// # Examples
/// ```
/// use my_library::colors::green;
/// assert_ne!(green("success"), "success");
/// ```
pub fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}

/// Returns string wrapped in ANSI yellow color codes
///
/// # Examples
/// ```
/// use my_library::colors::yellow;
/// assert_ne!(yellow("warning"), "warning");
/// ```
pub fn yellow(s: &str) -> String {
    format!("\x1b[33m{}\x1b[0m", s)
}

/// Returns string wrapped in ANSI blue color codes
///
/// # Examples
/// ```
/// use my_library::colors::blue;
/// assert_ne!(blue("info"), "info");
/// ```
pub fn blue(s: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", s)
}

/// Returns string wrapped in ANSI bold codes
///
/// # Examples
/// ```
/// use my_library::colors::bold;
/// assert_ne!(bold("important"), "important");
/// ```
pub fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

/// Returns string wrapped in ANSI underline codes
///
/// # Examples
/// ```
/// use my_library::colors::underline;
/// assert_ne!(underline("underlined"), "underlined");
/// ```
pub fn underline(s: &str) -> String {
    format!("\x1b[4m{}\x1b[0m", s)
}