## [0.1.1] - 2026-07-28

### 🐛 Bug Fixes

- Added correct exit codes; added removal of comments before processing commit source
## [0.1.0] - 2026-07-28

### 🚀 Features

- *(commit)* Added commit model and error handling; implemented cli
- *(commit-parser)* Added a parser to parse raw commit messages into `CommitMessage` structs
- *(config)* Added basic configuration struct (currently without fields)
- *(rules/lint)* Added rules and a linter to apply them
- *(git)* Added collectiong commits from various sources; added a git module to interact with a repo; added workflow
- *(async)* Marked all functions, that can be async as async
- *(rules)* Added more rules
- *(tracing)* Added tracing

### 🐛 Bug Fixes

- *(parser)* Footers can now be after a body without a mandatory `\n\n`
- *(parser)* Adjusted expected commit format
- *(config)* Added a real default default configuration

### 💼 Other

- *(v0.1.0)* Prepare release

### 🚜 Refactor

- *(error)* Moved error to a seperate directory and split it into config errors and model errors
- *(rule)* Changed the return type of `check()` from `Vec<Diagnostic>` to `Option<Diagnostic>`
- *(parser)* Changed `parse_commit_header()` to be more clear

### 📚 Documentation

- *(commit)* Added doc comments
- *(repo)* Added multiple documentation markdowns
