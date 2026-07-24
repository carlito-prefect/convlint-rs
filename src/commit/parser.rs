use crate::{
    commit::model::{CommitBody, CommitFooter, CommitHeader, CommitMessage, CommitScope},
    error::{ConvlintError, ConvlintResult},
};

/// The parser parses raw strings into [`CommitMessage`]s.
#[derive(Debug, Clone)]
pub struct CommitParser {
    /// The commits as raw strings passed to the parser.
    pub(crate) raw_commits: Vec<&'static str>,
}

impl CommitParser {
    /// Creates a new [`CommitParser`].
    #[must_use]
    pub const fn new(raw_commits: Vec<&'static str>) -> Self {
        Self { raw_commits }
    }

    /// Applies the `parse_commit()` method to all raw commit
    /// messages that were passed to the parser.
    ///
    /// # Errors
    ///
    /// This function will return an error if any commit could
    /// not be parsed successfully.
    pub fn parse_commits(&self) -> ConvlintResult<Vec<CommitMessage>> {
        let mut parsed_commits = Vec::new();
        for raw_commit in &self.raw_commits {
            parsed_commits.push(Self::parse_commit(raw_commit)?);
        }
        Ok(parsed_commits)
    }

    /// Parse the whole commit message including header, body and footers.
    /// The expected form of the commit message can be found in [`CommitMessage`].
    ///
    /// # Errors
    ///
    /// This function will return an error if the raw commit is empty, the commit
    /// body, if any, could not be parsed or if any footer can be parsed successfully.
    pub(crate) fn parse_commit(raw_commit: &'static str) -> ConvlintResult<CommitMessage> {
        if raw_commit.is_empty() {
            return Err(ConvlintError::EmptyContent(String::from("commit")));
        }
        let mut parts_iter = raw_commit
            .split("\n\n")
            .map(str::trim)
            .collect::<Vec<_>>()
            .into_iter();

        let Some(raw_header) = parts_iter.next() else {
            return Err(ConvlintError::EmptyContent(String::from("commit header")));
        };
        let header = Self::parse_commit_header(raw_header)?;

        let Some(raw_body) = parts_iter.next() else {
            return Ok(CommitMessage {
                header,
                body: None,
                footers: vec![],
            });
        };
        let body = Self::parse_commit_body(raw_body)?;

        let mut footers = Vec::new();
        for raw_footer in parts_iter {
            footers.push(Self::parse_commit_footer(raw_footer)?);
        }

        Ok(CommitMessage {
            header,
            body: Some(body),
            footers,
        })
    }

    /// Parse the commit header. The expected format of the header can be found in [`CommitHeader`].
    ///
    /// # Errors
    ///
    /// This function will return an error if the raw header is malformed (e.g. does not contain a type,
    /// does not contain a description, is missing the scope if `()` is found or if the `:` is missing).
    // FIXME: refactor
    pub(crate) fn parse_commit_header(raw_header: &'static str) -> ConvlintResult<CommitHeader> {
        if raw_header.is_empty() {
            return Err(ConvlintError::EmptyContent(String::from("commit header")));
        }
        let mut header = CommitHeader::default();

        let Some(split_raw_header) = raw_header.split_once(':') else {
            return Err(ConvlintError::MissingCharacter(
                ':',
                String::from(
                    "conventional commits require a colon after the commit type (optionally with a scope inbetween)",
                ),
            ));
        };

        if split_raw_header.0.trim().is_empty() {
            return Err(ConvlintError::EmptyContent(String::from(
                "conventional type",
            )));
        }
        if split_raw_header.1.trim().is_empty() {
            return Err(ConvlintError::MissingDescription);
        }
        header.description = split_raw_header.1.trim().to_string();

        if let Some(split_type) = split_raw_header.0.split_once('(') {
            header.commit_type = split_type.0.trim().to_string();

            let Some(split_scope) = split_type.1.split_once(')') else {
                return Err(ConvlintError::MissingCharacter(
                    ')',
                    String::from("a closing parenthesis is expected after the scope name"),
                ));
            };

            if split_scope.0.is_empty() {
                return Err(ConvlintError::MissingScopeNameError);
            }
            header.scope = Some(CommitScope {
                scope_name: split_scope.0.trim().to_string(),
            });
            if split_scope.1 != "!" && !split_scope.1.is_empty() {
                return Err(ConvlintError::UnexpectedContent(split_scope.1.to_string()));
            } else if split_scope.1 == "!" {
                header.breaking = true;
            }
        } else {
            if split_raw_header.1.ends_with('!') {
                header.breaking = true;
            }
            header.commit_type = split_raw_header.0[..split_raw_header.0.len()]
                .trim()
                .to_string();
        }

        Ok(header)
    }

