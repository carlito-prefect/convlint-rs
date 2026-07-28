# Contributing Guidelines

Thank you for your interest in contributing to `convlint`. This document outlines the technical workflow and submission guidelines for this repository.

## 1. Code of Conduct
All contributors are expected to adhere strictly to our [Code of Conduct](CODE_OF_CONDUCT.md).

## 2. Reporting Issues
* Review existing GitHub issues prior to opening a new report to avoid duplicates.
* Provide a clear title, reproduction steps, expected behavior, and relevant system environment details when filing bug reports. Use the issue templates where possible.

## 3. Local Development Setup
Ensure you have an up-to-date Rust toolchain installed.

```bash
# Clone the repository
git clone [https://github.com/carlito-prefect/convlint-rs.git](https://github.com/carlito-prefect/convlint-rs.git)
cd convlint-rs

# Run tests
cargo test

# Verify formatting and static analysis
cargo fmt --all -- --check
cargo clippy -- -D warnings
```
