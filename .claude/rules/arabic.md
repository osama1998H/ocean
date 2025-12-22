---
globs: src/utils/arabic.rs, src/main.rs, src/executor/**/*.rs
---

# Arabic Text Handling Rules

## Text Processing Pipeline

```
Raw Text -> shape_arabic() -> RTL alignment (if needed) -> Terminal Output
```

## Functions to Use

- `shape_arabic(text)` - Connect Arabic letters (ALWAYS use for Arabic text)
- `shape_if_arabic(text)` - Only shapes if Arabic detected (use for mixed/unknown)
- `contains_arabic(text)` - Check if text has Arabic characters
- `right_align(text)` - Add left padding for RTL display

## When to Apply Shaping

- User-facing output (welcome, prompts, errors, help)
- Command output containing Arabic
- NOT for internal processing (parsing, file paths, etc.)

## RTL Alignment

- VTE terminals (Linux): Use native RTL via escape codes
- Non-VTE (macOS): Use `right_align()` padding fallback
- Check `use_rtl_padding` flag in executor/main

## Unicode Ranges (Arabic Detection)

```rust
'\u{0600}'..='\u{06FF}'  // Arabic
'\u{0750}'..='\u{077F}'  // Arabic Supplement
'\u{08A0}'..='\u{08FF}'  // Arabic Extended-A
'\u{FB50}'..='\u{FDFF}'  // Arabic Presentation Forms-A
'\u{FE70}'..='\u{FEFF}'  // Arabic Presentation Forms-B
```
