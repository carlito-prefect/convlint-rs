use std::fmt::Display;

use derive_more::Display;

/// Represents a commit message's content.
///
/// This is parsed by [`crate::commit::parser::CommitParser`].
/// Every commit message consists of a header, which is mandatory,
/// an optional body and list of 0 or more footers.
///
/// Expected commit message format:
/// ```txt
/// type(scope): description
///
/// some body text
/// on multiple lines
/// which is allowed in the body
///
/// footersyntax1: with some content
///
/// footersyntax2 #with some other content
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitMessage {
    /// The header of the commit message.
    ///
    /// This field is mandatory, because a commit cannot be empty.
    pub header: CommitHeader,

    /// The body of the commit message.
    ///
    /// This field is optional, but can be enforced by setting the
    /// `body_empty` setting to level: `error` in the configuration
    /// file.
    pub body: Option<CommitBody>,

    /// The footers of the commit message.
    ///
    /// This field can be totally empty and hold
    ///
    pub footers: Vec<CommitFooter>,
}

impl Display for CommitMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        out.push_str(&self.header.to_string());
        if self.has_body() {
            out.push_str("\n\n");
            out.push_str(&self.body.clone().map(|b| b.content).unwrap_or_default());
        }
        if self.has_footers() {
            out.push_str("\n\n");
            out.push_str(
                &self
                    .footers
                    .iter()
                    .map(|f| format!("{}: {}", f.token, f.value))
                    .collect::<Vec<_>>()
                    .join("\n\n"),
            );
        }
        write!(f, "{out}")
    }
}

impl CommitMessage {
    /// Returns `true` if the commit message has a body.
    pub const fn has_body(&self) -> bool {
        self.body.is_some()
    }

    /// Returns `true` if the commit message has any footers.
    pub const fn has_footers(&self) -> bool {
        !self.footers.is_empty()
    }

    /// Returns `true` if either the header or any of
    /// the footers indicate that this is a commit with breaking
    /// changes.
    pub fn is_breaking(&self) -> bool {
        self.header.breaking || self.footers.iter().any(|f| f.breaking)
    }
}

/// Represents a commit message's header.
///
/// A header must consist of a type such as `feat` or `fix`,
/// optionally a scope that inidicates what the commit's type
/// refers to and a description, that briefly summarizes the
/// changes of the commit.
///
/// Expects format of:
///
/// ```txt
/// type(\(scope\))?\s*:\s*description
/// ```
#[derive(Debug, Clone, Display, Default, PartialEq, Eq)]
#[display(
    "{commit_type}{}{}: {description}",
    scope
        .clone()
        .map(|s| format!("({})", s.scope_name))
        .unwrap_or_default(),
    if *breaking {
        "!"
    } else {
        ""
    }

)]
pub struct CommitHeader {
    /// The type of the commit.
    ///
    /// A type must be any type of conventional commiting such as `fix`.
    pub commit_type: String,

    /// The scope of the commit.
    ///
    /// With `feat(parser): ...` the scope is `parser` which indicates
    /// that there is a new parser feature in this commit.
    pub scope: Option<CommitScope>,

    /// The description of the commit.
    ///
    /// A description must be at least one character
    /// long, but the minimum can be set with `description-min-length`
    /// in the configuration file.
    pub description: String,

    /// Indicates if the header specifies breaking changes.
    ///
    /// The parser checks if there is a `!` after the scope which
    /// inidcates breaking changes.
    pub breaking: bool,
}

/// Represents the body of a commit message.
///
/// The body is continous string interrupted by newlines
/// until there are two consecutive newlines.
///
/// The body is expected to consist of one or more
/// continous lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitBody {
    /// The commit body's content.
    pub content: String,
}

/// Represents a commit message's footer.
///
/// The footer is expected to be of format:
///
/// ```txt
/// some text: some info on this footer
/// ```
/// or
/// ```txt
/// some text #some into on this footer
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitFooter {
    /// The part of the footer before the `:` or `#`.
    pub token: String,

    /// The part of the footer after the `:` or `#`.
    pub value: String,

    /// Indicates if the footer specifies breaking changes.
    ///
    /// The parser checks for sub-strings in the token to see
    /// if the footer specifies breaking changes.
    pub breaking: bool,
}

