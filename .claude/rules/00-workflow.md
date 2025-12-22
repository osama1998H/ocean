# Operating Procedure (Agent MUST Follow)

## Before Writing ANY Code

### Step 1: EXPLORE (No Edits Allowed)
- [ ] Identify the owning module(s) for this change
- [ ] Find 2+ existing examples of the same pattern in codebase
- [ ] List the 5-10 most relevant files and WHY each matters
- [ ] State which architectural layer this touches

### Step 2: PLAN (No Edits Allowed)
- [ ] Provide step-by-step implementation plan
- [ ] List files that will be modified
- [ ] State architectural risks or side effects
- [ ] Define test strategy (which tests to add/run)

### Step 3: IMPLEMENT (Edits Allowed)
- [ ] Keep diff minimal - prefer modifying over creating
- [ ] Reuse existing abstractions (don't invent new patterns)
- [ ] Follow existing code style in that module
- [ ] Add tests for new functionality

### Step 4: VERIFY
- [ ] Run `cargo test` - ALL tests must pass
- [ ] Run `cargo build` - no warnings
- [ ] Summarize what changed and why

## Red Flags (Stop and Ask)
- Change touches >3 modules -> needs architecture review
- Creating new top-level module -> propose alternative first
- Modifying lexer/parser grammar -> explain precedence impact
- Removing any existing functionality -> confirm with user
