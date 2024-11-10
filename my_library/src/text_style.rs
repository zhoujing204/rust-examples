//! Text styling module providing ANSI color and formatting capabilities.

/// Represents various foreground and background colors for text styling
///
/// # Examples
/// ```
/// use my_library::text_style::{style, Color};
///
/// // Foreground colors
/// let red_text = style("Error").color(Color::Red).build();
///
/// // Background colors
/// let highlighted = style("Important").color(Color::BgYellow).build();
/// ```
#[derive(Default)]
pub enum Color {
    /// Standard foreground colors
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    /// Background colors
    BgRed,
    BgGreen,
    BgYellow,
    BgBlue,
    BgMagenta,
    BgCyan,
    BgWhite,
    /// Default color (no color modification)
    #[default]
    None,
}

/// A builder struct for creating styled text with various formatting options
///
/// # Examples
/// ```
/// use my_library::text_style::{ColorString, Color};
///
/// let styled = ColorString::new("Important message")
///     .bold()
///     .color(Color::Red)
///     .underline()
///     .build();
/// ```
#[derive(Default)]
pub struct ColorString {
    content: String,
    bold: bool,
    italic: bool,
    underline: bool,
    blink: bool,
    color: Option<Color>,
}

impl ColorString {
    /// Creates a new ColorString instance with the given content
    ///
    /// # Examples
    /// ```
    /// use my_library::text_style::ColorString;
    ///
    /// let styled = ColorString::new("Hello")
    ///     .bold()
    ///     .build();
    /// assert!(styled.contains("\x1b[1m"));
    /// ```
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }

    /// Adds bold styling to the text
    ///
    /// # Examples
    /// ```
    /// use my_library::text_style::style;
    ///
    /// let bold_text = style("Important").bold().build();
    /// assert!(bold_text.contains("\x1b[1m"));
    /// ```
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Adds color to the text
    ///
    /// # Examples
    /// ```
    /// use my_library::text_style::{style, Color};
    ///
    /// let error = style("Error").color(Color::Red).build();
    /// assert!(error.contains("\x1b[31m"));
    ///
    /// let highlight = style("Important").color(Color::BgYellow).build();
    /// assert!(highlight.contains("\x1b[43m"));
    /// ```
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Adds underline styling to the text
    ///
    /// # Examples
    /// ```
    /// use my_library::text_style::style;
    ///
    /// let underlined = style("Emphasis").underline().build();
    /// assert!(underlined.contains("\x1b[4m"));
    /// ```
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    /// Adds italic styling to the text
    ///
    /// # Examples
    /// ```
    /// use my_library::text_style::style;
    ///
    /// let italicized = style("Quoted").italic().build();
    /// assert!(italicized.contains("\x1b[3m"));
    /// ```
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Adds blinking effect to the text
    ///
    /// # Examples
    /// ```
    /// use my_library::text_style::style;
    ///
    /// let blinking = style("Alert!").blink().build();
    /// assert!(blinking.contains("\x1b[5m"));
    /// ```
    pub fn blink(mut self) -> Self {
        self.blink = true;
        self
    }

    /// Builds the final styled string with all applied formatting
    ///
    /// # Examples
    /// ```
    /// use my_library::text_style::{style, Color};
    ///
    /// // Combine multiple styles
    /// let styled = style("Warning")
    ///     .bold()
    ///     .color(Color::Yellow)
    ///     .underline()
    ///     .build();
    ///
    /// assert!(styled.contains("\x1b["));
    /// assert!(styled.contains("Warning"));
    ///
    /// // Simple styling
    /// let basic = style("Normal").build();
    /// assert_eq!(basic, "Normal");
    /// ```
    pub fn build(self) -> String {
        let mut format_codes = Vec::new();

        if self.bold {
            format_codes.push("1");
        }
        if self.italic {
            format_codes.push("3");
        }
        if self.underline {
            format_codes.push("4");
        }
        if self.blink {
            format_codes.push("5");
        }

        if let Some(color) = self.color {
            format_codes.push(match color {
                Color::Red => "31",
                Color::Green => "32",
                Color::Yellow => "33",
                Color::Blue => "34",
                Color::Magenta => "35",
                Color::Cyan => "36",
                Color::White => "37",
                Color::BgRed => "41",
                Color::BgGreen => "42",
                Color::BgYellow => "43",
                Color::BgBlue => "44",
                Color::BgMagenta => "45",
                Color::BgCyan => "46",
                Color::BgWhite => "47",
                Color::None => "",
            });
        }

        if format_codes.is_empty() {
            self.content
        } else {
            format!("\x1b[{}m{}\x1b[0m",
                format_codes.join(";"),
                self.content
            )
        }
    }

    /// Resets all styling options back to their default values while preserving the content
    ///
    /// # Examples
    /// ```
    /// use my_library::text_style::{style, Color};
    ///
    /// let mut styled = style("Test")
    ///     .bold()
    ///     .color(Color::Red)
    ///     .underline();
    ///
    /// styled = styled.reset();
    /// assert_eq!(styled.build(), "Test");
    /// ```
    pub fn reset(mut self) -> Self {
        self.bold = false;
        self.italic = false;
        self.underline = false;
        self.blink = false;
        self.color = None;
        self
    }
}

/// Creates a new style builder for text formatting
///
/// This is a convenience function that creates a new [`ColorString`] instance.
///
/// # Examples
/// ```
/// use my_library::text_style::{style, Color};
///
/// // Error message
/// let error = style("Error")
///     .color(Color::Red)
///     .bold()
///     .build();
///
/// // Success message
/// let success = style("Success")
///     .color(Color::Green)
///     .build();
///
/// // Warning with background
/// let warning = style("Warning")
///     .color(Color::BgYellow)
///     .bold()
///     .build();
/// ```
pub fn style(content: impl Into<String>) -> ColorString {
    ColorString::new(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bold() {
        let styled = style("test").bold().build();
        assert!(styled.contains("\x1b[1m"));
    }

    #[test]
    fn test_color() {
        let styled = style("test").color(Color::Red).build();
        assert!(styled.contains("\x1b[31m"));
    }

    #[test]
    fn test_multiple_styles() {
        let styled = style("test")
            .bold()
            .color(Color::Blue)
            .italic()
            .build();
        assert!(styled.contains("1;3;34"));
    }

    #[test]
    fn test_no_formatting() {
        let styled = style("test").build();
        assert_eq!(styled, "test");
    }

    #[test]
    fn test_background_color() {
        let styled = style("test").color(Color::BgRed).build();
        assert!(styled.contains("\x1b[41m"));
    }

    #[test]
    fn test_reset() {
        let styled = style("test")
            .bold()
            .color(Color::Red)
            .italic()
            .reset()
            .build();
        assert_eq!(styled, "test");
    }

    #[test]
    fn test_reset_and_restyle() {
        let styled = style("test")
            .bold()
            .color(Color::Red)
            .reset()
            .color(Color::Blue)
            .build();
        assert!(styled.contains("\x1b[34m"));
        assert!(!styled.contains("\x1b[1m"));
        assert!(!styled.contains("\x1b[31m"));
    }

}
