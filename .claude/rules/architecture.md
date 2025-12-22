# Architecture Constraints

## Module Dependencies (Allowed Imports)

```
main.rs -> can import: lexer, parser, executor, commands, utils
executor -> can import: parser (for AST types), commands, utils
commands -> can import: executor (for CommandResult), utils
parser -> can import: lexer (for Token types)
lexer -> can import: (none - leaf module)
utils -> can import: (none - leaf module)
```

## Forbidden Patterns

- commands/ importing from lexer/ directly
- parser/ importing from executor/
- Any module importing from main.rs
- Circular dependencies

## Adding New Commands

1. Add match arm in `commands/mod.rs` `execute_builtin()`
2. Include BOTH Arabic and English name: `"عربي" | "english"`
3. Create `cmd_<name>()` function returning `CommandResult`
4. Add tests for both Arabic and English invocations

## Adding New Operators

1. Add token variant in `lexer/token.rs`
2. Update lexer in `lexer/mod.rs` to recognize it
3. Update parser grammar in `parser/mod.rs`
4. Handle in executor `execute_with_input()`
5. Add lexer, parser, AND executor tests

## AST Changes

If modifying `Command` enum in `parser/ast.rs`:
- Update ALL match statements in `executor/mod.rs`
- Update Display impl if adding new variant
- Consider backward compatibility
