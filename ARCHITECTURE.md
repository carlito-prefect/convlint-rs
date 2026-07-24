# Convlint Architecture

## Overview

`convlint` validates commit messages against configurable rules and can be
used in:

- local development
- Git hooks
- CI pipelines

---

## Goals

- validate conventional commit messages
- provide clear diagnostics
- support custom rules
- provide a Rust-native implementation
- automatic configuration discovery
- optional serialized output

---

## Non-Goals

- rewrite Git history
- manage releases
- replace Git

---

## Source Code Model

```txt
src/
├── cli.rs
├── error.rs
├── config.rs
├── commit/
│ └── mod.rs
├── git/
│ └── mod.rs
├── lint/
│ └── mod.rs
└── rules/
  └── mod.rs
```

---

## Dataflow

General workflow:

```txt
Commit source -> Commit message text -> Parser -> Commit message AST -> Rule engine -> Diagnostics -> Output formatter
```

---

## Commit AST

Commit messages are represented as structured data.

Example:

```txt
feat(parser): add Pratt parser

Implement precedence parsing.

BREAKING CHANGE: Parser API changed.
```

is represented as:

```rust
CommitMessage {
    header: CommitHeader {
        type: "feat",
        scope: Some("parser"),
        breaking: false,
        description: "add Pratt parser",
    },
    body: Some(...),
    footers: [
        CommitFooter {
            token: "BREAKING CHANGE",
            value: "Parser API changed."
        }
    ],
}
```

---

## Commit Sources

Commit messages can come from various sources.

- file: `.git/COMMIT_EDITMSG`
- git history: `HEAD~10..HEAD`
- stdin
- direct string

This allows the linter to be used in different contexts: hooks, CI, manual checks, integrations.

---

## Parser

The parser converts raw text into internal commit model.

Responsibilities:

- parse header
- extract type
- extract scope
- detect breaking changes
- parse body
- parse footers

The parser does not validate the commit message.

---

## Rule Engine

Rules validate the parsed commit.

Example rules:

- type-empty
- type-enum
- scope-empty
- subject-empty
- header-max-length

Interface:

```rust
trait Rule {
    fn name(&self) -> &'static str;

    fn check(
        &self,
        commit: &CommitMessage
    ) -> Vec<Diagnostic>;
}
```

## Diagnostics

Diagnostics are structured objects.

Example:

```rust
Diagnostic {
    rule: "header-max-length",
    severity: Severity::Error,
    message: "Header exceeds 72 characters",
    suggestion: None,
}
```

Diagnostics can be rendered as:

- terminal output
- JSON
- CI annotations

---

## Configuration

## Configuration

Supported format:

- `Convlint.toml`

Future:

- migrate `commitlint.config.js` configs
- migrate other formats

---

## CLI

### init

Creates default configuration.

Behavior:

- Search for existing config
- Never overwrite automatically
- Create `Convlint.toml` if missing

---

### lint

Validate commit messages.

Sources:

- file (`--edit`)
- git range (`--from`, `--to`)
- stdin

Restrictions:

- `--edit` cannot be combined with `--from`/`--to`

---

## Git Integration

Convlint integrates with Git in two ways.

### Commit hooks

Convlint is designed to be used with the `pre-commit` framework.

The pre-commit framework installs and manages Git hooks.
Convlint only receives the commit message file and validates it.

Example:

```yaml
repos:
  - repo: local
    hooks:
      - id: convlint
        name: convlint
        stages: [commit-msg]
        language: system
        entry: convlint lint --edit
```

### Commit history

Convlint can query existing commits for CI validation.

Examples:

- `--from HEAD~10 --to HEAD`
- `--from origin/main --to HEAD`

---

## Error Handling

Error types:

- `UserError`: invalid configuration or arguments
- `ParseError`: Malformed commit
- `ValidationError`: Commit does not match the rules