    /// Parse the commit message's body. The expected form of the body can
    /// be found in [`CommitBody`].
    ///
    /// # Errors
    ///
    /// This function will return an error if the raw body is empty or the
    /// lines in the body are not consecutive.
    pub(crate) fn parse_commit_body(raw_body: &'static str) -> ConvlintResult<CommitBody> {
        let body = raw_body.trim();
        if body.is_empty() {
            return Err(ConvlintError::EmptyContent(String::from("commit body")));
        }

        if body.split_once("\n\n").is_some() {
            return Err(ConvlintError::UnexpectedContent(String::from(
                "no newlines allowed in commit bodies",
            )));
        }

        Ok(CommitBody {
            content: body.to_string(),
        })
    }

    /// Parse the commit message's body. The expected form of the body can
    /// be found in [`CommitFooter`].
    ///
    /// # Errors
    ///
    /// This function will return an error if the footer is empty or there is no `:`/`#`.
    pub(crate) fn parse_commit_footer(raw_footer: &'static str) -> ConvlintResult<CommitFooter> {
        if raw_footer.is_empty() {
            return Err(ConvlintError::EmptyContent("commit footer".into()));
        }
        if let Some(split_colon) = raw_footer.split_once(':') {
            return Ok(CommitFooter {
                token: split_colon.0.trim().to_string(),
                value: split_colon.1.trim().to_string(),
                breaking: split_colon.0.to_lowercase().contains("breaking"),
            });
        }
        let Some(split_hashtag) = raw_footer.split_once('#') else {
            return Err(ConvlintError::MissingCharacter(
                ':',
                "a footer must contain a `:` or `#` somewhere in it".into(),
            ));
        };

        Ok(CommitFooter {
            token: split_hashtag.0.trim().to_string(),
            value: split_hashtag.1.trim().to_string(),
            breaking: split_hashtag.0.to_lowercase().contains("breaking"),
        })
    }
}

#[cfg(test)]
pub mod tests {
    use rstest::rstest;

    use crate::{
        commit::{
            model::{CommitBody, CommitFooter, CommitHeader, CommitMessage, CommitScope},
            parser::CommitParser,
        },
        error::ConvlintError,
    };

