# convlint

A fast, flexible, and developer-friendly Conventional Commits linter written in Rust.

This project is inspired by KeisukeYamashita's [commitlint-rs](https://github.com/KeisukeYamashita/commitlint-rs) and conventional-changelog's [commitlint](https://github.com/conventional-changelog/commitlint).

## Features

- Conventional Commits validation
- Configurable lint rules
- Clear diagnostics
- Git history validation
- Machine-readable output formats
- Rust-native implementation

## Installation

> Not available yet

## Usage

Create the default configuration file:

```bash
convlint init
```

Lint a commit message:

```bash
convlint lint --edit .git/COMMIT_EDITMSG
```

Lint a commit range:

```bash
# this lints all commits between HEAD~10 and HEAD (inclusive)
convlint lint --from HEAD~10 --to HEAD
```

```bash
# this lints all commits between the initial commit and HEAD (inclusive)
convlint lint --to HEAD~10
```
## Configuration

Configuration is stored in `Convlint.toml`.

> Example not yet final
