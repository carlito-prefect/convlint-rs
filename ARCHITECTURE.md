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
- automatic configuration discovery
- optional serialized diagnostics output
- auto-format commits if possible

---

## Non-Goals

- rewrite Git history
- manage releases
- replace Git

---

## Source Code Model

```txt
convlint/
├── ARCHITECTURE.md
├── Cargo.lock
├── Cargo.toml
├── CHANGELOG.md
├── cliff.toml
├── README.md
├── rust-toolchain.toml
└── src
    // the command line interface
    ├── cli.rs
    // all structures to represent
    // commits and parse them
    ├── commit
    │   ├── model.rs
    │   ├── mod.rs
    │   ├── parser.rs
    │   └── source.rs
    ├── config.rs
    // all errors
    ├── error
    │   ├── config_error.rs
    │   ├── git_error.rs
    │   ├── model_error.rs
    │   ├── mod.rs
    │   └── source_error.rs
    // all git functionality
    ├── git.rs
    ├── lib.rs
    // the linter which applies
    // rules to commits
    ├── lint
    │   ├── diagnostic.rs
    │   ├── engine.rs
    │   ├── mod.rs
    │   └── severity.rs
    ├── main.rs
    // all rules available in convlint
    └── rules
        ├── body_line_length.rs
        ├── body_required.rs
        ├── breaking_change_consistency.rs
        ├── description_length.rs
        ├── description_required.rs
        ├── footer_line_length.rs
        ├── footer_required.rs
        ├── header_length.rs
        ├── mod.rs
        ├── scope_required.rs
        └── type_exists.r
```

---

## Dataflow

General workflow:

```txt
File/Stdin/String/Git-Range
        ↓
Fetch commit from source
        ↓
Parse commits to `CommitMessage`s
        ↓
Apply each rule to each commit message
        ↓
Collect all diagnostics
        ↓
Output diagnostics to stdout/file
```

> outputting diagnostics to file is not yet supported

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
            value: "Parser API changed.",
            breaking: true,
        }
    ],
}
```

---

## Commit Sources

Commit messages can come from various sources.

- file: `.git/COMMIT_EDITMSG`
- git history: `HEAD~10..HEAD`
- stdin (must be passed via pipe, because the otherwise the stdin will wait forever)
- direct string (as command line argument)

This allows the linter to be used in different contexts: hooks, CI, manual checks, integrations.

---

## Parser

The parser converts raw text into internal commit model.

Responsibilities:

- parse header:
  - extract type
  - extract scope
  - detect breaking changes
- parse body
- parse footers:
  - extract token:value
  - detect breaking changes

The parser does not validate the commit message.

---

## Rule Engine

Rules validate the parsed commit.

Example rules:

- type-exists (checks if the given type is allowed)
- scope-required (checks if a scope is required and exists)
- description-length (checks if the length of the description is valid)
- body-required (checks if a body is required and exists)

Interface:

```rust
trait Rule: Send + Sync {
    fn id() -> &'static str
    where
        Self: Sized;

    fn check(
        commit: &CommitMessage,
        config: &ConvlintTOML
    ) -> Option<Diagnostic>
    where
        Self: Sized;
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
    commit: CommitMessage { .. },
}
```

Diagnostics can be rendered as:

- terminal output
- JSON (not yet supported)

---

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
- Create `Convlint.toml`

---

### lint

Validate commit messages.

Sources:

- file (`--edit`)
- git range (`--from`, `--to`)
- stdin
- message (a pure string)

Restrictions:

- `--edit` cannot be combined with `--from`/`--to`
- `--from` can be used without `--to`, but not vice versa, since this could result in a lot of commits checked accidentally in large repos

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

- `ConfigError`: errors loading, storing and processing the configuration
- `SourceError`: errors when determining the commit source
- `ModelError`: errors that happen during transformation from source to commit model
- `GitError`: errors when processing a git repo
