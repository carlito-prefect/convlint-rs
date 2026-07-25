## [unreleased]

### 🚀 Features

- *(commit)* Added commit model and error handling; implemented cli
- *(commit-parser)* Added a parser to parse raw commit messages into `CommitMessage` structs
- *(config)* Added basic configuration struct (currently without fields)
- *(rules/lint)* Added rules and a linter to apply them
- *(git)* Added collectiong commits from various sources; added a git module to interact with a repo; added workflow
- *(async)* Marked all functions, that can be async as async

### 🚜 Refactor

- *(error)* Moved error to a seperate directory and split it into config errors and model errors

### 📚 Documentation

- *(commit)* Added doc comments
