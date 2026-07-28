# Convlint

Convlint is yet another conventional commit linter that validates commit(s) and is applicable locally, in CI and pre-commit.

This project is inspired by KeisukeYamashita's [commitlint-rs](https://github.com/KeisukeYamashita/commitlint-rs) and conventional-changelog's [commitlint](https://github.com/conventional-changelog/commitlint).

---

## Features

- **Commit Validation** - validate commits based on configured rules
- **Rule Configuration** - configure how to treat violations of specific rules
- **CI Integration** - validate all commits of a PR
- **Pre-Commit Integration** - add hook into the pre-commit configuration and run it before each commit
- **Clear Error Handling** - don't panic on unexpected events
- **Clear Diagnostic Message** - output all violations to stdout

---

## Quick Start

### Installation

#### Cargo

This method requires Rust and Cargo to be installed.

```bash
# Clone the repository
git clone git clone [https://github.com/carlito-prefect/convlint-rs.git](https://github.com/carlito-prefect/convlint-rs.git)

# Build and install it from the repo
cargo install --path . --target [your wanted target (e.g. x86_64-unknown-linux-musl)] # use `--locked` to not resolve the latest package versions but rather use the lock file
```

#### Pre-compiled binary

Download a release binary from the [release section](https://github.com/carlito-prefect/convlint-rs/releases) of the repository.

```bash
# Move to a directory included in the $PATH
cp convlint ~/.local/bin/
```

**Trouble Shooting**: check if the directory is in your `$PATH` using `echo $PATH`

### Verify installation

```bash
convlint -V
```

This should output the version of the installed `convlint` binary (e.g. `convlint 0.1.0`).

### Initialize Convlint

Convlint needs the `Convlint.toml` configuration file. If you don't want to write it yourself, you can generate the default configuration.

```bash
convlint init
```

### Configuration Examples

```toml
[rule.type-exists]
level = "error"
allowed = [
    "feat",
    "fix",
    "chore",
    "doc"
]
```

### Examples

**Lint a simple commit**:

```bash
convlint lint "feat(parser): implemented some new parser features"
```

This could output something like:

```txt
[WARNING] expected the commit message to have a body from rule `body-required`

   feat(parser): implemented some new parser feature
```

**Lint a commit from a file**:

```bash
convlint lint --edit [file] # falls back to `./.git/COMMIT_EDITMSG`
```

The fallback is the latest commit message. If used in pre-commit it 
holds the commit message of the commit that is about to be created

**Lint from a commit range**:

```bash
convlint lint --from HEAD~3 --to HEAD
```

---

## Repository structure

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

## Testing

To test the project, clone the repository and execute

```bash
cargo nextest run
```

> Currently only unit tests are implemented; integration tests will be added in future releases