impl CommitFooter {
    /// Creates a new [`CommitFooter`].
    pub fn new(token: &str, value: &str, breaking: bool) -> Self {
        Self {
            token: token.into(),
            value: value.into(),
            breaking,
        }
    }
}

/// Represents a commit's scope.
///
/// A scope must be of format: `(<scope>)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitScope {
    /// The name of the scopes.
    pub scope_name: String,
}

impl CommitScope {
    /// Creates a new [`CommitScope`].
    #[must_use]
    pub const fn new(scope_name: String) -> Self {
        Self { scope_name }
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use crate::commit::model::{CommitBody, CommitFooter, CommitHeader, CommitMessage};

    #[fixture]
    fn commit_message() -> CommitMessage {
        CommitMessage {
            header: CommitHeader {
                commit_type: "feat".into(),
                scope: None,
                description: "some".into(),
                breaking: false,
            },
            body: None,
            footers: vec![],
        }
    }

    #[rstest]
    #[case(Some(CommitBody { content: String::new() }))]
    #[case(None)]
    fn test_has_body(mut commit_message: CommitMessage, #[case] body: Option<CommitBody>) {
        let has_body_res = body.is_some();
        commit_message.body = body;
        assert_eq!(commit_message.has_body(), has_body_res);
    }

    #[rstest]
    #[case(vec![])]
    #[case(
        vec![
            CommitFooter {
                breaking: false,
                token: "some".into(),
                value: "more".into()
            }
        ]
    )]
    #[case(
        vec![
            CommitFooter::new("some", "content", false),
            CommitFooter::new("other", "content", false),
        ]
    )]
    fn test_has_footers(mut commit_message: CommitMessage, #[case] footers: Vec<CommitFooter>) {
        commit_message.footers = footers;
        assert_eq!(
            commit_message.has_footers(),
            !commit_message.footers.is_empty()
        );
    }

    #[rstest]
    // header is not breaking, there are no footers -> false
    #[case(false, vec![])]
    // header is breaking, there are no footers -> true
    #[case(true, vec![])]
    // header is not breaking, one non-breaking footer exists -> false
    #[case(false, vec![
        CommitFooter::new("some", "content", false),
    ])]
    // header is not breaking, one breaking footer exists -> true
    #[case(false, vec![
        CommitFooter::new("some", "content", true),
    ])]
    // header is breaking, one non-breaking footer exists -> true
    #[case(true, vec![
        CommitFooter::new("some", "content", false),
    ])]
    // header is breaking, one-breaking footer exists -> true
    #[case(true, vec![
        CommitFooter::new("some", "content", true),
    ])]
    // header is not breaking, two non-breaking footers exist -> false
    #[case(false, vec![
        CommitFooter::new("some", "content", false),
        CommitFooter::new("other", "content", false),
    ])]
    // header is not breaking, one of two footers is breaking -> true
    #[case(false, vec![
        CommitFooter::new("some", "content", true),
        CommitFooter::new("other", "content", false),
    ])]
    // header is not breaking, both footers are breaking -> true
    #[case(false, vec![
        CommitFooter::new("some", "content", true),
        CommitFooter::new("other", "content", true),
    ])]
    // header is breaking, two non-breaking footers exist -> true
    #[case(true, vec![
        CommitFooter::new("some", "content", false),
        CommitFooter::new("other", "content", false),
    ])]
    // header is breaking, one footer is breaking -> true
    #[case(true, vec![
        CommitFooter::new("some", "content", true),
        CommitFooter::new("other", "content", false),
    ])]
    // header is breaking, both footers are breaking -> true
    #[case(true, vec![
        CommitFooter::new("some", "content", true),
        CommitFooter::new("other", "content", true),
    ])]
    fn test_is_breaking(
        mut commit_message: CommitMessage,
        #[case] header_breaking: bool,
        #[case] footers: Vec<CommitFooter>,
    ) {
        commit_message.header.breaking = header_breaking;
        commit_message.footers = footers;
        assert_eq!(
            commit_message.is_breaking(),
            commit_message.header.breaking || commit_message.footers.iter().any(|f| f.breaking)
        );
    }
}
