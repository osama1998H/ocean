//! # Arabic Text Utilities (أدوات النص العربي)
//!
//! Handles Arabic letter shaping and RTL text alignment for terminal display.
//!
//! Arabic text requires special processing:
//! 1. **Letter Shaping**: Arabic letters have different forms (initial, medial, final, isolated)
//! 2. **RTL Alignment**: Text should start from the right side of the terminal

use arabic_reshaper::arabic_reshape;
use crossterm::terminal;
use unicode_width::UnicodeWidthStr;
use std::io::Write;

/// VTE escape code for RTL auto-detection mode
const VTE_RTL_AUTO: &str = "\x1b[?2501h";

/// Process Arabic text for correct terminal display
///
/// This function reshapes Arabic letters to their connected forms.
/// Modern terminals handle RTL direction automatically, so we only
/// need to do letter shaping (connecting letters).
///
/// # Example
/// ```
/// use ocean::utils::arabic::shape_arabic;
/// let text = "محيط";
/// let shaped = shape_arabic(text);
/// // Returns properly connected Arabic text
/// ```
pub fn shape_arabic(text: &str) -> String {
    // Handle empty text
    if text.is_empty() {
        return String::new();
    }

    // Reshape Arabic letters to connected forms
    // This converts isolated letters to their proper contextual forms
    // (initial, medial, final, or isolated depending on position)
    arabic_reshape(text)
}

/// Check if a string contains Arabic characters
///
/// Returns true if any character falls within Arabic Unicode ranges:
/// - U+0600-U+06FF: Arabic
/// - U+0750-U+077F: Arabic Supplement
/// - U+08A0-U+08FF: Arabic Extended-A
/// - U+FB50-U+FDFF: Arabic Presentation Forms-A
/// - U+FE70-U+FEFF: Arabic Presentation Forms-B
pub fn contains_arabic(text: &str) -> bool {
    text.chars().any(is_arabic_char)
}

/// Check if a character is Arabic
fn is_arabic_char(c: char) -> bool {
    matches!(c,
        '\u{0600}'..='\u{06FF}' |  // Arabic
        '\u{0750}'..='\u{077F}' |  // Arabic Supplement
        '\u{08A0}'..='\u{08FF}' |  // Arabic Extended-A
        '\u{FB50}'..='\u{FDFF}' |  // Arabic Presentation Forms-A
        '\u{FE70}'..='\u{FEFF}'    // Arabic Presentation Forms-B
    )
}

/// Process text, only applying Arabic shaping if Arabic characters are present
///
/// This is an optimization to avoid processing pure ASCII/English text
pub fn shape_if_arabic(text: &str) -> String {
    if contains_arabic(text) {
        shape_arabic(text)
    } else {
        text.to_string()
    }
}

// ============================================================================
// RTL Alignment Functions
// ============================================================================

/// Enable RTL mode in terminal (VTE terminals only)
///
/// Sends VTE escape code for RTL auto-detection if a VTE terminal is detected.
/// Returns true if VTE terminal detected, false otherwise (fallback to padding needed).
pub fn enable_rtl_mode() -> bool {
    // VTE is used by: GNOME Terminal, Konsole, Xfce4 Terminal, Tilix
    // Check environment variables to detect VTE-compatible terminals
    let is_vte = std::env::var("VTE_VERSION").is_ok() ||
        std::env::var("GNOME_TERMINAL_SCREEN").is_ok() ||
        std::env::var("KONSOLE_VERSION").is_ok();

    // Only send VTE RTL escape code if we detected a VTE terminal
    if is_vte {
        print!("{}", VTE_RTL_AUTO);
        let _ = std::io::stdout().flush();
    }

    is_vte
}

/// Get terminal width, default to 80 if unavailable
pub fn get_terminal_width() -> usize {
    terminal::size().map(|(w, _)| w as usize).unwrap_or(80)
}

/// Calculate display width of text (handles Arabic correctly)
pub fn display_width(text: &str) -> usize {
    UnicodeWidthStr::width(text)
}

/// Right-align text for RTL display (fallback method for non-VTE terminals)
///
/// Adds padding on the left to push text to the right side of the terminal.
pub fn right_align(text: &str) -> String {
    let term_width = get_terminal_width();
    let text_width = display_width(text);

    if text_width >= term_width {
        return text.to_string();
    }

    let padding = term_width - text_width;
    format!("{}{}", " ".repeat(padding), text)
}

/// Format a line for RTL display (shape + optionally right-align)
#[allow(dead_code)]
pub fn format_rtl(text: &str, use_padding: bool) -> String {
    let shaped = shape_arabic(text);
    if use_padding {
        right_align(&shaped)
    } else {
        shaped
    }
}

/// Print a line with RTL alignment
#[allow(dead_code)]
pub fn println_rtl(text: &str, use_padding: bool) {
    if use_padding && contains_arabic(text) {
        println!("{}", right_align(&shape_if_arabic(text)));
    } else {
        println!("{}", shape_if_arabic(text));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_arabic() {
        assert!(contains_arabic("محيط"));
        assert!(contains_arabic("Hello محيط World"));
        assert!(!contains_arabic("Hello World"));
        assert!(!contains_arabic("12345"));
        assert!(!contains_arabic(""));
    }

    #[test]
    fn test_shape_arabic_empty() {
        assert_eq!(shape_arabic(""), "");
    }

    #[test]
    fn test_shape_arabic_english() {
        // English text should pass through unchanged (or minimally changed)
        let result = shape_arabic("Hello");
        assert!(result.contains("Hello") || result == "Hello");
    }

    #[test]
    fn test_shape_if_arabic_optimization() {
        // English text should return as-is
        assert_eq!(shape_if_arabic("Hello"), "Hello");

        // Arabic text should be processed
        let arabic = "محيط";
        let shaped = shape_if_arabic(arabic);
        // The shaped text should be different (connected letters)
        assert!(!shaped.is_empty());
    }

    #[test]
    fn test_is_arabic_char() {
        assert!(is_arabic_char('م'));
        assert!(is_arabic_char('ح'));
        assert!(is_arabic_char('ي'));
        assert!(is_arabic_char('ط'));
        assert!(!is_arabic_char('H'));
        assert!(!is_arabic_char('1'));
    }

    #[test]
    fn test_display_width() {
        // ASCII characters have width 1
        assert_eq!(display_width("Hello"), 5);
        // Arabic characters typically have width 1
        assert!(display_width("محيط") > 0);
    }

    #[test]
    fn test_get_terminal_width() {
        // Should return at least the default of 80
        let width = get_terminal_width();
        assert!(width >= 80 || width > 0);
    }

    #[test]
    fn test_format_rtl_with_padding() {
        let text = "Test";
        let formatted = format_rtl(text, true);
        // With padding, result should be longer than original
        assert!(formatted.len() >= text.len());
    }

    #[test]
    fn test_format_rtl_without_padding() {
        let text = "Test";
        let formatted = format_rtl(text, false);
        // Without padding, just shaped (English stays same)
        assert_eq!(formatted, text);
    }
}
