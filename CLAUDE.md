# Ocean (محيط) - Arabic Shell

> A modern shell with native Arabic commands. Part of the Tarqeem Project.

## Architecture Overview

**Three-Stage Pipeline (CRITICAL - never bypass):**
```
Input → Lexer → Parser → Executor → Output
         ↓        ↓         ↓
      Tokens    AST    CommandResult
```

### Directory Structure
```
src/
├── lexer/          # Tokenization (Arabic + English)
│   ├── mod.rs      # Lexer implementation
│   └── token.rs    # Token types with Span
├── parser/         # AST building
│   ├── mod.rs      # Parser (grammar: sequence→and_or→pipeline→simple)
│   └── ast.rs      # Command enum, Redirect types
├── executor/       # Command execution
│   ├── mod.rs      # Execute commands, handle redirects
│   └── pipeline.rs # Pipeline data flow
├── commands/       # 17+ built-in commands
│   └── mod.rs      # Command dispatcher (Arabic + English aliases)
├── utils/          # Utilities
│   ├── mod.rs      # Path expansion
│   └── arabic.rs   # RTL support, letter shaping
├── lib.rs          # Library exports
└── main.rs         # REPL entry point
```

## Critical Invariants (NEVER BREAK)

1. **Dual Command Names**: ALWAYS check both Arabic AND English variants
   - Example: `"اطبع" | "echo"` must be equivalent
2. **Pipeline Data Flow**: Output MUST pass to next command's input
3. **Token Position**: Every token needs valid Span (line/col) for errors
4. **Parser Precedence**: `;` < `&&`/`||` < `|` < simple_cmd
5. **Exit Codes**: `&&` runs right only if left=0, `||` only if left≠0
6. **RTL Processing**: ALWAYS call `shape_if_arabic()` before printing

## Command Dispatch Pattern

```rust
// In commands/mod.rs - FOLLOW THIS PATTERN
match name {
    "اطبع" | "echo" => Some(cmd_echo(args, input)),
    "انتقل" | "cd" => Some(cmd_cd(args)),
    // ... always include BOTH Arabic and English
    _ => None,  // Falls through to external command
}
```

## Standard Commands

```bash
cargo test              # Run 38 unit tests
cargo build --release   # Build optimized binary
cargo run               # Run shell
```

## Agent Operating Procedure (MANDATORY)

### 1. EXPLORE (read-only)
- Identify owning module for the change
- Find 2+ existing examples of the same pattern
- List 5-10 most relevant files

### 2. PLAN
- Steps with impacted components
- Architectural risks (interfaces, side effects)
- Test strategy

### 3. IMPLEMENT
- Minimal diff, reuse existing patterns
- Check BOTH Arabic and English variants
- Update relevant tests

### 4. VERIFY
- Run `cargo test`
- Run `cargo build`
- Summarize changes

## Imports

@README.md
@src/lib.rs