    #[rstest]
    #[case(
        "feat: some changes",
        CommitHeader {
            commit_type: "feat".into(),
            scope: None,
            description: "some changes".into(),
            breaking: false
        }
    )]
    #[case(
        "fix(parser): parser was fixed",
        CommitHeader {
            commit_type: "fix".into(),
            scope: Some(CommitScope {
                scope_name: "parser".into()
            }),
            description: "parser was fixed".into(),
            breaking: false
        }
    )]
    #[case(
        "feat(lexer)!: implemented the lexer",
        CommitHeader {
            commit_type: "feat".into(),
            scope: Some(CommitScope {
                scope_name: "lexer".into()
            }),
            description: "implemented the lexer".into(),
            breaking: true
        }
    )]
    fn parse_commit_header_success(
        #[case] raw_header: &'static str,
        #[case] expected: CommitHeader,
    ) {
        let header_res = CommitParser::parse_commit_header(raw_header);
        dbg!(&header_res);
        assert!(header_res.is_ok());
        assert_eq!(header_res.unwrap(), expected);
    }

    #[rstest]
    #[case(
        "fix some description",
        ConvlintError::MissingCharacter(
            ':',
            String::from(
                "conventional commits require a colon after the commit type (optionally with a scope inbetween)"
            )
        )
    )]
    #[case("fix(): some description", ConvlintError::MissingScopeNameError)]
    #[case(
        "fix(scope: some description",
        ConvlintError::MissingCharacter(
            ')',
            String::from("a closing parenthesis is expected after the scope name")
        )
    )]
    #[case("fix:", ConvlintError::MissingDescription)]
    #[case("", ConvlintError::EmptyContent(String::from("commit header")))]
    #[case(
        ": some description",
        ConvlintError::EmptyContent(String::from("conventional type"))
    )]
    #[case(
        "fix(scope)some!: some more",
        ConvlintError::UnexpectedContent(String::from("some!"))
    )]
    fn parse_commit_header_fail(#[case] raw_header: &'static str, #[case] expected: ConvlintError) {
        let header_res = CommitParser::parse_commit_header(raw_header);
        assert!(header_res.is_err());
        assert_eq!(header_res.unwrap_err(), expected);
    }

    #[rstest]
    fn parse_commit_body_success() {
        let raw_body = r"
            this is the body
            with multiple lines
            even a third line
            and empty lines are ignored

            ";
        let body_res = CommitParser::parse_commit_body(raw_body);
        assert!(body_res.is_ok());
        assert_eq!(
            body_res.unwrap(),
            CommitBody {
                content: raw_body.trim().to_string()
            }
        );
    }

    #[rstest]
    #[case("", ConvlintError::EmptyContent(String::from("commit body")))]
    #[case(
        r"some start

        after newline",
        ConvlintError::UnexpectedContent(String::from("no newlines allowed in commit bodies"))
    )]
    fn parse_commit_body_fail(#[case] raw_body: &'static str, #[case] expected: ConvlintError) {
        let body_res = CommitParser::parse_commit_body(raw_body);
        assert!(body_res.is_err());
        assert_eq!(body_res.unwrap_err(), expected);
    }

    #[rstest]
    #[case(
        "token: some info",
        CommitFooter {
            token: "token".into(),
            value: "some info".into(),
            breaking: false
        }
    )]
    #[case(
        "token #some info",
        CommitFooter {
            token: "token".into(),
            value: "some info".into(),
            breaking: false
        }
    )]
    #[case(
        "breaking changes: some info",
        CommitFooter {
            token: "breaking changes".into(),
            value: "some info".into(),
            breaking: true
        }
    )]
    #[case(
        "BREAKING CHANGES: some info",
        CommitFooter {
            token: "BREAKING CHANGES".into(),
            value: "some info".into(),
            breaking: true
        }
    )]
    fn parse_commit_footer_success(
        #[case] raw_footer: &'static str,
        #[case] expected: CommitFooter,
    ) {
        let footer_res = CommitParser::parse_commit_footer(raw_footer);
        assert!(footer_res.is_ok());
        assert_eq!(footer_res.unwrap(), expected);
    }

    #[rstest]
    #[case(
        "token some info",
        ConvlintError::MissingCharacter(
            ':',
            String::from("a footer must contain a `:` or `#` somewhere in it")
        )
    )]
    #[case("", ConvlintError::EmptyContent(String::from("commit footer")))]
    fn parse_commit_footer_fail(#[case] raw_footer: &'static str, #[case] expected: ConvlintError) {
        let footer_res = CommitParser::parse_commit_footer(raw_footer);
        assert!(footer_res.is_err());
        assert_eq!(footer_res.unwrap_err(), expected);
    }

    #[rstest]
    #[case(
        "feat(parser): only header",
        CommitMessage {
            header: CommitHeader {
                commit_type: "feat".into(),
                scope: Some(CommitScope {
                    scope_name: "parser".into()
                }),
                description: "only header".into(),
                breaking: false
            },
            body: None,
            footers: vec![]
        }
    )]
    #[case(
        r"feat(parser)!: header and body

        here is the body",
        CommitMessage {
            header: CommitHeader {
                commit_type: "feat".into(),
                scope: Some(CommitScope {
                    scope_name: "parser".into()
                }),
                description: "header and body".into(),
                breaking: true
            },
            body: Some(CommitBody {
                content: "here is the body".into()
            }),
            footers: vec![]
        }
    )]
    #[case(
        r"feat(parser)!: header and body and footers

        here is the body

        footer: number 1

        footer: number 2

        breaking: footer",
        CommitMessage {
            header: CommitHeader {
                commit_type: "feat".into(),
                scope: Some(CommitScope {
                    scope_name: "parser".into()
                }),
                description: "header and body and footers".into(),
                breaking: true
            },
            body: Some(CommitBody {
                content: "here is the body".into()
            }),
            footers: vec![
                CommitFooter {
                    breaking: false,
                    token: "footer".into(),
                    value: "number 1".into(),
                },
                CommitFooter {
                    breaking: false,
                    token: "footer".into(),
                    value: "number 2".into(),
                },
                CommitFooter {
                    breaking: true,
                    token: "breaking".into(),
                    value: "footer".into()
                }
            ]
        }
    )]
    fn parse_commit_success(#[case] raw_commit: &'static str, #[case] expected: CommitMessage) {
        let commit_res = CommitParser::parse_commit(raw_commit);
        assert!(commit_res.is_ok());
        assert_eq!(commit_res.unwrap(), expected);
    }

    #[rstest]
    #[case("", ConvlintError::EmptyContent("commit".into()))]
    fn parse_commit_fail(#[case] raw_commit: &'static str, #[case] expected: ConvlintError) {
        let commit_res = CommitParser::parse_commit(raw_commit);
        assert!(commit_res.is_err());
        assert_eq!(commit_res.unwrap_err(), expected);
    }
}
