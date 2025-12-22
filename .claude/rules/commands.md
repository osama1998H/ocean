---
globs: src/commands/**/*.rs
---

# Command Implementation Rules

## Command Function Signature

```rust
fn cmd_<name>(args: &[&str]) -> CommandResult
fn cmd_<name>(args: &[&str], input: Option<&str>) -> CommandResult  // if needs pipe input
```

## Return Values

- `CommandResult::Success(String)` - Command succeeded, output to display/pipe
- `CommandResult::Error(String)` - Command failed, show error message
- `CommandResult::Exit(i32)` - Shell should exit with code
- `CommandResult::None` - Success but no output (e.g., `cd`)

## Error Message Format

Always bilingual:
```rust
CommandResult::Error(format!(
    "خطأ: {} / Error: {}",
    arabic_message, english_message
))
```

## Adding New Command Checklist

- [ ] Add to `execute_builtin()` match with Arabic AND English names
- [ ] Implement `cmd_<name>()` function
- [ ] Handle `input: Option<&str>` if command can receive piped input
- [ ] Return appropriate `CommandResult` variant
- [ ] Add usage error with bilingual message
- [ ] Update README.md command table (Arabic and English sections)
- [ ] Add test for both Arabic and English invocation
