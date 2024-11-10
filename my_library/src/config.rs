/// Logging level enumeration
///
/// Available levels from least to most severe:
/// - Debug
/// - Info
/// - Warn
/// - Error
#[derive(Debug, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// Output destination for logs
///
/// # Examples
/// ```
/// use my_library::config::LogOutput;
///
/// let stdout = LogOutput::StdOut;
/// let stderr = LogOutput::StdErr;
/// let file = LogOutput::File("app.log".to_string());
/// ```
#[derive(Debug, PartialEq)]
pub enum LogOutput {
    StdOut,
    StdErr,
    File(String)
}

/// Configuration struct for logging settings
///
/// # Examples
/// ```
/// use my_library::config::{Logging, LogLevel, LogOutput};
///
/// // Create with default settings
/// let default_config = Logging::new();
/// assert_eq!(default_config.enabled, false);
/// assert_eq!(default_config.level, LogLevel::Info);
/// assert_eq!(default_config.destination, LogOutput::StdOut);
///
/// // Create with custom settings
/// let custom_config = Logging {
///     enabled: true,
///     level: LogLevel::Debug,
///     destination: LogOutput::File("debug.log".to_string())
/// };
/// ```
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput
}

impl Logging {
    /// Creates a new Logging configuration with default settings
    ///
    /// Default configuration:
    /// - enabled: false
    /// - level: Info
    /// - destination: StdOut
    ///
    /// # Examples
    /// ```
    /// use my_library::config::Logging;
    ///
    /// let config = Logging::new();
    /// assert!(!config.enabled);  // logging is disabled by default
    /// ```
    pub fn new() -> Self {
        Logging {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::StdOut
        }
    }
}