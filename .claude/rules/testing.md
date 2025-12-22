# Testing Requirements

## Test Organization

All tests are inline in source files using `#[cfg(test)]` modules.

## When to Add Tests

- [ ] Any new command -> test with Arabic AND English name
- [ ] Any new operator -> lexer test + parser test + executor test
- [ ] Any bug fix -> regression test proving fix
- [ ] Any new utility function -> unit test

## Test Patterns

### Lexer Test Pattern
```rust
#[test]
fn test_<feature>() {
    let mut lexer = Lexer::new("<input>");
    let tokens = lexer.tokenize();
    assert_eq!(tokens.len(), <expected>);
    assert!(matches!(tokens[0].kind, TokenKind::<Expected>));
}
```

### Parser Test Pattern
```rust
fn parse(input: &str) -> Command {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    parser.parse().unwrap()
}

#[test]
fn test_<feature>() {
    let cmd = parse("<input>");
    assert!(matches!(cmd, Command::<Expected> { .. }));
}
```

## Required Commands

```bash
cargo test                    # Must pass before any PR
cargo test -- --nocapture     # For debugging test output
cargo build                   # Must have no warnings
```
