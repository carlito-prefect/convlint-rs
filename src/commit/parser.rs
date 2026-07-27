use crate::{
    commit::model::{CommitBody, CommitFooter, CommitHeader, CommitMessage, CommitScope},
    error::model_error::{ModelError, ModelResult},
};

/// The parser parses raw strings into [`CommitMessage`]s.
#[derive(Debug, Clone)]
pub struct CommitParser {
    /// The commits as raw strings passed to the parser.
    pub(crate) raw_commits: Vec<String>,
}

impl CommitParser {
    /// Creates a new [`CommitParser`].
    #[must_use]
    pub const fn new(raw_commits: Vec<String>) -> Self {
        Self { raw_commits }
    }

    /// Applies the `parse_commit()` method to all raw commit
    /// messages that were passed to the parser.
    ///
    /// # Errors
    ///
    /// This function will return an error if any commit could
    /// not be parsed successfully.
    pub fn parse_commits(&self) -> ModelResult<Vec<CommitMessage>> {
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
    pub fn parse_commit(raw_commit: &str) -> ModelResult<CommitMessage> {
        if raw_commit.is_empty() {
            return Err(ModelError::EmptyContent(String::from("commit")));
        }
        let mut parts = raw_commit.splitn(3, "\n\n");
        let Some(raw_header) = parts.next() else {
            return Err(ModelError::EmptyContent(String::from("commit header")));
        };
        let header = Self::parse_commit_header(raw_header)?;
        let Some(next_raw) = parts.next() else {
            return Ok(CommitMessage {
                header,
                body: None,
                footers: vec![],
            });
        };
        let mut footers = Vec::new();
        let body = if next_raw.split_once(':').is_none() && next_raw.split_once('#').is_none() {
            Some(Self::parse_commit_body(next_raw)?)
        } else {
            footers.push(Self::parse_commit_footer(next_raw)?);
            None
        };
        let Some(raw_footer) = parts.next() else {
            return Ok(CommitMessage {
                header,
                body,
                footers: vec![],
            });
        };
        let mut buf = String::new();
        for line in raw_footer.lines() {
            if !buf.is_empty() && (line.split_once(':').is_some() || line.split_once('#').is_some())
            {
                footers.push(Self::parse_commit_footer(&buf)?);
                buf.clear();
            }
            buf += line;
            buf += "\n";
        }
        // parse the last footer stored in buf
        if !buf.is_empty() {
            footers.push(Self::parse_commit_footer(&buf)?);
        }
        Ok(CommitMessage {
            header,
            body,
            footers,
        })
    }

    /// Parse the commit header. The expected format of the header can be found in [`CommitHeader`].
    ///
    /// # Errors
    ///
    /// This function will return an error if the raw header is malformed (e.g. does not contain a type,
    /// does not contain a description, is missing the scope if `()` is found or if the `:` is missing).
    pub fn parse_commit_header(raw_header: &str) -> ModelResult<CommitHeader> {
        // check if the entire header is empty
        if raw_header.trim().is_empty() {
            return Err(ModelError::EmptyContent(String::from("commit header")));
        }
        // split at `:`, if this fails, it is no valid header format
        let Some((pre_colon, post_colon)) = raw_header
            .split_once(':')
            .map(|(l, r)| (l.trim().to_string(), r.trim().to_string()))
        else {
            return Err(ModelError::MissingCharacter(
                ':',
                String::from(
                    "conventional commits require a colon after the commit type (optionally with a scope inbetween)",
                ),
            ));
        };

        // description must not be empty
        if post_colon.is_empty() {
            return Err(ModelError::MissingDescription);
        }

        // try split at `(`, if successfull, continue parsing a scope name
        let (conv_type, scope_name, breaking) = if let Some((ty, scope)) = pre_colon
            .split_once('(')
            .map(|(l, r)| (l.trim().to_string(), r.trim().to_string()))
        {
            // conventional type must not be empty
            if ty.is_empty() {
                return Err(ModelError::EmptyContent(String::from("conventional type")));
            }

            // try split at `)`, if this fails, it is no valid scope format
            let Some((scope_name, breaking_str)) = scope
                .split_once(')')
                .map(|(l, r)| (l.trim().to_string(), r.trim().to_string()))
            else {
                return Err(ModelError::MissingCharacter(
                    ')',
                    String::from("a closing parenthesis is expected after the scope name"),
                ));
            };

            // scope name must not be empty
            if scope_name.is_empty() {
                return Err(ModelError::MissingScopeNameError);
            }
            // there is at most one character allowed after `)`
            if breaking_str.len() > 1 {
                return Err(ModelError::UnexpectedContent(breaking_str));
            }

            (ty, Some(scope_name), breaking_str == "!")
        } else
        /* header has no scope */
        {
            // try splitting at `!`, if this fails, it is a header without scope
            // and breaking indication
            let Some((ty, breaking_str)) = pre_colon
                .split_once('!')
                .map(|(l, r)| (l.trim().to_string(), r.trim().to_string()))
            else {
                // conventional type must not be empty
                if pre_colon.is_empty() {
                    return Err(ModelError::EmptyContent(String::from("conventional type")));
                }

                return Ok(CommitHeader {
                    commit_type: pre_colon,
                    scope: None,
                    description: post_colon,
                    breaking: false,
                });
            };
            // there is no content allowed after breaking indication
            if !breaking_str.is_empty() {
                return Err(ModelError::UnexpectedContent(breaking_str));
            }
            (ty, None, pre_colon.ends_with('!'))
        };

        Ok(CommitHeader {
            commit_type: conv_type,
            scope: scope_name.map(CommitScope::new),
            description: post_colon,
            breaking,
        })
    }

    /// Parse the commit message's body. The expected form of the body can
    /// be found in [`CommitBody`].
    ///
    /// # Errors
    ///
    /// This function will return an error if the raw body is empty or the
    /// lines in the body are not consecutive.
    pub fn parse_commit_body(raw_body: &str) -> ModelResult<CommitBody> {
        let body = raw_body.trim();
        if body.is_empty() {
            return Err(ModelError::EmptyContent(String::from("commit body")));
        }

        if body.split_once("\n\n").is_some() {
            return Err(ModelError::UnexpectedContent(String::from(
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
    pub fn parse_commit_footer(raw_footer: &str) -> ModelResult<CommitFooter> {
        if raw_footer.is_empty() {
            return Err(ModelError::EmptyContent("commit footer".into()));
        }
        if let Some(split_colon) = raw_footer.split_once(':') {
            return Ok(CommitFooter {
                token: split_colon.0.trim().to_string(),
                value: split_colon.1.trim().to_string(),
                breaking: split_colon.0.to_lowercase().contains("breaking"),
            });
        }
        let Some(split_hashtag) = raw_footer.split_once('#') else {
            return Err(ModelError::MissingCharacter(
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
        error::model_error::ModelError,
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
        assert!(header_res.is_ok());
        assert_eq!(header_res.unwrap(), expected);
    }

    #[rstest]
    #[case(
        "fix some description",
        ModelError::MissingCharacter(
            ':',
            String::from(
                "conventional commits require a colon after the commit type (optionally with a scope inbetween)"
            )
        )
    )]
    #[case("fix(): some description", ModelError::MissingScopeNameError)]
    #[case(
        "fix(scope: some description",
        ModelError::MissingCharacter(
            ')',
            String::from("a closing parenthesis is expected after the scope name")
        )
    )]
    #[case("fix:", ModelError::MissingDescription)]
    #[case("", ModelError::EmptyContent(String::from("commit header")))]
    #[case(
        ": some description",
        ModelError::EmptyContent(String::from("conventional type"))
    )]
    #[case(
        "fix(scope)some!: some more",
        ModelError::UnexpectedContent(String::from("some!"))
    )]
    fn parse_commit_header_fail(#[case] raw_header: &'static str, #[case] expected: ModelError) {
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
    #[case("", ModelError::EmptyContent(String::from("commit body")))]
    #[case(
        r"some start

        after newline",
        ModelError::UnexpectedContent(String::from("no newlines allowed in commit bodies"))
    )]
    fn parse_commit_body_fail(#[case] raw_body: &'static str, #[case] expected: ModelError) {
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
        "breaking changes: some info\nmulti line",
        CommitFooter {
            token: "breaking changes".into(),
            value: "some info\nmulti line".into(),
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
        ModelError::MissingCharacter(
            ':',
            String::from("a footer must contain a `:` or `#` somewhere in it")
        )
    )]
    #[case("", ModelError::EmptyContent(String::from("commit footer")))]
    fn parse_commit_footer_fail(#[case] raw_footer: &'static str, #[case] expected: ModelError) {
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
        "feat(parser)!: header and body and footers

        here is the body

        footer: number 1

        footer: number 2\nfirst multi line

        breaking: footer\nsome longer footer",
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
                    value: "number 2\nfirst multi line".into(),
                },
                CommitFooter {
                    breaking: true,
                    token: "breaking".into(),
                    value: "footer\nsome longer footer".into()
                }
            ]
        }
    )]
    #[case(
        r"feat(parser)!: header and body and footers

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
            body: None,
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
    #[case("", ModelError::EmptyContent("commit".into()))]
    fn parse_commit_fail(#[case] raw_commit: &'static str, #[case] expected: ModelError) {
        let commit_res = CommitParser::parse_commit(raw_commit);
        assert!(commit_res.is_err());
        assert_eq!(commit_res.unwrap_err(), expected);
    }
}
