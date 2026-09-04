use serde::Serialize;
use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub len: usize,
}
impl Span {
    pub fn new(line: usize, column: usize, len: usize) -> Self {
        Self { line, column, len }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Diagnostic {
    pub code: String,
    pub line: usize,
    pub column: usize,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
}

impl Diagnostic {
    pub fn new(code: &str, span: Span, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            line: span.line,
            column: span.column,
            message: message.into(),
            suggestion: None,
        }
    }
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
    pub fn render_human(&self, filename: &str, source: &str) -> String {
        let src_line = source
            .lines()
            .nth(self.line.saturating_sub(1))
            .unwrap_or("");
        let caret_len = 1.max(self.message.split('`').nth(1).map(|s| s.len()).unwrap_or(1));
        let mut out = format!(
            "error[{}]: {}\n --> {}:{}:{}\n  |\n{} | {}\n  | {}{}\n",
            self.code,
            self.message,
            filename,
            self.line,
            self.column,
            self.line,
            src_line,
            " ".repeat(self.column.saturating_sub(1)),
            "^".repeat(caret_len)
        );
        if let Some(s) = &self.suggestion {
            out.push_str(&format!("  |\n  = help: did you mean `{s}`?\n"));
        }
        out
    }
}

impl Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}: {}", self.line, self.column, self.message)
    }
}

impl std::error::Error for Diagnostic {}

#[derive(Debug, Serialize)]
pub struct JsonDiagnostics<'a> {
    pub valid: bool,
    pub errors: &'a [Diagnostic],
}
