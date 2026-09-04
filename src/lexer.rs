use crate::diagnostics::{Diagnostic, Span};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Ident(String),
    Number(i32),
    String(String),
    Color(String),
    LBrace,
    RBrace,
    Eof,
}
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

pub fn lex(src: &str) -> Result<Vec<Token>, Diagnostic> {
    let mut out = Vec::new();
    for (li, line) in src.lines().enumerate() {
        let line_no = li + 1;
        let bytes = line.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            let c = bytes[i] as char;
            if c.is_whitespace() {
                i += 1;
                continue;
            }
            if c == '/' && i + 1 < bytes.len() && bytes[i + 1] as char == '/' {
                break;
            }
            let col = i + 1;
            match c {
                '{' => {
                    out.push(Token {
                        kind: TokenKind::LBrace,
                        span: Span::new(line_no, col, 1),
                    });
                    i += 1;
                }
                '}' => {
                    out.push(Token {
                        kind: TokenKind::RBrace,
                        span: Span::new(line_no, col, 1),
                    });
                    i += 1;
                }
                '"' => {
                    i += 1;
                    let start = i;
                    while i < bytes.len() && bytes[i] as char != '"' {
                        i += 1;
                    }
                    if i >= bytes.len() {
                        return Err(Diagnostic::new(
                            "E001",
                            Span::new(line_no, col, 1),
                            "unterminated string",
                        ));
                    }
                    out.push(Token {
                        kind: TokenKind::String(line[start..i].to_string()),
                        span: Span::new(line_no, col, i - start + 2),
                    });
                    i += 1;
                }
                '#' => {
                    let start = i;
                    i += 1;
                    while i < bytes.len() && (bytes[i] as char).is_ascii_hexdigit() {
                        i += 1;
                    }
                    out.push(Token {
                        kind: TokenKind::Color(line[start..i].to_string()),
                        span: Span::new(line_no, col, i - start),
                    });
                }
                '-' | '0'..='9' => {
                    let start = i;
                    if c == '-' {
                        i += 1;
                    }
                    while i < bytes.len() && (bytes[i] as char).is_ascii_digit() {
                        i += 1;
                    }
                    let s = &line[start..i];
                    let n = s.parse::<i32>().map_err(|_| {
                        Diagnostic::new(
                            "E002",
                            Span::new(line_no, col, s.len()),
                            format!("invalid number `{s}`"),
                        )
                    })?;
                    out.push(Token {
                        kind: TokenKind::Number(n),
                        span: Span::new(line_no, col, s.len()),
                    });
                }
                _ if is_ident_start(c) => {
                    let start = i;
                    i += 1;
                    while i < bytes.len() && is_ident_continue(bytes[i] as char) {
                        i += 1;
                    }
                    out.push(Token {
                        kind: TokenKind::Ident(line[start..i].to_string()),
                        span: Span::new(line_no, col, i - start),
                    });
                }
                _ => {
                    return Err(Diagnostic::new(
                        "E003",
                        Span::new(line_no, col, 1),
                        format!("unexpected character `{c}`"),
                    ));
                }
            }
        }
    }
    out.push(Token {
        kind: TokenKind::Eof,
        span: Span::new(src.lines().count() + 1, 1, 0),
    });
    Ok(out)
}
fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}
fn is_ident_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '-'
}
