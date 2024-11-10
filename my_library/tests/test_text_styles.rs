use my_library::text_style::{style, Color, ColorString};

// Basic Functionality Tests
#[test]
fn test_basic_styling() {
    let styled = style("test").build();
    assert_eq!(styled, "test");
}

// Color Tests
#[test]
fn test_foreground_colors() {
    let colors = [
        (Color::Red, "31"),
        (Color::Green, "32"),
        (Color::Blue, "34"),
        (Color::Yellow, "33"),
        (Color::Magenta, "35"),
        (Color::Cyan, "36"),
        (Color::White, "37"),
    ];

    for (color, code) in colors {
        let styled = style("test").color(color).build();
        assert!(styled.contains(&format!("\x1b[{}m", code)));
    }
}

#[test]
fn test_background_colors() {
    let colors = [
        (Color::BgRed, "41"),
        (Color::BgGreen, "42"),
        (Color::BgBlue, "44"),
        (Color::BgYellow, "43"),
        (Color::BgMagenta, "45"),
        (Color::BgCyan, "46"),
        (Color::BgWhite, "47"),
    ];

    for (color, code) in colors {
        let styled = style("test").color(color).build();
        assert!(styled.contains(&format!("\x1b[{}m", code)));
    }
}

// Formatting Tests
#[test]
fn test_text_formatting() {
    let bold = style("test").bold().build();
    assert!(bold.contains("\x1b[1m"));

    let italic = style("test").italic().build();
    assert!(italic.contains("\x1b[3m"));

    let underline = style("test").underline().build();
    assert!(underline.contains("\x1b[4m")); // Fixed: changed 'bold' to 'underline'

    let blink = style("test").blink().build();
    assert!(blink.contains("\x1b[5m"));
}

// Combined Styling Tests
#[test]
fn test_combined_styles() {
    let styled = style("test")
        .bold()
        .italic()
        .color(Color::Red)
        .build();

    assert!(styled.contains("1;3;31"));
    assert!(styled.starts_with("\x1b["));
    assert!(styled.ends_with("\x1b[0m"));
}

// Reset Tests
#[test]
fn test_reset_functionality() {
    let styled = style("test")
        .bold()
        .color(Color::Red)
        .reset()
        .build();

    assert_eq!(styled, "test");
}

#[test]
fn test_reset_and_new_style() {
    let styled = style("test")
        .bold()
        .color(Color::Red)
        .reset()
        .italic()
        .color(Color::Blue)
        .build();

    assert!(styled.contains("\x1b[3;34m"));
    assert!(!styled.contains("\x1b[1m"));
    assert!(!styled.contains("\x1b[31m"));
}

// Integration Tests
#[test]
fn test_complex_styling_scenarios() {
    // Error message styling
    let error = style("Error")
        .color(Color::Red)
        .bold()
        .build();
    assert!(error.contains("\x1b[1;31m"));

    // Success message styling
    let success = style("Success")
        .color(Color::Green)
        .bold()
        .build();
    assert!(success.contains("\x1b[1;32m"));

    // Warning with background
    let warning = style("Warning")
        .color(Color::BgYellow)
        .bold()
        .build();
    assert!(warning.contains("\x1b[1;43m"));
}

// Edge Cases
#[test]
fn test_empty_string() {
    let styled = style("").bold().color(Color::Red).build();
    assert!(styled.contains("\x1b["));
    assert!(styled.contains("\x1b[0m"));
}

#[test]
fn test_multiple_resets() {
    let styled = style("test")
        .bold()
        .reset()
        .color(Color::Red)
        .reset()
        .italic()
        .reset()
        .build();

    assert_eq!(styled, "test");
}

// Builder Pattern Tests
#[test]
fn test_builder_chain() {
    let styled = ColorString::new("test")
        .bold()
        .color(Color::Red)
        .italic()
        .underline()
        .blink()
        .build();

    assert!(styled.contains("1;3;4;5;31"));
}